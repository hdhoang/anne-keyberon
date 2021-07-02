#![no_main]
#![no_std]

//! Schedule & reschedule 1 task. The timing is not as expected
//! based off <https://github.com/korken89/oxidize2020-rtic/blob/master/examples/5_schedule.rs>

use anne_keyberon as _; // global logger + panicking-behavior + memory layout
use rtic::cyccnt::U32Ext;

/// RM0038 6.2.3 at reset, MCU uses MSI clock at 2.097MHz
const MSI_FREQ: u32 = 2_097_000;

#[rtic::app(device = hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        anne_keyberon::idle_wfi()
    }

    #[init(schedule=[hello_world_task])]
    fn init(cx: init::Context) {
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
            .hello_world_task(cx.start + (MSI_FREQ * 100).cycles(), 100)
            .ok();
        // These schedules are lost
        cx.schedule
            .hello_world_task(cx.start + (MSI_FREQ * 20).cycles(), 20)
            .ok();
        cx.schedule
            .hello_world_task(cx.start + (MSI_FREQ * 3).cycles(), 3)
            .ok();

        defmt::info!("Hello from init! Please wait for the scheduled task");
    }

    extern "C" {
        //! Unmanaged interrupts to spawn priority levels
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
    }
    #[task(schedule=[hello_world_task])]
    fn hello_world_task(cx: hello_world_task::Context, turn: u8) {
        defmt::info!("Hello world from turn={=u8}!", turn);
        cx.schedule
            .hello_world_task(cx.scheduled + (MSI_FREQ * 30).cycles(), 30)
            .ok();
    }
};
