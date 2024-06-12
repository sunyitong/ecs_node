use bevy::prelude::*;
use crate::components::component_node::{NodeInputs, NodeName, NodeOutputs, NodeTilePosition};
#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(windows)]
use crate::core::display_mock::*;
use crate::core::display_style::{LINE_STYLE_TEST, PORT_STYLE_TEST, RECT_STYLE_TEST, TEXT_STYLE_LARGE, TEXT_STYLE_SMALL};
use crate::core::display_trait::DisplayDraw;
use crate::platform::platform_data::{NODE_PORT_DIMENSION, NODE_PORT_TEXT_SHIFTING, TILE_MAP_DIMENSION, TILE_MAP_MARGIN, TILE_NODE_DIMENSION};
use crate::resource::resource_node_priority::NodePriorityList;

pub fn spawn_test_node(mut commands: Commands, mut node_priority_list: ResMut<NodePriorityList>){
    let position = (3,3);
    let entity_id = commands.spawn((
        NodeName("Test Node_1".to_string()),
        NodeTilePosition((position.0, position.1)),
        NodeInputs(vec![0.123, 0.324, 0.221, 0.232, 0.1, 0.99]),
        NodeOutputs(vec![0.114555, 0.1881]),
    )).id();
    node_priority_list.0.entry(position.0).or_insert_with(Vec::new).push(entity_id);
}

pub fn update_test_node(
    mut query_test_node: Query<(&NodeInputs, &mut NodeOutputs)>,
){
    for (inputs, mut outputs) in query_test_node.iter_mut(){
        let mut temp_value:f32 = 0.0;
        for i in inputs.0.iter() {
            temp_value += i;
        }
        outputs.0[0] = temp_value;
    }
}

pub fn draw_test_node(
    query_test_node: Query<(&NodeName, &NodeTilePosition, &NodeInputs, &NodeOutputs)>,
    mut display: NonSendMut<Display>,
) {
    for (name, position, inputs, outputs) in query_test_node.iter() {
        let node_ref_position_x = position.0.0*TILE_MAP_DIMENSION+TILE_MAP_MARGIN;
        let node_ref_position_y = position.0.1*TILE_MAP_DIMENSION+TILE_MAP_MARGIN;
        let input_spacing = TILE_NODE_DIMENSION / (inputs.0.len() as u32 + 1);
        let output_spacing = TILE_NODE_DIMENSION / (outputs.0.len() as u32 +1);
        display.draw_rectangle(node_ref_position_x, node_ref_position_y, TILE_NODE_DIMENSION, TILE_NODE_DIMENSION, LINE_STYLE_TEST);
        // display.draw_text(&*name.0, node_ref_position_x, node_ref_position_y, TEXT_STYLE_LARGE);
        for (c,v) in inputs.0.iter().enumerate(){
            display.draw_text(&*v.to_string(), node_ref_position_x+NODE_PORT_TEXT_SHIFTING, node_ref_position_y + input_spacing as i32 *(c as i32+1), TEXT_STYLE_SMALL);
            display.draw_circle(node_ref_position_x-2,node_ref_position_y + input_spacing as i32 *(c as i32+1), NODE_PORT_DIMENSION, PORT_STYLE_TEST);
        }
        for (c ,v) in outputs.0.iter().enumerate(){
            display.draw_text_right(&*v.to_string(), node_ref_position_x+TILE_NODE_DIMENSION as i32-NODE_PORT_TEXT_SHIFTING, node_ref_position_y + output_spacing as i32 *(c as i32+1), TEXT_STYLE_SMALL);
            display.draw_circle(node_ref_position_x+TILE_NODE_DIMENSION as i32-2,node_ref_position_y + output_spacing as i32 *(c as i32+1), NODE_PORT_DIMENSION, PORT_STYLE_TEST);
        }
    }
}