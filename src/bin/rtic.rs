#![no_main]
#![no_std]

#[rtic::app(device = hal::stm32, peripherals = true)]
mod app {
    use anne_keyberon as _; // global logger + panicking-behavior + memory layout

    #[shared]
    struct SharedResources {}
    #[local]
    struct LocalResources {}

    #[init]
    fn init(cx: init::Context) -> (SharedResources, LocalResources, init::Monotonics) {
        // - Relocate vector table to `0x0800_4000`, after Obins bootloader
        // cf memory.x from ah-/anne-key and `keyboards\anne_pro\ld\STM32L151.ld` in QMK
        unsafe {
            cx.core.SCB.vtor.write(0x4000);
        }
        cx.device.RCC.ahbenr.modify(|_, w| w.dma1en().set_bit());
        anne_keyberon::init_debug(cx.device.DBGMCU);

        defmt::error!("Hello from init! Please interrupt");

        (SharedResources{}, LocalResources{}, init::Monotonics())
    }
}
