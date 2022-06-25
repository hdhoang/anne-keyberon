#![no_main]
#![no_std]

//! Schedule 2 tasks, using a locked resource
//! based off <https://github.com/korken89/oxidize2020-rtic/blob/master/examples/6_resources_locks.rs>

use anne_keyberon as _; // global logger + panicking-behavior + memory layout
use rtic::cyccnt::U32Ext;

/// RM0038 6.2.3 at reset, MCU uses MSI clock at 2.097MHz
const MSI_FREQ: u32 = 2_097_000;

#[rtic::app(device = hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        // Resources go here!
        value: u8,
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        anne_keyberon::idle_wfi()
    }

    #[init(schedule=[priority_1_task, priority_2_task])]
    fn init(cx: init::Context) -> init::LateResources {
        // - Relocate vector table to `0x0800_4000`, after Obins bootloader
        // cf memory.x from ah-/anne-key and `keyboards\anne_pro\ld\STM32L151.ld` in QMK
        unsafe {
            cx.core.SCB.vtor.write(0x4000);
        }
        cx.device.RCC.ahbenr.modify(|_, w| w.dma1en().set_bit());
        anne_keyberon::init_debug(cx.device.DBGMCU);

        // When using schedule and a monotonic timer, remember to start the timer!

        // This is the `cortex_m::Peripherals` struct without the SysTick which RTIC has taken ownership of.
        let mut cp = cx.core;

        // Initialize (enable) the monotonic timer (CYCCNT)
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        // Schedule the task in the future,
        // but *100 here returns after around 6.5s
        cx.schedule
            .priority_1_task(cx.start + (MSI_FREQ * 10).cycles())
            .ok();
        cx.schedule
            .priority_2_task(cx.start + (MSI_FREQ * 15).cycles())
            .ok();
        defmt::error!("Hello from init! Please wait for the scheduled task");
        init::LateResources { value: 8 }
    }

    extern "C" {
        //! Unmanaged interrupts to spawn priority levels
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
    }
    #[task(priority = 2, resources=[value])]
    fn priority_2_task(cx: priority_2_task::Context) {
        defmt::error!("prio 2 begins");
        *(cx.resources.value) += 1;
        defmt::error!("prio 2 ends");
    }

    #[task(resources=[value])]
    fn priority_1_task(mut cx: priority_1_task::Context) {
        let value = cx.resources.value.lock(|v| *v);
        defmt::error!("Priority 1 begins, value={=u8}", value);
        for _ in 0..=1_000_000_00 {
            unsafe {
                core::ptr::read_volatile(&0);
            }
        }
        let value = cx.resources.value.lock(|v| *v);
        defmt::error!("Priority 1 ends, value={=u8}", value);
    }
};
