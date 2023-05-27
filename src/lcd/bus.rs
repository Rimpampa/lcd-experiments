use std::mem::replace;

use esp_idf_hal::gpio::{AnyIOPin, IOPin, Input, Output, PinDriver};
use esp_idf_sys::EspError;

type Result<T> = core::result::Result<T, EspError>;

pub struct Pins<D0, D1, D2, D3, D4, D5, D6, D7> {
    pub d0: D0,
    pub d1: D1,
    pub d2: D2,
    pub d3: D3,
    pub d4: D4,
    pub d5: D5,
    pub d6: D6,
    pub d7: D7,
}

macro_rules! impl_pins {
    () => {
        Pins<
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
            impl IOPin,
        >
    };
}

pub enum Bus<'a> {
    Input([PinDriver<'a, AnyIOPin, Input>; 8]),
    Output([PinDriver<'a, AnyIOPin, Output>; 8]),
    Null,
}

impl<'a> Bus<'a> {
    const NULL: Self = Self::Null;

    pub fn new_output(pins: impl_pins!()) -> Result<Self> {
        Ok(Self::Output([
            PinDriver::output(pins.d0.downgrade())?,
            PinDriver::output(pins.d1.downgrade())?,
            PinDriver::output(pins.d2.downgrade())?,
            PinDriver::output(pins.d3.downgrade())?,
            PinDriver::output(pins.d4.downgrade())?,
            PinDriver::output(pins.d5.downgrade())?,
            PinDriver::output(pins.d6.downgrade())?,
            PinDriver::output(pins.d7.downgrade())?,
        ]))
    }

    pub fn into_input(self) -> Result<Self> {
        let Self::Output(pins) = self else { return Ok(self) };
        Ok(Self::Input(pins.try_map(PinDriver::into_input)?))
    }

    pub fn into_output(self) -> Result<Self> {
        let Self::Input(pins) = self else { return Ok(self) };
        Ok(Self::Output(pins.try_map(PinDriver::into_output)?))
    }

    pub fn make_input(&mut self) -> Result<()> {
        *self = replace(self, Self::NULL).into_input()?;
        Ok(())
    }

    pub fn make_output(&mut self) -> Result<()> {
        *self = replace(self, Self::NULL).into_output()?;
        Ok(())
    }

    pub fn write(&mut self, value: u8) -> Result<()> {
        self.make_output()?;
        let Self::Output(pins) = self else { unreachable!() };
        pins.iter_mut()
            .enumerate()
            .try_for_each(|(i, pin)| pin.set_level(<_>::into(value & 1 << i != 0)))
    }

    pub fn read(&mut self) -> Result<u8> {
        self.make_input()?;
        let Self::Input(pins) = self else { unreachable!() };
        Ok(pins.iter_mut().enumerate().fold(
            0,
            |or, (i, pin)| if pin.is_high() { or | 1 << i } else { or },
        ))
    }
}
