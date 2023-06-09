use core::fmt;
use core::iter::zip;

/// A 5 by 8 black-and-white image represented as a matrix of bits
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Bitmap([Bitline; 8]);

impl Bitmap {
    /// Converts a bitmap into a [`Char`]
    ///
    /// This method basically consists in mapping
    /// all the bytes of the array with [`Line::new`]
    pub fn new(bitmap: [u8; 8]) -> Self {
        Self(bitmap.map(Bitline::new))
    }

    /// Renders the given character
    ///
    /// Current implementation only renders characters that
    /// are present in the DDROM of the ST7066U (ROM code A).
    /// If the character is not present, a blank space will
    /// be generated.
    ///
    /// The map of the characters is [`CHAR2BITMAP`]
    pub fn render(ch: char) -> Self {
        MAP.get(&ch).copied().unwrap_or_default()
    }

    /// Convert the bitmap into and array of bytes
    pub fn raw(self) -> [u8; 8] {
        self.0.map(|l| l.0)
    }

    /// Calculate the distance from the two [`Bitmap`]s
    ///
    /// The distance is computed by counting the number
    /// of different bits
    pub fn distance(self, other: Self) -> u32 {
        zip(self.0, other.0).map(|(a, b)| a.distance(b)).sum()
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.into_iter().try_for_each(|v| write!(f, "{v:?}"))
    }
}

/// Line of bits of a [`Bitmap`]
///
/// It's stored in a byte for semplicity but only
/// the least significant 5 bits are used
///
/// This struct has more of a debug purpose:
/// it's needed to implement [`fmt::Driver`]
/// and [`fmt::Debug`] in a simpler way
///
/// Fortunately `repr(transparent)` assures there
/// is no additional cost
#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Bitline(u8);

impl Bitline {
    /// Construct a new [`Bitline`]
    ///
    /// This method masks the given byte in order to
    /// keep only the 5 least significant bits
    pub const fn new(raw: u8) -> Self {
        Self(raw & 0b11111)
    }

    /// Calculate the distance from the two [`Bitline`]s
    ///
    /// The distance is computed by counting the number
    /// of different bits
    pub const fn distance(self, other: Self) -> u32 {
        (self.0 ^ other.0).count_ones()
    }
}

impl fmt::Debug for Bitline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0b{:05b}", self.0)
    }
}

impl fmt::Display for Bitline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chars = [4, 3, 2, 1, 0].map(|i| b":#"[usize::from(self.0 >> i) & 1] as char);
        chars.into_iter().try_for_each(|c| write!(f, "{c}"))
    }
}

macro bitmap($($t:tt),* $(,)?) {
    Bitmap([$(Bitline::new($t)),*])
}

