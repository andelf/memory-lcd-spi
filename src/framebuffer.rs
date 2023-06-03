//! Framebuffer for memory displays
//!
//! Considerations:
//! - No flip or mirror support
//! - Rotation is needed
//! - TODO: double buffering

use embedded_graphics_core::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, OriginDimensions, RawData, Size},
    primitives::Rectangle,
    Pixel,
};
use embedded_hal_1::spi::SpiBusWrite;

use crate::pixelcolor::Rgb111;

/// Display rotation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rotation {
    /// No rotation.
    Deg0,
    /// 90° clockwise rotation.
    Deg90,
    /// 180° clockwise rotation.
    Deg180,
    /// 270° clockwise rotation.
    Deg270,
}

impl Rotation {
    #[inline]
    fn is_column_row_swap(self) -> bool {
        matches!(self, Rotation::Deg90 | Rotation::Deg270)
    }
}

pub(crate) mod sealed {
    use embedded_hal_1::spi::SpiBusWrite;

    pub trait FramebufferSpiUpdate {
        fn update<SPI: SpiBusWrite>(&self, spi: &mut SPI) -> Result<(), SPI::Error>;
    }
}

pub trait FramebufferType: OriginDimensions + DrawTarget + Default + sealed::FramebufferSpiUpdate {}

pub struct Framebuffer4Bit<const WIDTH: u16, const HEIGHT: u16>
where
    [(); WIDTH as usize * HEIGHT as usize / 2]:,
{
    data: [u8; WIDTH as usize * HEIGHT as usize / 2],
    rotation: Rotation,
}

impl<const WIDTH: u16, const HEIGHT: u16> Default for Framebuffer4Bit<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 2]:,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> sealed::FramebufferSpiUpdate for Framebuffer4Bit<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 2]:,
{
    // only burst update is supported
    fn update<SPI: SpiBusWrite>(&self, spi: &mut SPI) -> Result<(), SPI::Error> {
        for i in 0..HEIGHT {
            let start = (i as usize) * WIDTH as usize / 2;
            let end = start + WIDTH as usize / 2;
            let line_data = &self.data[start..end];
            // NOTE: refer to manual, gate address are start from 1 to HEIGHT
            spi.write(&[crate::CMD_UPDATE_4BIT, i as u8 + 1])?;
            spi.write(line_data)?;
        }
        spi.write(&[0x00, 0x00])?;
        Ok(())
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> Framebuffer4Bit<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 2]:,
{
    pub fn new() -> Self {
        Self {
            data: [0; WIDTH as usize * HEIGHT as usize / 2],
            rotation: Rotation::Deg0,
        }
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }

    pub(crate) fn set_pixel(&mut self, x: u16, y: u16, color: Rgb111) {
        if self.rotation.is_column_row_swap() {
            if x >= HEIGHT || y >= WIDTH {
                return;
            }
        } else {
            if y >= HEIGHT || x >= WIDTH {
                return;
            }
        }
        let x = x as usize;
        let y = y as usize;

        let (x, y) = match self.rotation {
            Rotation::Deg0 => (x, y),
            Rotation::Deg90 => (y, HEIGHT as usize - x - 1),
            Rotation::Deg180 => (WIDTH as usize - x - 1, HEIGHT as usize - y - 1),
            Rotation::Deg270 => (WIDTH as usize - y - 1, x),
        };

        let index = (y * WIDTH as usize + x) / 2;

        let color = color.0.into_inner();

        if x % 2 == 0 {
            self.data[index] = (self.data[index] & 0b00001111) | (color << 4);
        } else {
            self.data[index] = (self.data[index] & 0b11110000) | color;
        }
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> FramebufferType for Framebuffer4Bit<WIDTH, HEIGHT> where
    [(); WIDTH as usize * HEIGHT as usize / 2]:
{
}

impl<const WIDTH: u16, const HEIGHT: u16> OriginDimensions for Framebuffer4Bit<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 2]:,
{
    fn size(&self) -> Size {
        match self.rotation {
            Rotation::Deg0 | Rotation::Deg180 => Size::new(WIDTH as u32, HEIGHT as u32),
            Rotation::Deg90 | Rotation::Deg270 => Size::new(HEIGHT as u32, WIDTH as u32),
        }
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> DrawTarget for Framebuffer4Bit<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 2]:,
{
    type Color = Rgb111;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            self.set_pixel(coord.x as u16, coord.y as u16, color);
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let raw = color.0.into_inner() << 4 | color.0.into_inner();
        self.data.fill(raw);
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        if self.rotation == Rotation::Deg0 && area.top_left.x % 2 == 0 && area.size.width % 2 == 0 {
            let w = area.size.width as usize / 2;
            let x_off = area.top_left.x as usize / 2;
            let raw_pix = color.0.into_inner() << 4 | color.0.into_inner();

            for y in area.top_left.y..area.top_left.y + area.size.height as i32 {
                let start = (y as usize) * WIDTH as usize / 2 + x_off;
                let end = start + w;
                self.data[start..end].fill(raw_pix);
            }
            Ok(())
        } else {
            self.fill_contiguous(area, core::iter::repeat(color))
        }
    }
}

pub struct FramebufferBW<const WIDTH: u16, const HEIGHT: u16>
where
    [(); WIDTH as usize * HEIGHT as usize / 8]:,
{
    data: [u8; WIDTH as usize * HEIGHT as usize / 8],
    rotation: Rotation,
}

impl<const WIDTH: u16, const HEIGHT: u16> Default for FramebufferBW<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 8]:,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> FramebufferBW<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 8]:,
{
    pub fn new() -> Self {
        Self {
            data: [0; WIDTH as usize * HEIGHT as usize / 8],
            rotation: Rotation::Deg0,
        }
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }

