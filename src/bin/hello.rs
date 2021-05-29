#![no_main]
#![no_std]

use anne_keyberon as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    unsafe { anne_keyberon::setup() }

    defmt::info!("Hello, world!");

    anne_keyberon::exit()
}
