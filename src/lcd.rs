//! This module implements [`Driver`] which is the LCD communication driver
//! for the **ST7066U** LCD display controller.
//! 
//! # Glossary
//!
//! Throughout the docs some terms will be used to refer to components of the
//! LCD controller, here is a list:
//! - **DDROM**: the Driver Data ROM is the read-only memory that contains
//!   all of the predefined charater bitmaps
//! - **DDRAM**: the Driver Data RAM is the part of memory of the display that
//!   stores what is shown as addresses to both the **DDROM** and **CGRAM**
//! - **CGRAM**: the Character Generator RAM is the memory that contains
//!   the user generated characters as bitmaps
//! - **AC**: the Address Counter is a special register of the display which
//!   stores the current address for reading and writing operation happening
//!   on the **DDRAM** and **CGRAM**
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
//! # fn main() -> Result<()> {
//! let lcd = Driver::setup(Pins { ..todo!() })?;
//!
//! // Inizialization: specify number of lines and font size
//! lcd.function_set(Lines::Two, Font::Size5x2)?;
//!
//! // Enable the display and the cursor (disable blinking)
//! lcd.on_off(true, true, false)?;
//!
//! // Clear the display contents
//! lcd.clear()?;
//!
//! // Write "Hello"
//! lcd.write(b'H')?;
//! lcd.write(b'e')?;
//! lcd.write(b'l')?;
//! lcd.write(b'l')?;
//! lcd.write(b'o')?;
//! # }
//! ```

use esp_idf_hal::{delay::Ets, gpio};
use gpio::{AnyOutputPin, IOPin, Output, OutputPin, PinDriver};

pub mod bus;
pub mod cmd;

type Result<T> = core::result::Result<T, esp_idf_sys::EspError>;

/// Struct used for describing the pins that are connected to the LCD display
///
/// A value of this type can be used with [`Driver::setup`] to configure
/// the LCD display peripheral
pub struct Pins<Rs, Rw, En, D0, D1, D2, D3, D4, D5, D6, D7> {
    /// Register select pin
    pub rs: Rs,
    /// Read/Write pin
    pub rw: Rw,
    /// Enable pin
    pub en: En,
    /// 8-bit wide data bus pins
    pub bus: bus::Pins<D0, D1, D2, D3, D4, D5, D6, D7>,
}

/// An **ST7066U** based LCD driver
pub struct Driver<'a> {
    /// Register select pin
    ///
    /// This pin selects between the _Data Register_ and the _Instruction Register_:
    /// - **HIGH** ⇒ _Data Register_
    /// - **LOW** ⇒ _Instruction Register_
    ///
    /// **Note** that the _Instruction Register_ cannot be read
    rs: PinDriver<'a, AnyOutputPin, Output>,
    /// Read/Write pin
    ///
    /// This pin selects between a read or write operation:
    /// - **HIGH** ⇒ Read
    /// - **LOW** ⇒ Write
    ///
    /// **Note** that the _Instruction Register_ cannot be read
    rw: PinDriver<'a, AnyOutputPin, Output>,
    /// Enable pin
    ///
    /// This pin starts the read or write operation
    en: PinDriver<'a, AnyOutputPin, Output>,
    /// 8-bit wide data bus
    bus: bus::Bus<'a>,
}

impl<'a> Driver<'a> {
    /// Sets up the [`Driver`] pins
    ///
    /// At the start all of the pins are set to output mode,
    /// and they are kept at their default level.
    /// Only the enable pin is set to low explicitly.
    pub fn setup(
        pins: Pins<
            impl OutputPin,
            impl OutputPin,
            impl OutputPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
        >,
    ) -> Result<Self> {
        let mut en = PinDriver::output(pins.en.downgrade_output())?;
        en.set_low()?;
        let s = Self {
            rs: PinDriver::output(pins.rs.downgrade_output())?,
            rw: PinDriver::output(pins.rw.downgrade_output())?,
            en,
            bus: bus::Bus::new_output(pins.bus)?,
        };
        Ok(s)
    }

    /// Executes the given [`Command`](cmd::Command)
    pub fn exec(&mut self, cmd: cmd::Command) -> Result<()> {
        self.rs.set_low()?;
        self.rw.set_low()?;
        self.bus.write(cmd.bits())?;
        self.en.set_high()?;
        self.en.set_low()?;

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
        Ets::delay_us(us);

        Ok(())
    }

    /// Writes a byte to the [`Driver`]
    ///
    /// Depeding on whether the last address setup command
    /// was [`CgramAddress`](Command::CgramAddress) or
    /// [`DdramAddress`](Command::DdramAddress) this function
    /// will write either to the **CGRAM** or to the **DDRAM**, respectively.
    pub fn write(&mut self, value: u8) -> Result<()> {
        self.rs.set_high()?;
        self.rw.set_low()?;
        self.bus.write(value)?;
        self.en.set_high()?;
        self.en.set_low()?;
        Ets::delay_us(37);
        Ok(())
    }

    /// Checks the busy flag to know if the [`Driver`] is executing a command
    pub fn is_busy(&mut self) -> Result<bool> {
        self.read_address_counter().map(|v| v & 0b10000000 != 0)
    }

    /// Reads the address counter
    ///
    /// The resulting address will refer either to the **CGRAM** or
    /// to the **DDRAM** depeding on whether the last address setup command
    /// was [`CgramAddress`](Command::CgramAddress) or
    /// [`DdramAddress`](Command::DdramAddress) respectively.
    ///
    /// The most significant bit of the returned value is the busy flag
    pub fn read_address_counter(&mut self) -> Result<u8> {
        self.rs.set_low()?;
        self.rw.set_high()?;
        self.bus.write(0)?;
        self.en.set_high()?;
        let value = self.bus.read();
        self.en.set_low()?;
        Ets::delay_us(1);
        value
    }

    /// Reads a byte from the [`Driver`]
    ///
    /// Depeding on whether the last address setup command
    /// was [`CgramAddress`](Command::CgramAddress) or
    /// [`DdramAddress`](Command::DdramAddress) this function
    /// will read either from the **CGRAM** or from the **DDRAM**, respectively.
    pub fn read(&mut self) -> Result<u8> {
        self.rs.set_high()?;
        self.rw.set_high()?;
        self.en.set_high()?;
        let value = self.bus.read();
        self.en.set_low()?;
        Ets::delay_us(37);
        value
    }
}
