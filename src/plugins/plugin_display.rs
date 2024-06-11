use bevy::prelude::*;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(windows)]
use crate::core::display_mock::*;

use crate::platform::platform_data::*;

pub struct PluginDisplay;

impl Plugin for PluginDisplay {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_display);
    }
}

fn setup_display(world: &mut World){
    let display = Display::new(DISPLAY_WIDTH, DISPLAY_HEIGHT, BYTES_PER_PIXCEL);
    world.insert_non_send_resource(display);
}