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

/// Prepare the Key MCU for debugging
/// - Relocate vector table to `0x0800_4000`, after Obins bootloader
/// cf memory.x from ah-/anne-key and `keyboards\anne_pro\ld\STM32L151.ld` in QMK
/// - Enable debugging interface, to get defmt RTT working
/// cf https://github.com/probe-rs/probe-rs/issues/350
/// # Safety
/// This function must be called first in ALL [`cortex_m_rt::entry`] `main` functions
pub unsafe fn setup() {
    use hal::stm32 as core;

    (*core::SCB::ptr()).vtor.write(0x4000);

    (*core::DBGMCU::ptr()).cr.modify(|_, w| {
        w.dbg_sleep().set_bit();
        w.dbg_standby().set_bit();
        w.dbg_stop().set_bit()
    });
    (*core::RCC::ptr())
        .ahbenr
        .modify(|_, w| w.dma1en().set_bit());
}

/// Same, but with RTIC cx.device
/// https://github.com/cavokz/rtic-examples/commit/0bb43ee49b38f6a083028ad8e3e46856af2a1836
pub fn init_profile(device: &hal::stm32::Peripherals) {
    device.RCC.ahbenr.modify(|_, w| w.dma1en().set_bit());
    // By default, power down the DBG module during wfi()
    device.DBGMCU.cr.modify(|_, w| w.dbg_standby().clear_bit());
    device.DBGMCU.cr.modify(|_, w| w.dbg_sleep().clear_bit());
    device.DBGMCU.cr.modify(|_, w| w.dbg_stop().clear_bit());

    // #[cfg(debug_assertions)]
    {
        // ~On development,~ keep the DBG module powered on during wfi()
        device.DBGMCU.cr.modify(|_, w| w.dbg_standby().set_bit());
        device.DBGMCU.cr.modify(|_, w| w.dbg_sleep().set_bit());
        device.DBGMCU.cr.modify(|_, w| w.dbg_stop().set_bit());
    }
}
