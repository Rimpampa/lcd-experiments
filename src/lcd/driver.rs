//! This module implements [`Driver`] which is the LCD communication driver
//! for the **ST7066U** LCD display controller.
//!
//! The commands are declared in the [`cmd`] submodule, while this module
//! implements the read and write operations to execute them.
//!
//! # Example
//!
//! The following example shows how to initialize the display and
//! write `"Hello"` on it:
//! ```
//! # use esp_test::lcd::{Driver, Pins, Result, cmd::Lines, cmd::Font};
//! # fn main() {
//! let lcd = Driver::setup(Pins { ..todo!() });
//!
//! // Inizialization: specify number of lines and font size
//! lcd.function_set(Lines::Two, Font::Size5x2);
//!
//! // Enable the display and the cursor (disable blinking)
//! lcd.on_off(true, true, false);
//!
//! // Clear the display contents
//! lcd.clear();
//!
//! // Write "Hello"
//! lcd.write(b'H');
//! lcd.write(b'e');
//! lcd.write(b'l');
//! lcd.write(b'l');
//! lcd.write(b'o');
//! # }
//! ```

use hal::{clock::Clocks, delay::Delay};

// pub mod bus;
pub mod cmd;

pub trait Pins {
    /// Register select pin
    ///
    /// This pin selects between the _Data Register_ and the _Instruction Register_:
    /// - **HIGH** ⇒ _Data Register_
    /// - **LOW** ⇒ _Instruction Register_
    ///
    /// **Note** that the _Instruction Register_ cannot be read
    fn set_rs(&mut self, value: bool);
    /// Read/Write pin
    ///
    /// This pin selects between a read or write operation:
    /// - **HIGH** ⇒ Read
    /// - **LOW** ⇒ Write
    ///
    /// **Note** that the _Instruction Register_ cannot be read
    fn set_rw(&mut self, value: bool);
    /// Enable pin
    ///
    /// This pin starts the read or write operation
    fn set_en(&mut self, value: bool);

    fn write(&mut self, value: u8);
    fn read(&mut self) -> u8;
}

/// An **ST7066U** based LCD driver
pub struct Driver<Pins: self::Pins> {
    pins: Pins,
    delay: Delay,
}

impl<Pins: self::Pins> Driver<Pins> {
    /// Sets up the [`Driver`] pins
    ///
    /// At the start all of the pins are set to output mode,
    /// and they are kept at their default level.
    /// Only the enable pin is set to low explicitly.
    pub fn setup(mut pins: Pins, clocks: &Clocks<'_>) -> Self {
        pins.set_en(false);
        let delay = Delay::new(clocks);
        Self { pins, delay }
    }

    /// Executes the given [`Command`](cmd::Command)
    pub fn exec(&mut self, cmd: cmd::Command) {
        self.pins.set_rs(false);
        self.pins.set_rw(false);
        self.pins.write(cmd.bits());
        self.pins.set_en(true);
        self.pins.set_en(false);

        use cmd::Command::*;
        let us = match cmd {
            Clear() => 1600,
            ReturnHome() => 1600,
            EntryMode { .. } => 40,
            Onoff { .. } => 40,
            Shift(_) => 40,
            FunctionSet { .. } => 40,
            CgRamAddress(_) => 40,
            DdRamAddress(_) => 40,
        };
        self.delay.delay(us);
    }

    /// Writes a byte to the [`Driver`]
    ///
    /// Depeding on whether the last address setup command
    /// was [`CgramAddress`](Command::CgramAddress) or
    /// [`DdramAddress`](Command::DdramAddress) this function
    /// will write either to the **CGRAM** or to the **DDRAM**, respectively.
    pub fn write(&mut self, value: u8) {
        self.pins.set_rs(true);
        self.pins.set_rw(false);
        self.pins.write(value);
        self.pins.set_en(true);
        self.pins.set_en(false);
        self.delay.delay(37);
    }

    /// Checks the busy flag to know if the [`Driver`] is executing a command
    pub fn is_busy(&mut self) -> bool {
        self.read_address_counter() & 0b10000000 != 0
    }

    /// Reads the address counter
    ///
    /// The resulting address will refer either to the **CGRAM** or
    /// to the **DDRAM** depeding on whether the last address setup command
    /// was [`CgramAddress`](Command::CgramAddress) or
    /// [`DdramAddress`](Command::DdramAddress) respectively.
    ///
    /// The most significant bit of the returned value is the busy flag
    pub fn read_address_counter(&mut self) -> u8 {
        self.pins.set_rs(false);
        self.pins.set_rw(true);
        self.pins.write(0);
        self.pins.set_en(true);
        let value = self.pins.read();
        self.pins.set_en(false);
        self.delay.delay(1);
        value
    }

    /// Reads a byte from the [`Driver`]
    ///
    /// Depeding on whether the last address setup command
    /// was [`CgramAddress`](Command::CgramAddress) or
    /// [`DdramAddress`](Command::DdramAddress) this function
    /// will read either from the **CGRAM** or from the **DDRAM**, respectively.
    pub fn read(&mut self) -> u8 {
        self.pins.set_rs(true);
        self.pins.set_rw(true);
        self.pins.set_en(true);
        let value = self.pins.read();
        self.pins.set_en(false);
        self.delay.delay(37);
        value
    }
}
