mod core;
mod resource;
mod plugins;
mod platform;
mod systems;
mod components;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(windows)]
use crate::core::display_mock::*;
use crate::core::display_trait::*;
use crate::core::display_style::*;
use crate::resource::resource_image::*;

use bevy::{app::ScheduleRunnerPlugin, prelude::*, utils::Duration};
use crate::plugins::fps_check::FpsCheck;
use crate::plugins::plugin_display::PluginDisplay;
use crate::systems::system_node::{draw_test_node, spawn_test_node, update_test_node};
use crate::systems::system_tilemap::draw_tile_map;

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
        .add_systems(Startup, spawn_test_node)
        .add_systems(Update, (draw_tile_map, update_test_node, draw_test_node).chain())
        .run();
}
