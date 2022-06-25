#![no_main]
#![no_std]

use anne_keyberon as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    unsafe { anne_keyberon::setup() }

    ack(10, 10);
    anne_keyberon::exit()
}

fn ack(m: u32, n: u32) -> u32 {
    defmt::error!("ack(m={=u32}, n={=u32})", m, n);
    let mut big = [2; 512];
    if m == 0 {
        n + 1
    } else {
        big[100] += 1;
        if n == 0 {
            ack(m - 1, 1)
        } else {
            ack(m - 1, ack(m, n - 1))
        }
    }
}
