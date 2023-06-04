# memory-lcd-spi

## Features

- Rotation support
- 8-color mode with `Rgb111` color
- black/white mode for fast update

## Tested

- JDI's LPM013M126A or LPM013M126C, 176x176 1.3inch
- JDI's LPM009M360A, 72x144 0.9inch

## Usage

```rust
let mut display: MemoryLCD<LPM009M360A<BinaryColor>, _, _> = MemoryLCD::new(spi, cs);

display.set_rotation(memory_lcd_spi::framebuffer::Rotation::Deg90);
display.clear(BinaryColor::Off);

// drawing code with embedded-graphics
display.update(&mut delay);
```

Or `Rgb111` mode:

```rust
let mut display: MemoryLCD<LPM013M126A<Rgb111>, _, _> = MemoryLCD::new(spi, cs);
display.clear(Rgb111::BLACK);
```

> **NOTE**: `DISP` pin is not managed by this driver. You should control it by yourself.
