use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::primitives::PrimitiveStyle;
use tinybmp::Bmp;

pub trait DisplayDraw {
    fn draw_line(&mut self,start_x:i32, start_y:i32, end_x:i32, end_y:i32, style:PrimitiveStyle<Rgb888>);
    fn draw_rectangle(&mut self, start_x:i32, start_y:i32, size_width:u32, size_height:u32, style:PrimitiveStyle<Rgb888>);
    fn draw_rectangle_round(&mut self, start_x:i32, start_y:i32, size_width:u32, size_height:u32, round:u32, style:PrimitiveStyle<Rgb888>);
    fn draw_circle(&mut self, corner_x:i32, corner_y:i32, diameter:u32, style:PrimitiveStyle<Rgb888>);
    fn draw_triangle(&mut self, p1_x:i32, p1_y:i32, p2_x:i32, p2_y:i32, p3_x:i32, p3_y:i32, style:PrimitiveStyle<Rgb888>);
    fn draw_text(&mut self, t:&str, start_x:i32, start_y:i32, style:MonoTextStyle<Rgb888>);
    fn draw_image_bmp(&mut self, start_x:i32, start_y:i32, bytes: &[u8]);
}

pub trait DisplayUpdate {
    fn clean(&mut self);
    fn start(&mut self);
    fn update(&mut self);
}