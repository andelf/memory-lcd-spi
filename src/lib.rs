//! Memory LCD display driver in SPI mode

#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use core::ops::{Deref, DerefMut};

use crate::error::Error;
use embedded_hal_1::{delay::DelayUs, digital::OutputPin, spi::SpiBusWrite};
use framebuffer::FramebufferType;

pub mod displays;
pub mod error;
pub mod framebuffer;
pub mod pixelcolor;

/// Specification for displays.
pub trait DisplaySpec {
    const WIDTH: u16;
    const HEIGHT: u16;

    type Framebuffer: FramebufferType;
}

// NOTE: The commands are actually 6 bit, but the MSB of next 10 bit is always 0,
// So it's a trick to use 8 bit to represent the command.
pub const CMD_NO_UPDATE: u8 = 0x00;
pub const CMD_BLINKING_BLACK: u8 = 0x10;
pub const CMD_BLINKING_INVERSION: u8 = 0x14;
pub const CMD_BLINKING_WHITE: u8 = 0x18;
pub const CMD_ALL_CLEAR: u8 = 0x20;
pub const CMD_VCOM: u8 = 0x40;
// 0x90 = 0b100100_00
// Use 4 bit data input
pub const CMD_UPDATE_4BIT: u8 = 0x90;
// Also apply to Sharp's update mode: 0b100_xxxxx
pub const CMD_UPDATE_1BIT: u8 = 0x88;

pub struct MemoryLCD<SPEC: DisplaySpec, SPI, CS> {
    spi: SPI,
    cs: CS,
    framebuffer: SPEC::Framebuffer,
}

impl<SPEC, SPI, CS> MemoryLCD<SPEC, SPI, CS>
where
    SPI: SpiBusWrite<u8>,
    CS: OutputPin,
    SPEC: DisplaySpec,
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self {
            spi,
            cs,
            framebuffer: SPEC::Framebuffer::default(),
        }
    }

    pub fn turn_on_display<DISP: OutputPin>(self, disp: &mut DISP) -> Result<(), Error<SPI::Error, DISP::Error>> {
        disp.set_high().map_err(Error::Gpio)?;
        Ok(())
    }

    pub fn turn_off_display<DISP: OutputPin>(self, disp: &mut DISP) -> Result<(), Error<SPI::Error, DISP::Error>> {
        disp.set_low().map_err(Error::Gpio)?;
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), Error<SPI::Error, CS::Error>> {
        self.cs.set_high().map_err(Error::Gpio)?;
        self.spi.write(&[CMD_ALL_CLEAR, 0x00]).map_err(Error::Spi)?;
        self.cs.set_low().map_err(Error::Gpio)?;
        Ok(())
    }

    pub fn update<D: DelayUs>(&mut self, _delay: &mut D) -> Result<(), Error<SPI::Error, CS::Error>> {
        use crate::framebuffer::sealed::FramebufferSpiUpdate;

        self.cs.set_high().map_err(Error::Gpio)?;
        self.framebuffer.update(&mut self.spi).map_err(Error::Spi)?;
        self.cs.set_low().map_err(Error::Gpio)?;
        Ok(())
    }
}

impl<SPEC, SPI, CS> Deref for MemoryLCD<SPEC, SPI, CS>
where
    SPEC: DisplaySpec,
{
    type Target = SPEC::Framebuffer;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<SPEC, SPI, CS> DerefMut for MemoryLCD<SPEC, SPI, CS>
where
    SPEC: DisplaySpec,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.framebuffer
    }
}

/// A dummy pin if your `CS` is controlled by the SPI driver
pub struct NoCS;

impl embedded_hal_1::digital::ErrorType for NoCS {
    type Error = core::convert::Infallible;
}

impl OutputPin for NoCS {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
