use bevy::prelude::*;
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::resource::resource_global::{GlobalPointerPosition, GlobalScaleFactor};

pub fn move_focus_point (
    mut focus_point: ResMut<GlobalPointerPosition>,
    input_device: NonSend<DeviceState>,
){

    let move_distance = 1;
    let keys: Vec<Keycode> = input_device.get_keys();
    if keys.contains(&Keycode::W) {
        focus_point.0.1 += move_distance;
    } else if keys.contains(&Keycode::S) {
        focus_point.0.1 -= move_distance;
    } else if keys.contains(&Keycode::A) {
        focus_point.0.0 -= move_distance;
    } else if keys.contains(&Keycode::D) {
        focus_point.0.0 += move_distance;
    }
}

pub fn zoom_canvas (
    mut scale_factor: ResMut<GlobalScaleFactor>,
    input_device: NonSend<DeviceState>,
){
    let keys: Vec<Keycode> = input_device.get_keys();
    if keys.contains(&Keycode::R) {
        scale_factor.0 += 1;
        if scale_factor.0 > 8{
            scale_factor.0 = 8;
        }
    } else if keys.contains(&Keycode::F) {
        scale_factor.0 -= 1;
        if scale_factor.0 < 1{
            scale_factor.0 = 1;
        }
    }



}