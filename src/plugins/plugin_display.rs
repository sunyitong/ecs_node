use bevy::prelude::*;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(any(windows, target_os = "macos"))]
use crate::core::display_mock::*;
use crate::core::display_trait::DisplayUpdate;

use crate::platform::platform_data::*;

pub struct PluginDisplay;

impl Plugin for PluginDisplay {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_display);
        app.add_systems(First, first_update_display);
        app.add_systems(Last, last_update_display);
    }
}

fn setup_display(world: &mut World){
    let mut display = Display::new(DISPLAY_WIDTH, DISPLAY_HEIGHT, BYTES_PER_PIXEL);
    display.clean();
    world.insert_non_send_resource(display);
}

fn first_update_display(
    mut display: NonSendMut<Display>
){
    display.clean();
}

fn last_update_display(
    mut display: NonSendMut<Display>
){
    display.update();
}