use crate::lcd::Result;

/// Number of lines of the display
#[derive(Clone, Copy, Debug)]
pub enum Lines {
    One,
    Two,
}

/// Font size used by the display controller
#[derive(Clone, Copy, Debug)]
pub enum Font {
    Size5x11,
    Size5x8,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

/// Shift direction and target
#[derive(Clone, Copy, Debug)]
pub enum Shift {
    /// Shift the entire display in the given [`Direction`]
    Display(Direction),
    /// Shift the cursor position in the given [`Direction`]
    Cursor(Direction),
}

macro commands(
    $( #[ $enum_attr:meta ] )*
    $vis:vis enum $enum_name:ident {
        $(
            $( #[doc = $doc:literal ] )*
            #[method( $method:ident )]
            $name:ident $params:tt
        ),* $(,)?
    }
) {
    $( #[ $enum_attr ] )*
    $vis enum $enum_name {
        $(
            $( #[doc = $doc ] )*
            $name $params,
        )*
    }

    impl Driver<'_> {
        $(
            display_command!{
                $( #[doc = $doc ] )*
                #[method( $method )]
                $name $params
            }
        )*
    }
}

macro display_command {
    (
        $( #[doc = $doc:literal ] )*
        #[method( $name:ident )]
        $cmd:ident {
            $(
                $( # $_:tt )*
                $param:ident : $type:ty
            ),* $(,)?
        }
    ) => {
        $( #[doc = $doc ] )*
        pub fn $name(&mut self, $( $param : $type ),* ) -> Result<()> {
            self.exec(Command:: $cmd { $( $param ),* })
        }
    },

    (
        $( #[doc = $doc:literal ] )*
        #[method( $name:ident )]
        $cmd:ident ( $( $type:ty )? )
    ) => {
        $( #[doc = $doc ] )*
        pub fn $name(&mut self, $( v : $type )? ) -> Result<()> {
            self.exec(Command:: $cmd ( $( v as $type )? ) )
        }
    }
}

use super::Driver;
commands! {
    #[derive(Clone, Copy, Debug)]
    pub enum Command {
        /// Clear the display
        ///
        /// This resets the shift and cursor position while
        /// filling the **DDRAM** with spaces (`0x20`)
        #[method(clear)]
        Clear(),
        /// Reset the display shift and cursor position
        ///
        /// Same as [`Clear`](Self::Clear) but without modifiying the **DDRAM** contents
        #[method(return_home)]
        ReturnHome(),
        /// Set what happens after a read or write operations is performed
        #[method(entry_mode_set)]
        EntryMode {
            /// Where to move the cursor
            ///
            /// More specifically whether to increment or decrement the **AC**
            /// which is the same for both **CGRAM** and **DDRAM**
            cursor: Direction,
            /// Whether or not to shift all the data
            ///
            /// The shift happens in the same direction of the `cursor` only when
            /// a **DDRAM** write operation is performed
            display: bool,
        },
        /// Enable or disable some components of the display
        #[method(onoff)]
        Onoff {
            /// Enables or disables the whole display
            ///
            /// Disabling the display will not erase the data in the DDRAM
            display: bool,
            /// Shows or hides the cursor under the current address
            cursor: bool,
            /// Enables or disables the blinking of the current character
            blink: bool,
        },
        /// Shift the display or the cursor
        #[method(shift)]
        Shift(Shift),
        /// Sets the functioning mode of the display
        #[method(function_set)]
        FunctionSet {
            /// Number of lines of the display
            lines: Lines,
            /// Font size used by the display
            font: Font,
        },
        /// Set the address in the **CGRAM** of the following read/write operations
        #[method(set_cgram_address)]
        CgRamAddress(u8),
        /// Set the address in the **DDRAM** of the following read/write operations
        #[method(set_ddram_address)]
        DdRamAddress(u8),
    }
}

impl Command {
    pub fn bits(self) -> u8 {
        use self::{Command::*, Direction::*, Font::*, Lines::*, Shift::*};
        match self {
            Clear() => 0b00000001,
            ReturnHome() => 0b00000010,
            EntryMode {
                cursor: Left,
                display,
            } => 0b00000100 | display as u8,
            EntryMode {
                cursor: Right,
                display,
            } => 0b00000110 | display as u8,
            Onoff {
                display,
                cursor,
                blink,
            } => 0b00001000 | (display as u8) << 2 | (cursor as u8) << 1 | blink as u8,
            Shift(Display(Right)) => 0b00011100,
            Shift(Display(Left)) => 0b00011000,
            Shift(Cursor(Right)) => 0b00010100,
            Shift(Cursor(Left)) => 0b00010000,
            FunctionSet {
                lines: One,
                font: Size5x8,
            } => 0b00110000,
            FunctionSet {
                lines: Two,
                font: Size5x8,
            } => 0b00111000,
            FunctionSet {
                lines: One,
                font: Size5x11,
            } => 0b00110100,
            FunctionSet {
                lines: Two,
                font: Size5x11,
            } => 0b00111100,
            CgRamAddress(address) => 0b01000000 | address,
            DdRamAddress(address) => 0b10000000 | address,
        }
    }
}

// impl Driver<'_> {
//     /// Clears the [`Driver`]
//     ///
//     /// Convenience method for calling [`exec`] with
//     /// [`Clear`](Command::Clear)
//     pub fn clear(&mut self) -> Result<()> {
//         self.exec(Command::Clear)
//     }

//     /// Resets the [`Driver`] shift and address counter
//     ///
//     /// Convenience method for calling [`exec`] with
//     /// [`ReturnHome`](Command::ReturnHome)
//     pub fn return_home(&mut self) -> Result<()> {
//         self.exec(Command::ReturnHome)
//     }

//     /// Sets the [`Driver`] entry mode
//     ///
//     /// Convenience method for calling [`exec`] with
//     /// [`EntryMode`](Command::EntryMode)
//     pub fn entry_mode_set(&mut self, cursor: Direction, display: bool) -> Result<()> {
//         self.exec(Command::EntryMode { cursor, display })
//     }

//     /// Sets what elements of the [`Driver`] to enable
//     ///
//     /// Convenience method for calling [`exec`] with
//     /// [`OnOff`](Command::OnOff)
//     pub fn onoff(&mut self, display: bool, cursor: bool, blink: bool) -> Result<()> {
//         self.exec(Command::Onoff {
//             display,
//             cursor,
//             blink,
//         })
//     }

//     /// Shifts either the [`Driver`] or the AC
//     ///
//     /// Convenience method for calling [`exec`] with
//     /// [`Shift`](Command::Shift)
//     pub fn cursor_or_display_shift(&mut self, shift: Shift) -> Result<()> {
//         self.exec(Command::Shift(shift))
//     }

//     /// Sets the [`Driver`] function mode
//     ///
//     /// Convenience method for calling [`exec`] with
//     /// [`FunctionSet`](Command::FunctionSet)
//     pub fn function_set(&mut self, lines: Lines, font: Font) -> Result<()> {
//         self.exec(Command::FunctionSet { lines, font })
//     }

//     /// Sets the **CGRAM** address for the next read/write operation
//     ///
//     /// Convenience method for calling [`exec`] with
//     /// [`CgramAddress`](Command::CgramAddress)
//     pub fn set_cgram_address(&mut self, address: u8) -> Result<()> {
//         self.exec(Command::CgRamAddress(address))
//     }

//     /// Sets the **DDRAM** address for the next read/write operation
//     ///
//     /// Convenience method for calling [`exec`] with
//     /// [`DdramAddress`](Command::DdramAddress)
//     pub fn set_ddram_address(&mut self, address: u8) -> Result<()> {
//         self.exec(Command::DdRamAddress(address))
//     }
// }
