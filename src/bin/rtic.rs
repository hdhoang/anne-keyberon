#![no_main]
#![no_std]

use anne_keyberon as _; // global logger + panicking-behavior + memory layout

#[rtic::app(device = hal::stm32, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        // - Relocate vector table to `0x0800_4000`, after Obins bootloader
        // cf memory.x from ah-/anne-key and `keyboards\anne_pro\ld\STM32L151.ld` in QMK
        unsafe {
            cx.core.SCB.vtor.write(0x4000);
        }
        anne_keyberon::init_profile(&cx.device);

        defmt::info!("Hello from init! Please interrupt");
    }
};