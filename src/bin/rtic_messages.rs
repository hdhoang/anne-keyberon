#![no_main]
#![no_std]

//! Message-passing with different queue depths.
//! based off <https://github.com/korken89/oxidize2020-rtic/blob/master/examples/7_message_passing.rs>

#[rtic::app(device = hal::stm32, peripherals = true, dispatchers=[EXTI0, EXTI1])]
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

        // Print the value via message passing!
        printer1::spawn(42).ok();

        // This will fail as printer1 has default capacity of 1!
        if printer1::spawn(43).is_err() {
            defmt::error!("Second spawn of printer1 failed!");
        }
        // Print to the printer that can take multiple!
        printer2::spawn(1).ok();
        printer2::spawn(2).ok();
        printer2::spawn(3).ok();
        printer2::spawn(4).ok();
        defmt::error!("Hello from init! Please wait for the tasks");
        (SharedResources {}, LocalResources {}, init::Monotonics())
    }

    #[task]
    fn printer1(_cx: printer1::Context, val: u32) {
        defmt::error!("printer1 says {=u32}", val)
    }
    #[task(capacity = 42)]
    fn printer2(_cx: printer2::Context, val: u32) {
        defmt::error!("printer2 says {=u32}", val)
    }
}
