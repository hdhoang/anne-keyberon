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

/// Power-consuming idle handler for RTIC
/// cf <https://github.com/probe-rs/probe-rs/issues/350>
pub fn idle_loop() -> ! {
    defmt::info!("loop idling...");
    loop {
        continue
    }
}

/// WFI idle handler, closest to RTIC default
/// cf <https://github.com/probe-rs/probe-rs/issues/350>
pub fn idle_wfi() -> ! {
    defmt::info!("wfi idling...");
    loop {
        cortex_m::asm::wfi()
    }
}

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
/// cf <https://github.com/probe-rs/probe-rs/issues/350>
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
/// cf <https://github.com/cavokz/rtic-examples/commit/0bb43ee49b38f6a083028ad8e3e46856af2a1836>
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

/// Frome anne-key clock::init_clock
pub fn init_clock(p: &hal::stm32::Peripherals) {
    // power up USB
    p.USB.cntr.modify(|_, w| w.pdwn().clear_bit());

    // enable Flash 64-bit access, +prefetch, +latency
    p.FLASH.acr.modify(|_, w| w.acc64().set_bit());
    p.FLASH.acr.modify(|_, w| w.prften().set_bit());
    p.FLASH.acr.modify(|_, w| w.latency().set_bit());

    // enable HSE clock, wait
    p.RCC.cr.modify(|_, w| w.hseon().set_bit());
    while p.RCC.cr.read().hserdy().bit_is_clear() {}

    // enable APB1, APB2, why different bits?
    p.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
    p.RCC.apb1enr.modify(|_, w| w.pwren().set_bit());

    // RM0038 5.1.5
    // disable low-power mode, Voltage scaling range 1 "high perf"
    while p.PWR.csr.read().vosf().bit_is_set() {}
    p.PWR.cr.modify(|_, w| {
        w.lprun().clear_bit();
        unsafe { w.vos().bits(0b01) }
    });
    while p.PWR.csr.read().vosf().bit_is_set() {}

    #[rustfmt::skip]
    p.RCC.cfgr.modify(|_, w| unsafe {
        w.ppre1().bits(0b100)
         .ppre2().bits(0b100)
         .pllmul().bits(0b0010)
         .plldiv().bits(0b10)
         .pllsrc().set_bit()
    });

    p.RCC.cr.modify(|_, w| w.pllon().set_bit());
    while p.RCC.cr.read().pllrdy().bit_is_clear() {}

    p.RCC.cfgr.modify(|_, w| unsafe { w.sw().bits(0b11) });
    while p.RCC.cfgr.read().sws().bits() != 0b11 {}

    p.RCC.cr.modify(|_, w| w.msion().clear_bit());

    #[rustfmt::skip]
    p.RCC.ahbenr.modify(|_, w|
        w.gpiopaen().set_bit()
         .gpiopben().set_bit()
         .gpiopcen().set_bit());
}
