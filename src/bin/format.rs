#![no_main]
#![no_std]

use anne_keyberon as _; // global logger + panicking-behavior + memory layout
use defmt::Format; // <- derive attribute

#[derive(Format)]
struct S1<T> {
    x: u8,
    y: T,
}

#[derive(Format)]
struct S2 {
    z: u8,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    unsafe { anne_keyberon::setup() }

    let s = S1 {
        x: 42,
        y: S2 { z: 43 },
    };
    defmt::error!("s={:?}", s);
    let x = 42;
    defmt::error!("x={=u8}", x);

    anne_keyberon::exit()
}
