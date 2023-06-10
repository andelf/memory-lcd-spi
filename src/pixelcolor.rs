use embedded_graphics_core::{
    pixelcolor::{
        raw::{RawU24, RawU4},
        BinaryColor, Rgb888,
    },
    prelude::{PixelColor, RawData, RgbColor},
};

/// Rgb111 color used in color memory LCDs. Format: `0bRGB0`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Rgb111(pub RawU4);

impl PixelColor for Rgb111 {
    type Raw = RawU4;
}

impl Rgb111 {
    pub fn from_u3(raw: u8) -> Self {
        Self(RawU4::new((raw & 0b111) << 1))
    }
}

impl RgbColor for Rgb111 {
    fn r(&self) -> u8 {
        (self.0.into_inner() >> 3) & 0b1
    }

    fn g(&self) -> u8 {
        (self.0.into_inner() >> 2) & 0b1
    }

    fn b(&self) -> u8 {
        (self.0.into_inner() >> 1) & 0b1
    }

    const MAX_R: u8 = 0b1;
    const MAX_G: u8 = 0b1;
    const MAX_B: u8 = 0b1;

    const BLACK: Self = Self(RawU4::new(0));
    const RED: Self = Self(RawU4::new(0b1000));
    const GREEN: Self = Self(RawU4::new(0b0100));
    const BLUE: Self = Self(RawU4::new(0b0010));
    const YELLOW: Self = Self(RawU4::new(0b1100));
    const MAGENTA: Self = Self(RawU4::new(0b1010));
    const CYAN: Self = Self(RawU4::new(0b0110));
    const WHITE: Self = Self(RawU4::new(0b1110));
}

impl From<BinaryColor> for Rgb111 {
    fn from(color: BinaryColor) -> Self {
        match color {
            BinaryColor::Off => Self::BLACK,
            BinaryColor::On => Self::WHITE,
        }
    }
}

impl From<RawU4> for Rgb111 {
    fn from(raw: RawU4) -> Self {
        Self(raw)
    }
}

impl From<Rgb888> for Rgb111 {
    fn from(color: Rgb888) -> Self {
        let raw: RawU24 = color.into();
        let storage = raw.into_inner();

        let red = ((storage >> 16) as u8 >> 7) & 0b1;
        let green = ((storage >> 8) as u8 >> 7) & 0b1;
        let blue = (storage as u8 >> 7) & 0b1;

        Rgb111(RawU4::new((red << 3) | (green << 2) | (blue << 1)))
    }
}
