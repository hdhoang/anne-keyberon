#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

use defmt_rtt as _; // global logger
use hal as _; // memory layout

use panic_probe as _;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", {
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n
});

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

/// SAFETY: must be called first in ALL [`cortex_m::entry`] `main` functions
/// This function prepares the Arm core for running Key firmware:
/// - Relocate vector table to `0x0800_4000`, after Obins bootloader
/// cf memory.x from ah-/anne-key and `keyboards\anne_pro\ld\STM32L151.ld` in QMK
/// - Enable debugging interface, to get defmt RTT working
/// cf https://github.com/probe-rs/probe-rs/issues/350
pub unsafe fn setup() {
    use hal::stm32 as core;

    &(*core::SCB::ptr()).vtor.write(0x4000);

    &(*core::DBGMCU::ptr()).cr.modify(|_, w| {
        w.dbg_sleep().set_bit();
        w.dbg_standby().set_bit();
        w.dbg_stop().set_bit()
    });
    &(*core::RCC::ptr())
        .ahbenr
        .modify(|_, w| w.dma1en().set_bit());
}