static MAP: phf::Map<char, Bitmap> = phf::phf_map! {
    ' ' => bitmap![0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    '!' => bitmap![0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00000, 0b00100, 0b00000],
    '"' => bitmap![0b01010, 0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    '#' => bitmap![0b01010, 0b01010, 0b11111, 0b01010, 0b11111, 0b01010, 0b01010, 0b00000],
    '$' => bitmap![0b00100, 0b01111, 0b10100, 0b01110, 0b00101, 0b11110, 0b00100, 0b00000],
    '%' => bitmap![0b11000, 0b11001, 0b00010, 0b00100, 0b01000, 0b10011, 0b00011, 0b00000],
    '&' => bitmap![0b01100, 0b10010, 0b10100, 0b01000, 0b10101, 0b10010, 0b01101, 0b00000],
    '\'' => bitmap![0b01100, 0b00100, 0b01000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    '(' => bitmap![0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010, 0b00000],
    ')' => bitmap![0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000, 0b00000],
    '*' => bitmap![0b00000, 0b00100, 0b10101, 0b01110, 0b10101, 0b00100, 0b00000, 0b00000],
    '+' => bitmap![0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000, 0b00000],
    ',' => bitmap![0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b00100, 0b01000, 0b00000],
    '-' => bitmap![0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000, 0b00000],
    '.' => bitmap![0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100, 0b00000],
    '/' => bitmap![0b00000, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b00000, 0b00000],
    '0' => bitmap![0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110, 0b00000],
    '1' => bitmap![0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110, 0b00000],
    '2' => bitmap![0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111, 0b00000],
    '3' => bitmap![0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110, 0b00000],
    '4' => bitmap![0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010, 0b00000],
    '5' => bitmap![0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110, 0b00000],
    '6' => bitmap![0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110, 0b00000],
    '7' => bitmap![0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00000],
    '8' => bitmap![0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110, 0b00000],
    '9' => bitmap![0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100, 0b00000],
    ':' => bitmap![0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b01100, 0b00000, 0b00000],
    ';' => bitmap![0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b00100, 0b01000, 0b00000],
    '<' => bitmap![0b00010, 0b00100, 0b01000, 0b10000, 0b01000, 0b00100, 0b00010, 0b00000],
    '=' => bitmap![0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
    '>' => bitmap![0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000, 0b00000],
    '?' => bitmap![0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100, 0b00000],
    '@' => bitmap![0b01110, 0b10001, 0b00001, 0b01101, 0b10101, 0b10101, 0b01110, 0b00000],
    'A' => bitmap![0b01110, 0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b00000],
    'B' => bitmap![0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110, 0b00000],
    'C' => bitmap![0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110, 0b00000],
    'D' => bitmap![0b11100, 0b10010, 0b10001, 0b10001, 0b10001, 0b10010, 0b11100, 0b00000],
    'E' => bitmap![0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111, 0b00000],
    'F' => bitmap![0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000, 0b00000],
    'G' => bitmap![0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111, 0b00000],
    'H' => bitmap![0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001, 0b00000],
    'I' => bitmap![0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110, 0b00000],
    'J' => bitmap![0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100, 0b00000],
    'K' => bitmap![0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001, 0b00000],
    'L' => bitmap![0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111, 0b00000],
    'M' => bitmap![0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001, 0b00000],
    'N' => bitmap![0b10001, 0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b00000],
    'O' => bitmap![0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110, 0b00000],
    'P' => bitmap![0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000, 0b00000],
    'Q' => bitmap![0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101, 0b00000],
    'R' => bitmap![0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001, 0b00000],
    'S' => bitmap![0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110, 0b00000],
    'T' => bitmap![0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000],
    'U' => bitmap![0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110, 0b00000],
    'V' => bitmap![0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100, 0b00000],
    'W' => bitmap![0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010, 0b00000],
    'X' => bitmap![0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001, 0b00000],
    'Y' => bitmap![0b10001, 0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00000],
    'Z' => bitmap![0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111, 0b00000],
    '[' => bitmap![0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110, 0b00000],
    '¥' => bitmap![0b10001, 0b01010, 0b11111, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000],
    ']' => bitmap![0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110, 0b00000],
    '^' => bitmap![0b00100, 0b01010, 0b10001, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    '_' => bitmap![0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111, 0b00000],
    '`' => bitmap![0b01000, 0b00100, 0b00010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    'a' => bitmap![0b00000, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111, 0b00000],
    'b' => bitmap![0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b11110, 0b00000],
    'c' => bitmap![0b00000, 0b00000, 0b01110, 0b10000, 0b10000, 0b10001, 0b01110, 0b00000],
    'd' => bitmap![0b00001, 0b00001, 0b01101, 0b10011, 0b10001, 0b10001, 0b01111, 0b00000],
    'e' => bitmap![0b00000, 0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110, 0b00000],
    'f' => bitmap![0b00110, 0b01001, 0b01000, 0b11100, 0b01000, 0b01000, 0b01000, 0b00000],
    'g' => bitmap![0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110, 0b00000],
    'h' => bitmap![0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001, 0b00000],
    'i' => bitmap![0b00100, 0b00000, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110, 0b00000],
    'j' => bitmap![0b00010, 0b00000, 0b00110, 0b00010, 0b00010, 0b10010, 0b01100, 0b00000],
    'k' => bitmap![0b10000, 0b10000, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b00000],
    'l' => bitmap![0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110, 0b00000],
    'm' => bitmap![0b00000, 0b00000, 0b11010, 0b10101, 0b10101, 0b10001, 0b10001, 0b00000],
    'n' => bitmap![0b00000, 0b00000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001, 0b00000],
    'o' => bitmap![0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110, 0b00000],
    'p' => bitmap![0b00000, 0b00000, 0b11110, 0b10001, 0b11110, 0b10000, 0b10000, 0b00000],
    'q' => bitmap![0b00000, 0b00000, 0b01101, 0b10011, 0b01111, 0b00001, 0b00001, 0b00000],
    'r' => bitmap![0b00000, 0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000, 0b00000],
    's' => bitmap![0b00000, 0b00000, 0b01110, 0b10000, 0b01110, 0b00001, 0b11110, 0b00000],
    't' => bitmap![0b01000, 0b01000, 0b11100, 0b01000, 0b01000, 0b01001, 0b00110, 0b00000],
    'u' => bitmap![0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101, 0b00000],
    'v' => bitmap![0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100, 0b00000],
    'w' => bitmap![0b00000, 0b00000, 0b10001, 0b10001, 0b10101, 0b10101, 0b01010, 0b00000],
    'x' => bitmap![0b00000, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b00000],
    'y' => bitmap![0b00000, 0b00000, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110, 0b00000],
    'z' => bitmap![0b00000, 0b00000, 0b11111, 0b00010, 0b00100, 0b01000, 0b11111, 0b00000],
    '{' => bitmap![0b00010, 0b00100, 0b00100, 0b01000, 0b00100, 0b00100, 0b00010, 0b00000],
    '|' => bitmap![0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000],
    '}' => bitmap![0b01000, 0b00100, 0b00100, 0b00010, 0b00100, 0b00100, 0b01000, 0b00000],
    '→' => bitmap![0b00000, 0b00100, 0b00010, 0b11111, 0b00010, 0b00100, 0b00000, 0b00000],
    '←' => bitmap![0b00000, 0b00100, 0b01000, 0b11111, 0b01000, 0b00100, 0b00000, 0b00000],
};
