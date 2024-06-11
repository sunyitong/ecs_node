mod core;
mod resource;

use std::thread;
use std::time::Duration;
// use crate::core::display_arm::Display;
use crate::core::display_mock::*;
use crate::core::display_trait::*;
use crate::core::display_style::*;
use crate::resource::resource_image::*;


fn main() {
    let mut display = Display::new(480,480,4);
    display.clean();
    display.draw_image_bmp(200,150,IMAGE_TEST);
    display.draw_line(0,0,480,480,LINE_STYLE_TEST);
    display.draw_rectangle(10,10,50,100,RECT_STYLE_TEST);
    display.draw_rectangle_round(100,100,100,200,50,RECT_STYLE_TEST);
    display.draw_circle(0,0,100,LINE_STYLE_TEST);
    display.draw_triangle(50,10,10,200, 200,200,LINE_STYLE_TEST);
    display.draw_text("Hello World, Rust. ECS Node Test!",50, 350, TEXT_STYLE_LARGE);
    display.draw_text("Hello World, Rust. ECS Node Test!",50, 400, TEXT_STYLE_SMALL);
    
    display.update();
    thread::sleep(Duration::from_millis(5000));
}