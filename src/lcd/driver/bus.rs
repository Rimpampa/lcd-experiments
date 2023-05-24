use core::mem::replace;

use hal::gpio::{AnyPin, Input, Output, PullDown, PushPull, IO};

use hal::Error;

pub trait Pins {
    fn read_d0(&mut self) -> bool;
    fn read_d1(&mut self) -> bool;
    fn read_d2(&mut self) -> bool;
    fn read_d3(&mut self) -> bool;
    fn read_d4(&mut self) -> bool;
    fn read_d5(&mut self) -> bool;
    fn read_d6(&mut self) -> bool;
    fn read_d7(&mut self) -> bool;

    fn write_d0(&mut self, value: bool);
    fn write_d1(&mut self, value: bool);
    fn write_d2(&mut self, value: bool);
    fn write_d3(&mut self, value: bool);
    fn write_d4(&mut self, value: bool);
    fn write_d5(&mut self, value: bool);
    fn write_d6(&mut self, value: bool);
    fn write_d7(&mut self, value: bool);
}

pub enum Bus<Pins: self::Pins> {
    Input(Pins::ReadPins),
    Output(Pins::WritePins),
    Null,
}

impl<Pins: self::Pins> Bus<Pins> {
    const NULL: Self = Self::Null;

    pub fn new_output(pins: Pins) -> Self {
        Self::Output(pins.into_write())
    }

    pub fn into_input(self) -> Self {
        let Self::Output(pins) = self else { return self };
        Self::Input(pins.into_read())
    }

    pub fn into_output(self) -> Self {
        let Self::Input(pins) = self else { return self };
        Self::Output(pins.into_write())
    }

    pub fn make_input(&mut self) {
        *self = replace(self, Self::NULL).into_input()?;
    }

    pub fn make_output(&mut self) {
        *self = replace(self, Self::NULL).into_output()?;
    }

    pub fn write(&mut self, value: u8) {
        self.make_output()?;
        let Self::Output(pins) = self else { unreachable!() };
        pins.write_d0(value & 0b00000001 != 0);
        pins.write_d1(value & 0b00000010 != 0);
        pins.write_d2(value & 0b00000100 != 0);
        pins.write_d3(value & 0b00001000 != 0);
        pins.write_d4(value & 0b00010000 != 0);
        pins.write_d5(value & 0b00100000 != 0);
        pins.write_d6(value & 0b01000000 != 0);
        pins.write_d7(value & 0b10000000 != 0);
    }

    pub fn read(&mut self) -> u8 {
        self.make_input()?;
        let Self::Input(pins) = self else { unreachable!() };
        u8::from(pins.read_d0()) << 0
            | u8::from(pins.read_d1()) << 1
            | u8::from(pins.read_d2()) << 2
            | u8::from(pins.read_d3()) << 3
            | u8::from(pins.read_d4()) << 4
            | u8::from(pins.read_d5()) << 5
            | u8::from(pins.read_d6()) << 6
            | u8::from(pins.read_d7()) << 7
    }
}
