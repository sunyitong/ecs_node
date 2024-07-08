use bevy::prelude::*;
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::systems:: system_node::{NodeId, Position};
use crate::resource::resource_global::*;

pub fn move_focus_point(
    mut focus_point: ResMut<GlobalPointerPosition>,
    input_device: NonSend<DeviceState>,
    focus_point_status: Res<FocusPointStatus>,
    mut is_focus_point_locked: ResMut<IsFocusPointLocked>,
    mut query_node: Query<(&NodeId, &mut Position)>,
) {
    let move_distance = 1;
    let keys: Vec<Keycode> = input_device.get_keys();
    let mut move_x = 0;
    let mut move_y = 0;

    // 处理指针移动
    if keys.contains(&Keycode::W) {
        move_y += move_distance;
    }
    if keys.contains(&Keycode::S) {
        move_y -= move_distance;
    }
    if keys.contains(&Keycode::A) {
        move_x -= move_distance;
    }
    if keys.contains(&Keycode::D) {
        move_x += move_distance;
    }

    focus_point.0.0 += move_x;
    focus_point.0.1 += move_y;

    // 处理节点移动
    if keys.contains(&Keycode::G) {
        if let FocusPointStatus::OnNode(node_id) = *focus_point_status{
            for (id, mut position) in query_node.iter_mut() {
                if node_id == id.0 {
                    position.0.0 += move_x;
                    position.0.1 += move_y;
                    is_focus_point_locked.0 = true;
                }
            }
        }
    } else {
        is_focus_point_locked.0 = false;
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