#![no_main]
#![no_std]

//! Spawn priority-based software tasks. Not run yet

use anne_keyberon as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = hal::stm32, peripherals = true)]
const APP: () = {
    #[init(spawn=[prio_1_task, prio_2_task])]
    fn init(cx: init::Context) {
        // - Relocate vector table to `0x0800_4000`, after Obins bootloader
        // cf memory.x from ah-/anne-key and `keyboards\anne_pro\ld\STM32L151.ld` in QMK
        unsafe {
            cx.core.SCB.vtor.write(0x4000);
        }
        anne_keyberon::init_profile(&cx.device);

        cx.spawn.prio_1_task().ok();
        cx.spawn.prio_2_task().ok();
        defmt::info!("Hello from init! Please wait for the tasks");
    }

    extern "C" {
        //! Unmanaged interrupts to spawn priority levels
        fn EXTI0();
        fn EXTI1();
    }
    #[task]
    fn prio_1_task(_cx: prio_1_task::Context) {
        defmt::info!("task 1")
    }
    #[task(priority = 2)]
    fn prio_2_task(_cx: prio_2_task::Context) {
        defmt::info!("task 2")
    }
};
