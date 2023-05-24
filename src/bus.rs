use core::mem::replace;
use hal::prelude::*;

macro_rules! bus {
    ($( $pin:ident : $gpio:ident ),* $(,)?) => {
        enum Bus {
            Output { $( $pin : $gpio <Output<PushPull>>, )* },
            Input { $( $pin : $gpio <Input<PullDown>>, )* },
            Null,
        }
    };
}

macro_rules! repeat {
    (for $name:ident in [$($t:ident),*] { $($expr:tt)* }) => {
        macro __repeat($$($$ $name:ident )*) {
            $($expr)*
        }
        __repeat!($($t)*)
    }
}

macro pin_num {
    (d0) => { 0 },
    (d1) => { 1 },
    (d2) => { 2 },
    (d3) => { 3 },
    (d4) => { 4 },
    (d5) => { 5 },
    (d6) => { 6 },
    (d7) => { 7 },
}

impl super::Bus {
    fn make_output(&mut self) {
        let Self::Input { .. } = self else { return };
        repeat!(for pin in [d0, d1, d2, d3, d4, d5, d6, d7] {
            let Self::Input { $($pin),* } = replace(self, Self::Null) else { unreachable!() };
            *self = Self::Output { $( $pin: $pin.into_push_pull_output(), )* }
        });
    }

    fn make_input(&mut self) {
        let Self::Output { .. } = self else { return };
        repeat!(for pin in [d0, d1, d2, d3, d4, d5, d6, d7] {
            let Self::Output { $($pin),* } = replace(self, Self::Null) else { unreachable!() };
            *self = Self::Input { $( $pin: $pin.into_pull_down_input(), )* }
        });
    }

    pub fn write(&mut self, value: u8) {
        self.make_output();
        repeat!(for pin in [d0, d1, d2, d3, d4, d5, d6, d7] {
            let Self::Output { $($pin),* } = self else { unreachable!() };
            $( $pin.set_state((value & (1 << pin_num!($pin)) != 0).into()).unwrap(); )*
        });
    }

    pub fn read(&mut self) -> u8 {
        self.make_input();
        repeat!(for pin in [d0, d1, d2, d3, d4, d5, d6, d7] {
            let Self::Input { $($pin),* } = self else { unreachable!() };
            return $( u8::from($pin.is_high().unwrap()) << pin_num!($pin) )|*
        });
    }
}
