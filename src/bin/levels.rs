#![no_main]
#![no_std]

use anne_keyberon as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    unsafe { anne_keyberon::setup() }

    defmt::info!("info");
    defmt::trace!("trace");
    defmt::warn!("warn");
    defmt::debug!("debug");
    defmt::error!("error");

    anne_keyberon::exit()
}
