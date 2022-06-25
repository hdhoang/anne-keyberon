#![no_main]
#![no_std]

//! Spawn priority-based software tasks.
//! based off <https://github.com/korken89/oxidize2020-rtic/blob/master/examples/4_spawn_with_priority.rs>

#[rtic::app(device = hal::stm32, peripherals = true, dispatchers= [EXTI0, EXTI1])]
mod app {
    use anne_keyberon as _; // global logger + panicking-behavior + memory layout

    #[shared]
    struct SharedResources {}
    #[local]
    struct LocalResources {}
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        anne_keyberon::idle_loop()
    }

    #[init]
    fn init(cx: init::Context) -> (SharedResources, LocalResources, init::Monotonics) {
        // - Relocate vector table to `0x0800_4000`, after Obins bootloader
        // cf memory.x from ah-/anne-key and `keyboards\anne_pro\ld\STM32L151.ld` in QMK
        unsafe {
            cx.core.SCB.vtor.write(0x4000);
        }
        cx.device.RCC.ahbenr.modify(|_, w| w.dma1en().set_bit());
        anne_keyberon::init_debug(cx.device.DBGMCU);

        prio_1_task::spawn().ok();
        prio_2_task::spawn().ok();
        defmt::error!("Hello from init! Please wait for the tasks");
        (SharedResources {}, LocalResources {}, init::Monotonics())
    }

    #[task]
    fn prio_1_task(_cx: prio_1_task::Context) {
        defmt::error!("task 1")
    }
    #[task(priority = 2)]
    fn prio_2_task(_cx: prio_2_task::Context) {
        defmt::error!("task 2")
    }
}