    pub(crate) fn set_pixel(&mut self, x: u16, y: u16, color: BinaryColor) {
        if self.rotation.is_column_row_swap() {
            if x >= HEIGHT || y >= WIDTH {
                return;
            }
        } else {
            if y >= HEIGHT || x >= WIDTH {
                return;
            }
        }

        let x = x as usize;
        let y = y as usize;

        let (x, y) = match self.rotation {
            Rotation::Deg0 => (x, y),
            Rotation::Deg90 => (y, HEIGHT as usize - x - 1),
            Rotation::Deg180 => (WIDTH as usize - x - 1, HEIGHT as usize - y - 1),
            Rotation::Deg270 => (WIDTH as usize - y - 1, x),
        };

        if y >= HEIGHT as usize || x >= WIDTH as usize {
            return;
        }

        let index = y * WIDTH as usize + x;

        if color.is_on() {
            self.data[index / 8] |= 1 << (8 - (index % 8) - 1);
        } else {
            self.data[index / 8] &= !(1 << (8 - (index % 8) - 1));
        }
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> sealed::FramebufferSpiUpdate for FramebufferBW<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 8]:,
{
    fn update<SPI: SpiBusWrite>(&self, spi: &mut SPI) -> Result<(), SPI::Error> {
        for i in 0..HEIGHT {
            let start = (i as usize) * WIDTH as usize / 8;
            let end = start + WIDTH as usize / 8;
            let gate_line = &self.data[start..end];
            spi.write(&[crate::CMD_UPDATE_1BIT, i as u8])?;
            spi.write(gate_line)?;
        }
        spi.write(&[0x00, 0x00])?;
        Ok(())
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> FramebufferType for FramebufferBW<WIDTH, HEIGHT> where
    [(); WIDTH as usize * HEIGHT as usize / 8]:
{
}

impl<const WIDTH: u16, const HEIGHT: u16> OriginDimensions for FramebufferBW<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 8]:,
{
    fn size(&self) -> Size {
        match self.rotation {
            Rotation::Deg0 | Rotation::Deg180 => Size::new(WIDTH as u32, HEIGHT as u32),
            Rotation::Deg90 | Rotation::Deg270 => Size::new(HEIGHT as u32, WIDTH as u32),
        }
    }
}

impl<const WIDTH: u16, const HEIGHT: u16> DrawTarget for FramebufferBW<WIDTH, HEIGHT>
where
    [(); WIDTH as usize * HEIGHT as usize / 8]:,
{
    type Color = BinaryColor;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            self.set_pixel(coord.x as u16, coord.y as u16, color);
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        if color.is_on() {
            self.data.fill(0xFF);
        } else {
            self.data.fill(0x00);
        }
        Ok(())
    }
}
