use std::cmp;

use bevy::prelude::*;
use crate::components::component_node::{NodeInputs, NodeOutputs, NodeTilePosition};
#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(any(windows, target_os = "macos"))]
use crate::core::display_mock::*;
use crate::core::display_style::{LINE_STYLE_TEST, PORT_STYLE_TEST, RECT_STYLE_TEST, TEXT_STYLE_LARGE, TEXT_STYLE_SMALL};
use crate::core::display_trait::DisplayDraw;
use crate::platform::platform_data::{NODE_PORT_DIMENSION, NODE_PORT_TEXT_SHIFTING, TILE_MAP_DIMENSION, TILE_MAP_MARGIN, TILE_NODE_DIMENSION};
use crate::platform::platform_data::*;
use crate::resource::resource_coordinate::{GlobalPointerPosition, GlobalScaleFactor};
use crate::{NODE_OUTLINE_SYTLE, NODE_PORT_STYLE, NODE_TEXT_STYLE};


#[derive(Bundle)]
struct NodeAdd {
    node_id: NodeId,
    node_name: NodeName,
    position: Position,
    port_input: PortInput,
    port_output: PortOutput,
}

#[derive(Component)]
pub struct NodeId (pub u32);

#[derive(Component)]
pub struct NodeName (pub String);

#[derive(Component)]
pub struct Position (pub (i32,i32));

#[derive(Component)]
pub struct PortInput (pub Vec<(f32)>);

#[derive(Component)]
pub struct PortOutput (pub Vec<(f32)>);


pub fn spawn_node_add (
    mut commands: Commands,
){
    commands.spawn(NodeAdd{
        node_id: NodeId(1),
        node_name: NodeName(String::from("ADD")),
        position: Position((0,0)),
        port_input: PortInput(vec![(0.0), (0.0)]),
        port_output: PortOutput(vec![(0.0)]),
    });

    commands.spawn(NodeAdd{
        node_id: NodeId(1),
        node_name: NodeName(String::from("ADD")),
        position: Position((50,10)),
        port_input: PortInput(vec![(0.0), (0.0)]),
        port_output: PortOutput(vec![(0.0)]),
    });
}


pub fn draw_node(
    query_node: Query<(&NodeName, &Position, &PortInput, &PortOutput)>,
    pointer_position: Res<GlobalPointerPosition>,
    scale_factor: Res<GlobalScaleFactor>,
    mut display: NonSendMut<Display>,
){
    let center_x = DISPLAY_WIDTH as i32 / 2;
    let center_y = DISPLAY_HEIGHT as i32 / 2;

    for (node_name, position, port_input, port_output) in query_node.iter() {
        // 计算缩放前的屏幕坐标
        let original_x = center_x - pointer_position.0.0 + position.0.0;
        let original_y = center_y + pointer_position.0.1 - position.0.1;

        // 应用缩放因子
        let scaled_x = center_x + (original_x - center_x) * scale_factor.0;
        let scaled_y = center_y + (original_y - center_y) * scale_factor.0;

        let node_width = 30;
        let port_spacing = 10;
        let port_diameter = 2;

        let input_node_number = port_input.0.len();
        let output_node_number = port_output.0.len();
        let height_count = cmp::max(input_node_number, output_node_number);

        let node_height = height_count as i32 * port_spacing;

        // 计算缩放后的大小
        let scaled_width = node_width * scale_factor.0 as i32;
        let scaled_height = node_height * scale_factor.0 as i32;
        let half_scaled_width = scaled_width / 2;
        let half_scaled_height = scaled_height / 2;
        let half_port_diameter = port_diameter * scale_factor.0 / 2;

        // 端口位置的基础计算
        let base_port_y = scaled_y - half_scaled_height + port_spacing / 2 * scale_factor.0;

        // 绘制端口
        for (i, _port) in port_input.0.iter().enumerate() {
            display.draw_circle(
                scaled_x - half_scaled_width - half_port_diameter, 
                base_port_y + i as i32 * port_spacing * scale_factor.0 - half_port_diameter,
                (port_diameter * scale_factor.0) as u32, 
                NODE_PORT_STYLE);
        }

        for (i, _port) in port_output.0.iter().enumerate() {
            display.draw_circle(
                scaled_x + half_scaled_width - half_port_diameter, 
                base_port_y + i as i32 * port_spacing * scale_factor.0 - half_port_diameter,
                (port_diameter * scale_factor.0) as u32, 
                NODE_PORT_STYLE);
        }

        // 绘制节点主体
        display.draw_rectangle_round(
            scaled_x - half_scaled_width,
            scaled_y - half_scaled_height,
            scaled_width as u32,
            scaled_height as u32,
            2 * scale_factor.0 as u32,
            NODE_OUTLINE_SYTLE
        );

        // 绘制节点名称
        display.draw_text_center(&node_name.0, scaled_x, scaled_y, NODE_TEXT_STYLE);
    }
}


