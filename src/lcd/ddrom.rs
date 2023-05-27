use super::Bitmap;

/// Search for a [`Bitmap`] inside the **DDROM**
///
/// # Return
///
/// If a [`Bitmap`] that matches the given one, [`Some`]
/// is returned containing the address in the **DDROM**
/// of that [`Bitmap`].
/// Otherwise [`None`] is returned.
pub fn search(char: Bitmap) -> Option<u8> {
    MAP.get(&char.raw()).copied()
}

/// Returns an [`Iterator`] over all of the [`Bitmap`]s present
/// in the **DDROM** paired with their respective addresses
pub fn all() -> impl Iterator<Item = (Bitmap, u8)> {
    MAP.entries().map(|(&a, &b)| (Bitmap::new(a), b))
}

/// Map every bitmap present in the DDROM to its respective address
static MAP: phf::Map<[u8; 8], u8> = phf::phf_map! {
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000] => 0x83,
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111, 0b00000] => 0x5f,
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100, 0b00000] => 0x2e,
    [0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b00100, 0b01000, 0b00000] => 0x2c,
    [0b00000, 0b00000, 0b00000, 0b00000, 0b10000, 0b01000, 0b00100, 0b00000] => 0xa4,
    [0b00000, 0b00000, 0b00000, 0b00000, 0b11100, 0b10100, 0b11100, 0b00000] => 0xa1,
    [0b00000, 0b00000, 0b00000, 0b00100, 0b00100, 0b00100, 0b11100, 0b00000] => 0xa3,
    [0b00000, 0b00000, 0b00000, 0b01011, 0b10101, 0b11010, 0b00000, 0b00000] => 0xf3,
    [0b00000, 0b00000, 0b00000, 0b01100, 0b01100, 0b00000, 0b00000, 0b00000] => 0xa5,
    [0b00000, 0b00000, 0b00000, 0b01110, 0b00010, 0b00010, 0b11111, 0b00000] => 0xad,
    [0b00000, 0b00000, 0b00000, 0b10101, 0b10101, 0b00001, 0b00110, 0b00000] => 0xaf,
    [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000, 0b00000] => 0x2d,
    [0b00000, 0b00000, 0b00000, 0b11111, 0b00100, 0b00100, 0b11111, 0b00000] => 0xaa,
    [0b00000, 0b00000, 0b00010, 0b00100, 0b01100, 0b10100, 0b00100, 0b00000] => 0xa8,
    [0b00000, 0b00000, 0b00010, 0b11111, 0b00110, 0b01010, 0b10010, 0b00000] => 0xab,
    [0b00000, 0b00000, 0b00100, 0b00000, 0b11111, 0b00000, 0b00100, 0b00000] => 0xfd,
    [0b00000, 0b00000, 0b00100, 0b11111, 0b10001, 0b00001, 0b00110, 0b00000] => 0xa9,
    [0b00000, 0b00000, 0b00110, 0b01001, 0b10001, 0b10001, 0b11110, 0b10000] => 0xe6,
    [0b00000, 0b00000, 0b00111, 0b00100, 0b00100, 0b10100, 0b01000, 0b00000] => 0xe8,
    [0b00000, 0b00000, 0b01000, 0b11111, 0b01001, 0b01010, 0b01000, 0b00000] => 0xac,
    [0b00000, 0b00000, 0b01001, 0b10101, 0b10010, 0b10010, 0b01101, 0b00000] => 0xe0,
    [0b00000, 0b00000, 0b01101, 0b10011, 0b01111, 0b00001, 0b00001, 0b00000] => 0x71,
    [0b00000, 0b00000, 0b01101, 0b10011, 0b10001, 0b10001, 0b01111, 0b00001] => 0xf1,
    [0b00000, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111, 0b00000] => 0x61,
    [0b00000, 0b00000, 0b01110, 0b10000, 0b01100, 0b10001, 0b01110, 0b00000] => 0xe3,
    [0b00000, 0b00000, 0b01110, 0b10000, 0b01110, 0b00001, 0b11110, 0b00000] => 0x73,
    [0b00000, 0b00000, 0b01110, 0b10000, 0b10000, 0b10001, 0b01110, 0b00000] => 0x63,
    [0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b01010, 0b11011, 0b00000] => 0xf4,
    [0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110, 0b00000] => 0x6f,
    [0b00000, 0b00000, 0b01110, 0b10001, 0b11110, 0b10001, 0b11110, 0b10000] => 0xe2,
    [0b00000, 0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110, 0b00000] => 0x65,
    [0b00000, 0b00000, 0b01111, 0b10001, 0b10001, 0b10001, 0b01111, 0b00001] => 0xe7,
    [0b00000, 0b00000, 0b01111, 0b10100, 0b10010, 0b10001, 0b01110, 0b00000] => 0xe5,
    [0b00000, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b00000] => 0x78,
    [0b00000, 0b00000, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110, 0b00000] => 0x79,
    [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100, 0b00000] => 0x76,
    [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10001, 0b01111, 0b00001] => 0xf9,
    [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101, 0b00000] => 0x75,
    [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b11101, 0b10000] => 0xe4,
    [0b00000, 0b00000, 0b10001, 0b10001, 0b10101, 0b10101, 0b01010, 0b00000] => 0x77,
    [0b00000, 0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000, 0b00000] => 0x72,
    [0b00000, 0b00000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001, 0b00000] => 0x6e,
    [0b00000, 0b00000, 0b10110, 0b11001, 0b10001, 0b10001, 0b11110, 0b10000] => 0xf0,
    [0b00000, 0b00000, 0b11010, 0b10101, 0b10101, 0b10001, 0b10001, 0b00000] => 0x6d,
    [0b00000, 0b00000, 0b11110, 0b00010, 0b11110, 0b00010, 0b11110, 0b00000] => 0xae,
    [0b00000, 0b00000, 0b11110, 0b10001, 0b11110, 0b10000, 0b10000, 0b00000] => 0x70,
    [0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000] => 0x3d,
    [0b00000, 0b00000, 0b11111, 0b00001, 0b00110, 0b00100, 0b01000, 0b00000] => 0xa7,
    [0b00000, 0b00000, 0b11111, 0b00010, 0b00100, 0b01000, 0b11111, 0b00000] => 0x7a,
    [0b00000, 0b00000, 0b11111, 0b01000, 0b01111, 0b01001, 0b10001, 0b00000] => 0xfb,
    [0b00000, 0b00000, 0b11111, 0b01010, 0b01010, 0b01010, 0b10011, 0b00000] => 0xf7,
    [0b00000, 0b00000, 0b11111, 0b10101, 0b11111, 0b10001, 0b10001, 0b00000] => 0xfc,
    [0b00000, 0b00001, 0b00001, 0b01010, 0b00100, 0b01010, 0b10000, 0b00000] => 0xd2,
    [0b00000, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b00000, 0b00000] => 0x2f,
    [0b00000, 0b00001, 0b11110, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000] => 0xfa,
    [0b00000, 0b00010, 0b11010, 0b00010, 0b00000, 0b00000, 0b00000, 0b00000] => 0xe9,
    [0b00000, 0b00100, 0b00010, 0b10001, 0b10001, 0b10001, 0b10001, 0b00000] => 0xca,
    [0b00000, 0b00100, 0b00010, 0b11111, 0b00010, 0b00100, 0b00000, 0b00000] => 0x7e,
    [0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000, 0b00000] => 0x2b,
    [0b00000, 0b00100, 0b01000, 0b10000, 0b10001, 0b11111, 0b00001, 0b00000] => 0xd1,
    [0b00000, 0b00100, 0b01000, 0b11111, 0b01000, 0b00100, 0b00000, 0b00000] => 0x7f,
    [0b00000, 0b00100, 0b01110, 0b10100, 0b10101, 0b01110, 0b00100, 0b00000] => 0xec,
    [0b00000, 0b00100, 0b10100, 0b10100, 0b10101, 0b10101, 0b10110, 0b00000] => 0xd9,
    [0b00000, 0b00100, 0b10101, 0b01110, 0b10101, 0b00100, 0b00000, 0b00000] => 0x2a,
    [0b00000, 0b01000, 0b10100, 0b00010, 0b00001, 0b00001, 0b00000, 0b00000] => 0xcd,
    [0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b00100, 0b01000, 0b00000] => 0x3b,
    [0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b01100, 0b00000, 0b00000] => 0x3a,
    [0b00000, 0b01110, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111, 0b00000] => 0xc6,
    [0b00000, 0b01110, 0b00000, 0b01110, 0b00000, 0b01110, 0b00001, 0b00000] => 0xd0,
    [0b00000, 0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b11111, 0b00000] => 0xd5,
    [0b00000, 0b01110, 0b10001, 0b11111, 0b10001, 0b10001, 0b01110, 0b00000] => 0xf2,
    [0b00000, 0b01111, 0b01001, 0b10001, 0b00001, 0b00010, 0b01100, 0b00000] => 0xb8,
    [0b00000, 0b01111, 0b01001, 0b10101, 0b00011, 0b00010, 0b01100, 0b00000] => 0xc0,
    [0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110, 0b00000] => 0x67,
    [0b00000, 0b10000, 0b10000, 0b10001, 0b10010, 0b10100, 0b11000, 0b00000] => 0xda,
    [0b00000, 0b10001, 0b10001, 0b01001, 0b00001, 0b00010, 0b01100, 0b00000] => 0xbf,
    [0b00000, 0b10100, 0b01000, 0b10100, 0b00000, 0b00000, 0b00000, 0b00000] => 0xeb,
    [0b00000, 0b10101, 0b10101, 0b10101, 0b00001, 0b00010, 0b00100, 0b00000] => 0xc2,
    [0b00000, 0b11000, 0b00000, 0b00001, 0b00001, 0b00010, 0b11100, 0b00000] => 0xdd,
    [0b00000, 0b11000, 0b00001, 0b11001, 0b00001, 0b00010, 0b11100, 0b00000] => 0xbc,
    [0b00000, 0b11111, 0b00001, 0b00001, 0b00001, 0b00001, 0b11111, 0b00000] => 0xba,
    [0b00000, 0b11111, 0b00001, 0b00001, 0b00001, 0b00010, 0b01100, 0b00000] => 0xcc,
    [0b00000, 0b11111, 0b00001, 0b00001, 0b01010, 0b00100, 0b00010, 0b00000] => 0xcf,
    [0b00000, 0b11111, 0b00001, 0b00010, 0b00100, 0b01010, 0b10001, 0b00000] => 0xbd,
    [0b00000, 0b11111, 0b00001, 0b01010, 0b00100, 0b01010, 0b10000, 0b00000] => 0xc7,
    [0b00000, 0b11111, 0b00001, 0b11111, 0b00001, 0b00001, 0b11111, 0b00000] => 0xd6,
    [0b00000, 0b11111, 0b00001, 0b11111, 0b00001, 0b00010, 0b00100, 0b00000] => 0xa6,
    [0b00000, 0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111, 0b00000] => 0xb4,
    [0b00000, 0b11111, 0b01000, 0b11111, 0b01000, 0b01000, 0b00111, 0b00000] => 0xd3,
    [0b00000, 0b11111, 0b10001, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000] => 0xdc,
    [0b00000, 0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b11111, 0b00000] => 0xdb,
    [0b00001, 0b00001, 0b01101, 0b10011, 0b10001, 0b10001, 0b01111, 0b00000] => 0x64,
    [0b00001, 0b00010, 0b00100, 0b01100, 0b10100, 0b00100, 0b00100, 0b00000] => 0xb2,
    [0b00010, 0b00000, 0b00110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010] => 0xea,
    [0b00010, 0b00000, 0b00110, 0b00010, 0b00010, 0b10010, 0b01100, 0b00000] => 0x6a,
    [0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000, 0b00000] => 0xc9,
    [0b00010, 0b00100, 0b00100, 0b01000, 0b00100, 0b00100, 0b00010, 0b00000] => 0x7b,
    [0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010, 0b00000] => 0x28,
    [0b00010, 0b00100, 0b01000, 0b10000, 0b01000, 0b00100, 0b00010, 0b00000] => 0x3c,
    [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010, 0b00000] => 0x34,
    [0b00010, 0b11100, 0b00100, 0b11111, 0b00100, 0b00100, 0b01000, 0b00000] => 0xc1,
    [0b00010, 0b11111, 0b00010, 0b00110, 0b01010, 0b10010, 0b00010, 0b00000] => 0xb5,
    [0b00100, 0b00000, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110, 0b00000] => 0x69,
    [0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00000, 0b00100, 0b00000] => 0x21,
    [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000] => 0x7c,
    [0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b01000, 0b10000, 0b00000] => 0xc5,
    [0b00100, 0b01010, 0b10001, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000] => 0x5e,
    [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110, 0b00000] => 0x31,
    [0b00100, 0b01111, 0b10100, 0b01110, 0b00101, 0b11110, 0b00100, 0b00000] => 0x24,
    [0b00100, 0b10010, 0b01000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000] => 0xde,
    [0b00100, 0b11111, 0b00010, 0b00100, 0b01110, 0b10101, 0b00100, 0b00000] => 0xc8,
    [0b00100, 0b11111, 0b00100, 0b00100, 0b10101, 0b10101, 0b00100, 0b00000] => 0xce,
    [0b00100, 0b11111, 0b00100, 0b11111, 0b00100, 0b00100, 0b00100, 0b00000] => 0xb7,
    [0b00100, 0b11111, 0b10001, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000] => 0xb3,
    [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110, 0b00000] => 0x36,
    [0b00110, 0b01001, 0b01000, 0b11100, 0b01000, 0b01000, 0b01000, 0b00000] => 0x66,
    [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100, 0b00000] => 0x4a,
    [0b00111, 0b00100, 0b00100, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000] => 0xa2,
    [0b01000, 0b00100, 0b00010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000] => 0x60,
    [0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000, 0b00000] => 0x3e,
    [0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000, 0b00000] => 0x29,
    [0b01000, 0b00100, 0b00100, 0b00010, 0b00100, 0b00100, 0b01000, 0b00000] => 0x7d,
    [0b01000, 0b01000, 0b01000, 0b01100, 0b01010, 0b01000, 0b01000, 0b00000] => 0xc4,
    [0b01000, 0b01000, 0b11100, 0b01000, 0b01000, 0b01001, 0b00110, 0b00000] => 0x74,
    [0b01000, 0b01000, 0b11100, 0b01000, 0b11100, 0b01000, 0b01111, 0b00000] => 0xed,
    [0b01000, 0b01000, 0b11111, 0b01001, 0b01010, 0b01000, 0b01000, 0b00000] => 0xd4,
    [0b01000, 0b01111, 0b10010, 0b00010, 0b00010, 0b00010, 0b00100, 0b00000] => 0xb9,
    [0b01000, 0b11111, 0b01001, 0b01001, 0b01001, 0b01001, 0b10010, 0b00000] => 0xb6,
    [0b01000, 0b11111, 0b01001, 0b01010, 0b01000, 0b01000, 0b00111, 0b00000] => 0xbe,
    [0b01010, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111, 0b00000] => 0xe1,
    [0b01010, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110, 0b00000] => 0xef,
    [0b01010, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101, 0b00000] => 0xf5,
    [0b01010, 0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000] => 0x22,
    [0b01010, 0b01010, 0b11111, 0b01010, 0b11111, 0b01010, 0b01010, 0b00000] => 0x23,
    [0b01010, 0b11111, 0b01010, 0b01010, 0b00010, 0b00100, 0b01000, 0b00000] => 0xbb,
    [0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110, 0b00000] => 0x6c,
    [0b01100, 0b00100, 0b01000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000] => 0x27,
    [0b01100, 0b10010, 0b10100, 0b01000, 0b10101, 0b10010, 0b01101, 0b00000] => 0x26,
    [0b01110, 0b00000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001, 0b00000] => 0xee,
    [0b01110, 0b00000, 0b11111, 0b00001, 0b00001, 0b00010, 0b00100, 0b00000] => 0xd7,
    [0b01110, 0b00000, 0b11111, 0b00100, 0b00100, 0b00100, 0b01000, 0b00000] => 0xc3,
    [0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110, 0b00000] => 0x5d,
    [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110, 0b00000] => 0x49,
    [0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110, 0b00000] => 0x5b,
    [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100, 0b00000] => 0x3f,
    [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111, 0b00000] => 0x32,
    [0b01110, 0b10001, 0b00001, 0b01101, 0b10101, 0b10101, 0b01110, 0b00000] => 0x40,
    [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110, 0b00000] => 0x43,
    [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111, 0b00000] => 0x47,
    [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110, 0b00000] => 0x38,
    [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100, 0b00000] => 0x39,
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110, 0b00000] => 0x4f,
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101, 0b00000] => 0x51,
    [0b01110, 0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b00000] => 0x41,
    [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110, 0b00000] => 0x30,
    [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110, 0b00000] => 0x53,
    [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111, 0b00000] => 0x4c,
    [0b10000, 0b10000, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b00000] => 0x6b,
    [0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001, 0b00000] => 0x68,
    [0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b11110, 0b00000] => 0x62,
    [0b10000, 0b10000, 0b11111, 0b10000, 0b10000, 0b10000, 0b01111, 0b00000] => 0xcb,
    [0b10001, 0b01010, 0b11111, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000] => 0x5c,
    [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001, 0b00000] => 0x58,
    [0b10001, 0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00000] => 0x59,
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100, 0b00000] => 0x56,
    [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110, 0b00000] => 0x55,
    [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010, 0b00000] => 0x57,
    [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001, 0b00000] => 0x48,
    [0b10001, 0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b00000] => 0x4e,
    [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001, 0b00000] => 0x4b,
    [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001, 0b00000] => 0x4d,
    [0b10010, 0b10010, 0b10010, 0b10010, 0b00010, 0b00100, 0b01000, 0b00000] => 0xd8,
    [0b11000, 0b11001, 0b00010, 0b00100, 0b01000, 0b10011, 0b00011, 0b00000] => 0x25,
    [0b11100, 0b10010, 0b10001, 0b10001, 0b10001, 0b10010, 0b11100, 0b00000] => 0x44,
    [0b11100, 0b10100, 0b11100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000] => 0xdf,
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000, 0b00000] => 0x50,
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110, 0b00000] => 0x42,
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001, 0b00000] => 0x52,
    [0b11111, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b00000] => 0xf8,
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00000] => 0x37,
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111, 0b00000] => 0x5a,
    [0b11111, 0b00001, 0b00101, 0b00110, 0b00100, 0b00100, 0b01000, 0b00000] => 0xb1,
    [0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110, 0b00000] => 0x33,
    [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000] => 0x54,
    [0b11111, 0b10000, 0b01000, 0b00100, 0b01000, 0b10000, 0b11111, 0b00000] => 0xf6,
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000, 0b00000] => 0x46,
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111, 0b00000] => 0x45,
    [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110, 0b00000] => 0x35,
    [0b11111, 0b11111, 0b11111, 0b11111, 0b11111, 0b11111, 0b11111, 0b11111] => 0xff,
};