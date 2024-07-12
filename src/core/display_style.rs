use embedded_graphics::{
    mono_font::{ascii::{FONT_6X10, FONT_6X13_BOLD, FONT_9X18, FONT_10X20}, MonoTextStyle}, 
    pixelcolor::WebColors, 
    primitives::PrimitiveStyleBuilder
};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::primitives::PrimitiveStyle;
use tinybmp::Bmp;

pub static LINE_STYLE_TEST: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::WHITE).stroke_width(1).build();

pub static TILE_MAP_STYLE_TEST: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::new(40,40,40)).stroke_width(1).build();

pub static RECT_STYLE_TEST: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::MAGENTA).stroke_width(1).build();

pub static PORT_STYLE_TEST: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::RED).stroke_width(1).fill_color(Rgb888::BLACK).build();

pub static TEXT_STYLE_SMALL: MonoTextStyle<Rgb888> = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);
pub static TEXT_STYLE_LARGE: MonoTextStyle<Rgb888> = MonoTextStyle::new(&FONT_9X18, Rgb888::WHITE);

pub static GLOBAL_AXIS_X: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::RED).stroke_width(1).build();
pub static GLOBAL_AXIS_Y: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::GREEN).stroke_width(1).build();

pub static FOCUS_POINT: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::WHITE).stroke_width(1).build();



// Background Color
pub static BACKGROUND_COLOR: u8 = 25;

// Node Style
pub static NODE_TEXT_STYLE: MonoTextStyle<Rgb888> = MonoTextStyle::new(&FONT_9X18, Rgb888::WHITE);

pub static NODE_PORT_TEXT_STYLE: MonoTextStyle<Rgb888> = MonoTextStyle::new(&FONT_9X18, Rgb888::CSS_DARK_ORANGE);

pub static NODE_PORT_STYLE: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .fill_color(Rgb888::new(255, 140, 0)).build();

pub static NODE_OUTLINE_SYTLE: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .fill_color(Rgb888::new(50, 50, 50)).build();

// Connection Style
pub static CONNECTION_LINE_STYLE: PrimitiveStyle<Rgb888> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb888::new(255, 150, 30)).stroke_width(1).build();


