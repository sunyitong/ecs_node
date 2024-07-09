use bevy::prelude::*;
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::plugins::plugin_input::KeyState;
use crate::systems:: system_node::{NodeId, Position};
use crate::resource::resource_global::*;

pub fn move_focus_point(
    mut focus_point: ResMut<GlobalPointerPosition>,
    key_state: Res<KeyState>,
    focus_point_status: Res<FocusPointStatus>,
    mut is_focus_point_locked: ResMut<IsFocusPointLocked>,
    mut temp_connection: ResMut<TempConnection>,
    mut query_node: Query<(&NodeId, &mut Position)>,
) {
    let move_distance = 1;
    let mut move_x = 0;
    let mut move_y = 0;

    // 处理指针移动
    if key_state.is_key_pressed(Keycode::W) {
        move_y += move_distance;
    }
    if key_state.is_key_pressed(Keycode::S) {
        move_y -= move_distance;
    }
    if key_state.is_key_pressed(Keycode::A) {
        move_x -= move_distance;
    }
    if key_state.is_key_pressed(Keycode::D) {
        move_x += move_distance;
    }

    focus_point.0.0 += move_x;
    focus_point.0.1 += move_y;

    // 处理节点移动
    if key_state.is_key_pressed(Keycode::G) {
        match *focus_point_status {
            FocusPointStatus::OnNode(node_id) => {
                for (id, mut position) in query_node.iter_mut() {
                    if node_id == id.0 {
                        position.0.0 += move_x;
                        position.0.1 += move_y;
                        is_focus_point_locked.0 = true;
                    }
                }
            }
            _ => {}
        }
    } else if key_state.is_key_just_released(Keycode::G) {
        match *focus_point_status {

            FocusPointStatus::OnOutputPort(node_id, output_port_index, (port_x, port_y)) => {
                if !temp_connection.is_output_port_set {
                    temp_connection.output_port = (node_id, output_port_index, (port_x, port_y));
                    temp_connection.is_output_port_set = true;
                }
            }

            FocusPointStatus::OnInputPort(node_id, input_port_index, (port_x, port_y)) => {
                if !temp_connection.is_input_port_set {
                    temp_connection.input_port = (node_id, input_port_index, (port_x, port_y));
                    temp_connection.is_input_port_set = true;
                }
            }

            _ => {}
        }
    } else {
        is_focus_point_locked.0 = false;
    }

    println!("{:?}", temp_connection);
}


pub fn zoom_canvas (
    mut scale_factor: ResMut<GlobalScaleFactor>,
    key_state: Res<KeyState>,
){
    if key_state.is_key_pressed(Keycode::R) {
        scale_factor.0 += 1;
        if scale_factor.0 > 8{
            scale_factor.0 = 8;
        }
    } else if key_state.is_key_pressed(Keycode::F) {
        scale_factor.0 -= 1;
        if scale_factor.0 < 1{
            scale_factor.0 = 1;
        }
    }
}