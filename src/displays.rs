//! Predefined display setting.

use embedded_graphics_core::pixelcolor::BinaryColor;

use crate::{
    framebuffer::{Framebuffer4Bit, FramebufferBW},
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

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }>;
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

    type Framebuffer = FramebufferBW<{ Self::WIDTH }, { Self::HEIGHT }>;
}
