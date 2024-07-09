use bevy::prelude::*;
#[cfg(any(windows, target_os = "macos"))]
use device_query::{DeviceQuery, DeviceState, Keycode};

use std::collections::HashSet;

pub struct PluginInput;

impl Plugin for PluginInput {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_input);
        app.add_systems(First, update_input);
        // app.add_systems(Last, debug_input);
    }
}

fn setup_input(world: &mut World){
    let device_state = DeviceState::new();
    world.insert_non_send_resource(device_state);
    world.insert_resource(KeyState::default());
}

fn update_input(
    input_device: NonSend<DeviceState>,
    mut key_state: ResMut<KeyState>,
){
    let new_keys: HashSet<Keycode> = input_device.get_keys().iter().cloned().collect();

    key_state.just_pressed = new_keys.difference(&key_state.currently_pressed).cloned().collect();
    key_state.just_released = key_state.currently_pressed.difference(&new_keys).cloned().collect();
    key_state.currently_pressed = new_keys;
}

fn debug_input (
    key_state: Res<KeyState>,
){
    println!("{:?}", key_state);
}


#[derive(Resource, Default, Debug)]
pub struct KeyState {
    pub just_pressed: HashSet<Keycode>,
    pub currently_pressed: HashSet<Keycode>,
    pub just_released: HashSet<Keycode>,
}

impl KeyState {
    pub fn is_key_just_pressed(&self, key: Keycode) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.currently_pressed.contains(&key)
    }

    pub fn is_key_just_released(&self, key: Keycode) -> bool {
        self.just_released.contains(&key)
    }
}

