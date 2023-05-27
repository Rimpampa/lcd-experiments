#![feature(array_try_map)]
#![feature(slice_flatten)]
#![feature(decl_macro)]
#![feature(array_zip)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

use std::error::Error;
use std::iter::zip;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;

mod lcd;
use lcd::canvas::{Canvas, DdRam};
use lcd::cmd::{Direction::Right, Font::Size5x8, Lines::Two};

fn changes<T: PartialEq>(
    old: impl IntoIterator<Item = T>,
    new: impl IntoIterator<Item = T>,
) -> impl Iterator<Item = (usize, T)> {
    zip(old, new)
        .enumerate()
        .filter_map(|(i, (old, new))| (old != new).then_some((i, new)))
}

fn main() -> Result<(), Box<dyn Error>> {
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

    let mut canvas = Canvas::default();
    canvas.write("Hello World!", None);

    let mut old_ddram = DdRam::default();
    display.set_ddram_address(0)?;
    old_ddram
        .iter_mut()
        .try_for_each(|v| display.read().map(|b| *v = b))?;

    let mut old_cgram = [[0u8; 8]; 8];
    display.set_cgram_address(0)?;
    old_cgram
        .flatten_mut()
        .iter_mut()
        .try_for_each(|v| display.read().map(|b| *v = b))?;

    let mut timebuf = heapless::HistoryBuffer::<_, 100>::new();
    loop {
        let start = std::time::Instant::now();
        let (ddram, cgram) = canvas.render();

        changes(old_cgram.flatten(), cgram.flatten()).try_fold(cgram.len(), |at, (i, v)| {
            if at != i {
                display.set_cgram_address(i as u8)?;
            }
            display.write(*v).map(|_| i + 1)
        })?;
        old_cgram[..cgram.len()].copy_from_slice(&cgram);

        changes(old_ddram, ddram)
            .map(|(i, v)| (if i >= 8 { i - 8 + 0x40 } else { i }, v))
            .try_fold(ddram.len(), |at, (i, v)| {
                if at != i {
                    display.set_ddram_address(i as u8)?;
                }
                display.write(v).map(|_| i + 1)
            })?;
        old_ddram = ddram;

        let elapsed = start.elapsed().as_micros() as u32;
        timebuf.write(elapsed);
        let avg = timebuf.iter().sum::<u32>() / timebuf.len() as u32;
        println!("elapsed: {elapsed} ({avg})");

        canvas.shift_left(None);
        FreeRtos::delay_ms(200);
    }
}
