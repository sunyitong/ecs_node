mod core;
mod resource;
mod plugins;
mod platform;

// use crate::core::display_arm::Display;
use crate::core::display_mock::*;
use crate::core::display_trait::*;
use crate::core::display_style::*;
use crate::resource::resource_image::*;

use bevy::{app::ScheduleRunnerPlugin, prelude::*, utils::Duration};
use crate::plugins::fps_check::FpsCheck;
use crate::plugins::plugin_display::PluginDisplay;


fn main() {
    // let mut display = Display::new(480,480,4);
    // display.clean();
    // display.draw_image_bmp(200,150,IMAGE_TEST);
    // display.draw_line(0,0,480,480,LINE_STYLE_TEST);
    // display.draw_rectangle(10,10,50,100,RECT_STYLE_TEST);
    // display.draw_rectangle_round(100,100,100,200,50,RECT_STYLE_TEST);
    // display.draw_circle(0,0,100,LINE_STYLE_TEST);
    // display.draw_triangle(50,10,10,200, 200,200,LINE_STYLE_TEST);
    // display.draw_text("Hello World, Rust. ECS Node Test!",50, 350, TEXT_STYLE_LARGE);
    // display.draw_text("Hello World, Rust. ECS Node Test!",50, 400, TEXT_STYLE_SMALL);
    // 
    // display.update();
    // thread::sleep(Duration::from_millis(5000));

    // This app loops forever at 60 fps
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 30.0,
            ))),
        )
        .add_plugins((PluginDisplay, FpsCheck))
        .insert_resource(TextPosition{coordinate: (0,0)})
        .add_systems(Update, ( read_display))
        .run();
}

#[derive(Resource)]
struct TextPosition {
    coordinate: (i32, i32)
}

fn read_display(
    mut display: NonSendMut<Display>,
    mut position: ResMut<TextPosition>
){
    display.clean();
    display.draw_text("From Bevy", position.coordinate.0, position.coordinate.1, TEXT_STYLE_LARGE);
    display.update();
    if position.coordinate.0 <= 480 {
        position.coordinate.0 += 1;
        position.coordinate.1 += 1;
    } else {
        position.coordinate.0 = 0;
        position.coordinate.1 = 0;
    }
}