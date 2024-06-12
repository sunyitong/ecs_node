pub use embedded_graphics::{
    prelude::*,
    pixelcolor::{Rgb888},
    geometry::Size,
    mono_font:: MonoTextStyle,
    text::{Text, Alignment},
    image::Image,
};
use minifb::{Window, WindowOptions};
use embedded_graphics::{
    primitives::{Rectangle, RoundedRectangle, Line, Circle, Triangle, PrimitiveStyle},
};
use tinybmp::Bmp;
use crate::core::display_trait::{DisplayDraw, DisplayUpdate};

pub struct Display {
    width: usize,
    height: usize,
    line_byte_length: usize,
    bytes_per_pixel: usize,
    buffer: Vec<u8>,
    window_win:Window,
    simulator_buffer: Vec<u32>,
}

impl Display {
    pub fn new(width:usize, height:usize, bytes_per_pixel:usize) -> Self {
        let line_byte_length = width * bytes_per_pixel;
        let buffer= vec![0u8; line_byte_length * height];
        let simulator_buffer = vec![0u32; width * height];


        Display {
            width,
            height,
            line_byte_length,
            bytes_per_pixel,
            buffer,
            window_win: Window::new(
                "Display Simulator",
                width,
                height,
                WindowOptions::default(),
            ).expect("Unable to create window"),
            simulator_buffer,
        }
    }

    fn set_pixel_color (&mut self, x:usize, y:usize, color:(u8,u8,u8)) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) * self.bytes_per_pixel;
            self.buffer[index + 2] = color.0;     // Red
            self.buffer[index + 1] = color.1; // Green
            self.buffer[index] = color.2; // Blue
        }
    }

    fn convert_buffer_to_simulator_all(&mut self) {
        for (i, chunk) in self.buffer.chunks(4).enumerate() {
            let r = u32::from(chunk[0]);
            let g = u32::from(chunk[1]) << 8;
            let b = u32::from(chunk[2]) << 16;
            self.simulator_buffer[i] = r | g | b;
        }
    }
}

impl DisplayUpdate for Display{
    fn clean(&mut self) {
        self.buffer.fill(0);
    }

    fn start(&mut self) {
        self.buffer.fill(0);
    }

    fn update(&mut self) {
        self.convert_buffer_to_simulator_all();

        // updating frame with buffer
        self.window_win.update_with_buffer(&self.simulator_buffer,self.width,self.height).unwrap();

        if !self.window_win.is_open(){
            std::process::exit(0);
        }
    }
}

impl DisplayDraw for Display{
    fn draw_line(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, style: PrimitiveStyle<Rgb888>) {
        Line::new(Point::new(start_x, start_y), Point::new(end_x, end_y))
            .into_styled(style)
            .draw(self).unwrap();
    }

    fn draw_rectangle(&mut self, start_x: i32, start_y: i32, size_width: u32, size_height: u32, style: PrimitiveStyle<Rgb888>) {
        Rectangle::new(Point::new(start_x, start_y), Size::new(size_width, size_height))
            .into_styled(style)
            .draw(self).unwrap();
    }

    fn draw_rectangle_round(&mut self, start_x: i32, start_y: i32, size_width: u32, size_height: u32, round: u32, style: PrimitiveStyle<Rgb888>) {
        RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::new(start_x, start_y), Size::new(size_width, size_height)),
            Size::new(round, round),
        )
            .into_styled(style)
            .draw(self).unwrap();
    }

    fn draw_circle(&mut self, corner_x: i32, corner_y: i32, diameter: u32, style: PrimitiveStyle<Rgb888>) {
        Circle::new(Point::new(corner_x, corner_y), diameter)
            .into_styled(style)
            .draw(self).unwrap();
    }

    fn draw_triangle(&mut self, p1_x: i32, p1_y: i32, p2_x: i32, p2_y: i32, p3_x: i32, p3_y: i32, style: PrimitiveStyle<Rgb888>) {
        Triangle::new(Point::new(p1_x, p1_y), Point::new(p2_x, p2_y), Point::new(p3_x, p3_y))
            .into_styled(style)
            .draw(self).unwrap();
    }

    fn draw_text(&mut self, t: &str, start_x: i32, start_y: i32, style: MonoTextStyle<Rgb888>) {
        Text::new(t, Point::new(start_x, start_y), style).draw(self).unwrap();
    }
    
    fn draw_text_right(&mut self, t: &str, start_x: i32, start_y: i32, style: MonoTextStyle<Rgb888>) {
        Text::with_alignment(t,Point::new(start_x, start_y), style, Alignment::Right).draw(self).unwrap();
    }

    fn draw_text_center(&mut self, t: &str, start_x: i32, start_y: i32, style: MonoTextStyle<Rgb888>) {
        Text::with_alignment(t,Point::new(start_x, start_y), style, Alignment::Center).draw(self).unwrap();
    }

    fn draw_image_bmp(&mut self, start_x: i32, start_y: i32, bytes: &[u8]) {
        let image: Bmp<Rgb888> = Bmp::from_slice(bytes).unwrap();
        Image::new(&image, Point::new(start_x, start_y)).draw(self).unwrap();
    }
}

impl DrawTarget for Display {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>> {
        for Pixel(coord, color) in pixels {
            let (x, y) = (coord.x as usize, coord.y as usize);
            if x < self.width && y < self.height {
                let rgb = (color.r(), color.g(), color.b());
                self.set_pixel_color(x, y, rgb);
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}