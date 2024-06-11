use embedded_graphics::{
    primitives::{PrimitiveStyleBuilder},
    mono_font::{ascii::FONT_6X10, ascii::FONT_9X18, MonoTextStyle},
};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::primitives::PrimitiveStyle;
use tinybmp::Bmp;

pub static LINE_STYLE_TEST: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::WHITE).stroke_width(1).build();

pub static RECT_STYLE_TEST: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::RED).stroke_width(5).fill_color(Rgb888::GREEN).build();

pub static TEXT_STYLE_SMALL: MonoTextStyle<Rgb888> = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);
pub static TEXT_STYLE_LARGE: MonoTextStyle<Rgb888> = MonoTextStyle::new(&FONT_9X18, Rgb888::WHITE);