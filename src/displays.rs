//! Predefined display setting.

use embedded_graphics_core::pixelcolor::BinaryColor;

use crate::{
    framebuffer::{Framebuffer4Bit, FramebufferBW, Sharp, JDI},
    pixelcolor::Rgb111,
    DisplaySpec,
};

/// 1.28 inch Memory LCD, aka. LPM013M126C
pub struct LPM013M126A<COLOR = Rgb111> {
    _color: core::marker::PhantomData<COLOR>,
}

impl DisplaySpec for LPM013M126A {
    const WIDTH: u16 = 176;
    const HEIGHT: u16 = 176;

    type Framebuffer = Framebuffer4Bit<{ Self::WIDTH }, { Self::HEIGHT }>;
}

impl DisplaySpec for LPM013M126A<BinaryColor> {
    const WIDTH: u16 = 176;
    const HEIGHT: u16 = 176;

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }, JDI>;
}

/// 0.85inch 8-color display
pub struct LPM009M360A<COLOR = Rgb111> {
    _color: core::marker::PhantomData<COLOR>,
}

impl DisplaySpec for LPM009M360A {
    const WIDTH: u16 = 72;
    const HEIGHT: u16 = 144;

    type Framebuffer = Framebuffer4Bit<{ Self::WIDTH }, { Self::HEIGHT }>;
}

impl DisplaySpec for LPM009M360A<BinaryColor> {
    const WIDTH: u16 = 72;
    const HEIGHT: u16 = 144;

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }, JDI>;
}

/// 0.56inch, 64x64 BW display, 13pin 0.3mm FPC
pub struct LS006B7DH01;

impl DisplaySpec for LS006B7DH01 {
    const WIDTH: u16 = 64;
    const HEIGHT: u16 = 64;

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }, Sharp>;
}

/// 1.28inch, 128x128 BW display, 10pin 0.5mm FPC
pub struct LS013B7DH03;

impl DisplaySpec for LS013B7DH03 {
    const WIDTH: u16 = 128;
    const HEIGHT: u16 = 128;

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }, Sharp>;
}
