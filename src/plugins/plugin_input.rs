use bevy::prelude::*;
#[cfg(any(windows, target_os = "macos"))]
use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct PluginInput;

impl Plugin for PluginInput {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_input);
    }
}

fn setup_input(world: &mut World){
    let device_state = DeviceState::new();
    world.insert_non_send_resource(device_state);
}