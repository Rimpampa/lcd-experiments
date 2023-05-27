use core::iter::zip;

use super::ddrom;
use super::Bitmap;

pub type CgRam = heapless::Vec<[u8; 8], 8>;
pub type DdRam = [u8; 16];

/// How to handle the gap between each charatecter of the display
///
/// LCD character display usually have a gap of inactive pixels
/// between a character and the other, which must be considered
/// when implementing the desired behaviour
#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub enum Gap {
    /// Skip the gap as it was not there
    #[default]
    Skip,
    /// Handle the pixel as if they where visibile
    ///
    /// This in turn result in those pixels being "hidden"
    /// behind the inactive pixels
    Hide,
}

#[derive(Default)]
pub struct Canvas {
    data: [[u8; 8]; 16],
    gap: Gap,
}

impl Canvas {
    pub fn render(&self) -> (DdRam, CgRam) {
        let mut cgram = CgRam::new();
        let mut ddram = DdRam::default();
        for (ddram, ch) in zip(&mut ddram, self.data.map(Bitmap::new)) {
            *ddram = Self::render_char(ch, &mut cgram);
        }
        (ddram, cgram)
    }

    /// Renders a character from the [`Canvas`]
    fn render_char(ch: Bitmap, cgram: &mut CgRam) -> u8 {
        let raw = ch.raw();
        ddrom::search(ch)
            .or_else(|| Some(cgram.iter().position(|&c| c == raw)? as u8))
            .or_else(|| cgram.push(raw).map(|_| cgram.len() as u8 - 1).ok())
            .unwrap_or_else(|| b' ')
    }

    /// Write the given text onto the canvas
    ///
    /// If the text doesn't fit inside the drawing area
    /// it will wrap around,
    pub fn write(&mut self, text: &str, custom_gap: Option<Gap>) {
        for (i, s) in text.chars().enumerate() {
            // NOTE:
            // When `Gap::Skip` the space between the characters has to be placed
            // manually
            if let Gap::Skip = custom_gap.unwrap_or(self.gap) {
                self.shift_left(Some(Gap::Skip));
            }
            self.data[i % self.data.len()] = Bitmap::render(s).raw();
        }
    }

    /// Shift the contents of the [`Canvas`] one pixel to the left
    pub fn shift_left(&mut self, custom_gap: Option<Gap>) {
        // NOTE:
        // When `Gap::Hide` use the 6th least significan bit to store
        // of each line to store the hidden pixel
        let shift = 5 + u8::from(custom_gap.unwrap_or(self.gap) == Gap::Hide);
        let mask = (1 << shift) - 1;

        self.data.iter_mut().flatten().for_each(|v| *v <<= 1);
        for x in 0usize..16 {
            for y in 0..8 {
                let prev = x.checked_sub(1).unwrap_or(15);
                self.data[prev][y] |= self.data[x][y] >> shift;
                self.data[x][y] &= mask;
            }
        }
    }
}
