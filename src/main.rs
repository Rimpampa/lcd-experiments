#![feature(array_try_map)]
#![feature(slice_flatten)]
#![feature(decl_macro)]
#![feature(array_zip)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(macro_metavar_expr)]
#![no_std]
#![no_main]

use esp_backtrace as _;

use hal::gpio::*;
use hal::prelude::*;
use hal::{clock::ClockControl, peripherals::Peripherals, timer::TimerGroup, Delay, Rtc, IO};

use core::iter::zip;

mod lcd;
use lcd::canvas::{Canvas, DdRam};
use lcd::cmd::{Direction::Right, Font::Size5x8, Lines::Two};

#[macro_use]
mod bus;

bus![
    d0: Gpio18,
    d1: Gpio19,
    d2: Gpio14,
    d3: Gpio27,
    d4: Gpio26,
    d5: Gpio25,
    d6: Gpio33,
    d7: Gpio32,
];

struct Pins {
    rs: Gpio4<Output<PushPull>>,
    rw: Gpio16<Output<PushPull>>,
    en: Gpio17<Output<PushPull>>,
    bus: Bus,
}

impl lcd::Pins for Pins {
    fn set_rs(&mut self, value: bool) {
        self.rs.set_state(value.into()).unwrap()
    }

    fn set_rw(&mut self, value: bool) {
        self.rw.set_state(value.into()).unwrap()
    }

    fn set_en(&mut self, value: bool) {
        self.en.set_state(value.into()).unwrap()
    }

    fn write(&mut self, value: u8) {
        self.bus.write(value)
    }

    fn read(&mut self) -> u8 {
        self.bus.read()
    }
}

mod timer;

fn changes<T: PartialEq>(
    old: impl IntoIterator<Item = T>,
    new: impl IntoIterator<Item = T>,
) -> impl Iterator<Item = (usize, T)> {
    zip(old, new)
        .enumerate()
        .filter_map(|(i, (old, new))| (old != new).then_some((i, new)))
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    timer::init(timer_group0.timer0);

    let pins = IO::new(peripherals.GPIO, peripherals.IO_MUX).pins;
    let mut display = lcd::Driver::setup(
        Pins {
            rs: pins.gpio4.into(),
            rw: pins.gpio16.into(),
            en: pins.gpio17.into(),
            bus: Bus::Output {
                d0: pins.gpio18.into(),
                d1: pins.gpio19.into(),
                d2: pins.gpio14.into(),
                d3: pins.gpio27.into(),
                d4: pins.gpio26.into(),
                d5: pins.gpio25.into(),
                d6: pins.gpio33.into(),
                d7: pins.gpio32.into(),
            },
        },
        &clocks,
    );

    display.function_set(Two, Size5x8);
    display.clear();
    display.onoff(true, false, false);
    display.entry_mode_set(Right, false);

    let mut canvas = Canvas::default();
    canvas.write("Hello World!", None);
    canvas.approx = true;

    let mut old_ddram = DdRam::default();
    display.set_ddram_address(0);
    old_ddram.iter_mut().for_each(|v| *v = display.read());

    let mut old_cgram = [[0u8; 8]; 8];
    display.set_cgram_address(0);
    old_cgram
        .flatten_mut()
        .iter_mut()
        .for_each(|v| *v = display.read());

    const REPRINT_HEADERS_CYCLES: usize = 20;
    let mut cycle = 0usize;

    let mut timebuf = heapless::HistoryBuffer::<_, 100>::new();
    let mut report = move |total, render, cgram, ddram| {
        if cycle == 0 {
            esp_println::print!(concat!(
                "+------------+------------+------------+------------+------------+\n",
                "|  Average   |   Total    | Rendering  |   CGRAM    |   DDRAM    |\n",
                "+------------+------------+------------+------------+------------+\n",
            ));
        }
        cycle += 1;
        if cycle == REPRINT_HEADERS_CYCLES {
            cycle = 0
        }

        timebuf.write(total);
        let average = timebuf.iter().sum::<u32>() / timebuf.len() as u32;

        let average = average as f32 / 1000f32;
        let total = total as f32 / 1000f32;
        let render = render as f32 / 1000f32;
        let cgram = cgram as f32 / 1000f32;
        let ddram = ddram as f32 / 1000f32;

        esp_println::println!(
            "| {average:>8.3}ms | {total:>8.3}ms | {render:>8.3}ms | {cgram:>8.3}ms | {ddram:>8.3}ms |"
        );
    };

    loop {
        timer::reset();

        let (ddram, cgram) = canvas.render();

        let elapsed_render = timer::elapsed_us();

        changes(old_cgram.flatten(), cgram.flatten()).fold(cgram.len(), |at, (i, v)| {
            if at != i {
                display.set_cgram_address(i as u8);
            }
            display.write(*v);
            i + 1
        });
        old_cgram[..cgram.len()].copy_from_slice(&cgram);

        let elapsed_cgram = timer::elapsed_us();

        changes(old_ddram, ddram)
            .map(|(i, v)| (if i >= 8 { i - 8 + 0x40 } else { i }, v))
            .fold(ddram.len(), |at, (i, v)| {
                if at != i {
                    display.set_ddram_address(i as u8);
                }
                display.write(v);
                i + 1
            });
        old_ddram = ddram;

        let elapsed_ddram = timer::elapsed_us();

        report(
            timer::elapsed_us(),
            elapsed_render,
            elapsed_cgram - elapsed_render,
            elapsed_ddram - elapsed_cgram,
        );

        canvas.shift_left(None);
        Delay::new(&clocks).delay(200_000);
    }
}
