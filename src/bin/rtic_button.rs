#![no_main]
#![no_std]

//! Report a button press, whoohoo
//! based off
//! anne-key matrix
//! and <https://github.com/stm32-rs/stm32l1xx-hal/blob/master/examples/button.rs>

use anne_keyberon as _; // global logger + panicking-behavior + memory layout

use embedded_hal::digital::v2::{InputPin, OutputPin};
use hal::gpio::{gpioa::*, GpioExt, Input, Output, PullDown, PullUp, PushPull};

#[rtic::app(device = hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        // Bottom-left Left-Control key on row PA0, column PA5
        COLUMN: PA5<Output<PushPull>>,
        ROW: PA0<Input<PullDown>>,
    }

    #[idle(resources = [COLUMN, ROW])]
    /// Put voltage on Column pin, measure output on row pin to detect button press
    fn idle(cx: idle::Context) -> ! {
        defmt::error!("start idle");
        let column = cx.resources.COLUMN;
        column.set_high().ok();
        defmt::error!("column is set high");
        let row = cx.resources.ROW;
        if row.is_low().unwrap() {
            defmt::error!("row is low")
        }
        loop {
            if row.is_high().unwrap() {
                defmt::error!("button pressed");
                anne_keyberon::exit()
            }
        }
    }

    #[init()]
    fn init(cx: init::Context) -> init::LateResources {
        // - Relocate vector table to `0x0800_4000`, after Obins bootloader
        // cf memory.x from ah-/anne-key and `keyboards\anne_pro\ld\STM32L151.ld` in QMK
        unsafe {
            cx.core.SCB.vtor.write(0x4000);
        }
        cx.device.RCC.ahbenr.modify(|_, w| w.dma1en().set_bit());
        anne_keyberon::init_debug(cx.device.DBGMCU);

        // let mut rcc = cx.device.RCC.freeze(hal::rcc::Config::hsi());
        let gpioa = cx.device.GPIOA.split();
        let column = gpioa.pa5.into_push_pull_output();
        let row = gpioa.pa0.into_pull_down_input();

        init::LateResources {
            COLUMN: column,
            ROW: row,
        }
    }
};
