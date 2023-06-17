# memory-lcd-spi

`embedded-hal` driver for Sharp's Memory LCD and JDI's Memory In Pixel displays.

[![crates.io](https://img.shields.io/crates/v/memory-lcd-spi.svg)](https://crates.io/crates/memory-lcd-spi)
[![Docs](https://docs.rs/memory-lcd-spi/badge.svg)](https://docs.rs/memory-lcd-spi)

## Features

- Rotation support
- 8-color mode with `Rgb111` color
- black/white mode for fast update

## Tested

- JDI's LPM013M126A or LPM013M126C, 176x176 1.3inch
- JDI's LPM009M360A, 72x144 0.9inch
- Sharp's LS006B7DH01, 64x64 0.56inch

## Usage

```rust
let mut display: MemoryLCD<LPM009M360A<BinaryColor>, _, _> = MemoryLCD::new(spi, cs);

display.set_rotation(memory_lcd_spi::framebuffer::Rotation::Deg90);
display.clear(BinaryColor::Off);

// drawing code with embedded-graphics
Line::new(
    Point::new(0, 0),
    Point::new(20, 20),
)
.into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
.draw(&mut *display) // Yes, explicit deref is required
.unwrap();

display.update(&mut delay);
```

Or `Rgb111` mode:

```rust
let mut display: MemoryLCD<LPM013M126A<Rgb111>, _, _> = MemoryLCD::new(spi, cs);
display.clear(Rgb111::BLACK);
```

> **Note**
>
> `DISP` pin is not managed by this driver. You should control it by yourself.
>
> `EXTCOMIN` in is not managed by this driver. Follow the datasheet, use either 60Hz PWM or GND.
