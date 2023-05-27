#![feature(decl_macro)]
#![feature(array_try_map)]

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;

mod lcd;
use lcd::cmd::{Direction::Right, Font::Size5x8, Lines::Two};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let pins = Peripherals::take().unwrap().pins;
    let mut display = lcd::Driver::setup(lcd::Pins {
        rs: pins.gpio4,
        rw: pins.gpio16,
        en: pins.gpio17,
        bus: lcd::bus::Pins {
            d0: pins.gpio18,
            d1: pins.gpio19,
            d2: pins.gpio14,
            d3: pins.gpio27,
            d4: pins.gpio26,
            d5: pins.gpio25,
            d6: pins.gpio33,
            d7: pins.gpio32,
        },
    })?;

    display.function_set(Two, Size5x8)?;
    display.clear()?;
    display.onoff(true, false, false)?;
    display.entry_mode_set(Right, false)?;

    display.write(b'H')?;
    display.write(b'e')?;
    display.write(b'l')?;
    display.write(b'l')?;
    display.write(b'o')?;
    display.write(b'!')?;

    loop {
        FreeRtos::delay_ms(100);
    }
}
