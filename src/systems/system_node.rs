use std::cmp;
use bevy::prelude::*;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(any(windows, target_os = "macos"))]
use crate::core::display_mock::*;

use crate::core::display_style::*;
use crate::core::display_trait::DisplayDraw;
use crate::platform::platform_data::*;
use crate::resource::resource_global::{GlobalPointerPosition, GlobalScaleFactor};
use crate::{FocusPointStatus, IsFocusPointLocked, IsTempConnectionSetting, TempConnection};

pub enum NodeType{
    Add,
}


#[derive(Bundle)]
struct NodeAdd {
    node_type: Type,
    node_priority: NodePriority,
    node_name: NodeName,
    position: Position,
    port_input: PortInput,
    port_output: PortOutput,
}

#[derive(Component)]
pub struct NodeId (pub Entity);

#[derive(Component)]
pub struct Type (pub NodeType);

#[derive(Component)]
pub struct NodePriority (pub u32);

#[derive(Component)]
pub struct NodeName (pub String);

#[derive(Component)]
pub struct Position (pub (i32,i32));

#[derive(Component)]
pub struct PortInput (pub Vec<f32>);

#[derive(Component)]
pub struct PortOutput (pub Vec<f32>);


#[derive(Bundle)]
struct PortConnection {
    connection_id: ConnectionId,
    connection: Connection,
    connection_coordinates: ConnectionCoordinates,
}


#[derive(Component)]
pub struct ConnectionId (pub u32);

#[derive(Component)]
pub struct Connection {
   pub from_node: Entity,
   pub to_node: Entity,
   pub from_output_port: u32,
   pub to_input_port: u32,
}

#[derive(Component)]
pub struct ConnectionCoordinates {
    pub start_coord: (i32, i32),
    pub end_coord: (i32, i32),
}


pub fn spawn_node_add (
    mut commands: Commands,
){
    let entity_0 = commands.spawn(NodeAdd{
        node_type: Type(NodeType::Add),
        node_priority: NodePriority(1),
        node_name: NodeName(String::from("ADD")),
        position: Position((0,0)),
        port_input: PortInput(vec![(0.0), (0.0)]),
        port_output: PortOutput(vec![(0.0)]),
    }).id();

    commands.entity(entity_0).insert(NodeId(entity_0));


    let entity_1 = commands.spawn(NodeAdd{
        node_type: Type(NodeType::Add),
        node_priority: NodePriority(2),
        node_name: NodeName(String::from("ADD")),
        position: Position((50,10)),
        port_input: PortInput(vec![(0.0), (0.0)]),
        port_output: PortOutput(vec![(0.0)]),
    }).id();

    commands.entity(entity_1).insert(NodeId(entity_1));

    let entity_2 = commands.spawn(NodeAdd{
        node_type: Type(NodeType::Add),
        node_priority: NodePriority(3),
        node_name: NodeName(String::from("ADD")),
        position: Position((-50,25)),
        port_input: PortInput(vec![(0.0), (0.0)]),
        port_output: PortOutput(vec![(0.0)]),
    }).id();

    commands.entity(entity_2).insert(NodeId(entity_2));

}

pub fn spwan_connection (
    mut commands: Commands,
    mut temp_connection: ResMut<TempConnection>,
    mut is_temp_connection_setting: ResMut<IsTempConnectionSetting>,
){
    // 确保两个端口的实体都已设置并且实体是有效的
    if temp_connection.is_output_port_set && temp_connection.is_input_port_set {
        if let (Some(from_node), Some(to_node)) = (temp_connection.output_port.0, temp_connection.input_port.0) {
            commands.spawn(PortConnection {
                connection_id: ConnectionId(1),
                connection: Connection {
                    from_node: from_node,
                    to_node: to_node,
                    from_output_port: temp_connection.output_port.1,
                    to_input_port: temp_connection.input_port.1
                },
                connection_coordinates: ConnectionCoordinates {
                    start_coord: temp_connection.output_port.2,
                    end_coord: temp_connection.input_port.2
                },
            });

            // 重置临时连接状态
            temp_connection.is_output_port_set = false;
            temp_connection.is_input_port_set = false;
            temp_connection.input_port = (None,0,(0,0));
            temp_connection.output_port = (None,0,(0,0));
            is_temp_connection_setting.0 = false;
        }
    }
}


pub fn draw_node(
    query_node: Query<(&NodeId, &NodeName, &Position, &PortInput, &PortOutput)>,
    pointer_position: Res<GlobalPointerPosition>,
    scale_factor: Res<GlobalScaleFactor>,
    is_focus_point_locked: Res<IsFocusPointLocked>,
    is_temp_connection_setting: Res<IsTempConnectionSetting>,
    mut focus_point_status: ResMut<FocusPointStatus>,
    mut display: NonSendMut<Display>,
) {
    let center_x = DISPLAY_WIDTH as i32 / 2;
    let center_y = DISPLAY_HEIGHT as i32 / 2;
    let node_width = 30;
    let port_spacing = 10;
    let port_diameter = 4;
    let node_half_width = node_width / 2;
    let port_half_diameter = port_diameter / 2;
    let mut is_focus_point_checked = false; 

    for (node_id, node_name, position, port_input, port_output) in query_node.iter() {
        let node_input_count = port_input.0.len();
        let node_output_count = port_output.0.len();
        let node_height = cmp::max(node_input_count, node_output_count) as i32 * port_spacing;
        let node_half_height = node_height / 2;
        let half_scaled_port_diameter = port_diameter * scale_factor.0 / 2;

        // 缩放前的屏幕坐标
        let original_x = center_x - pointer_position.0.0 + position.0.0;
        let original_y = center_y + pointer_position.0.1 - position.0.1;
        // 应用缩放因子
        let scaled_x = center_x + (original_x - center_x) * scale_factor.0;
        let scaled_y = center_y + (original_y - center_y) * scale_factor.0;

        let scaled_width = node_width * scale_factor.0 as i32;
        let scaled_height = node_height * scale_factor.0 as i32;
        let half_scaled_width = scaled_width / 2;
        let half_scaled_height = scaled_height / 2;

        // 节点边界和端口基础位置
        let base_port_y = scaled_y - half_scaled_height + port_spacing / 2 * scale_factor.0;
        let orig_node_left_x = position.0.0 - node_half_width;
        let orig_node_right_x = position.0.0 + node_half_width;

        if !is_focus_point_checked && !is_focus_point_locked.0 && !is_temp_connection_setting.0 {
            if pointer_position.0.0 > orig_node_left_x && pointer_position.0.0 < orig_node_right_x &&
            pointer_position.0.1 > position.0.1 - node_half_height && pointer_position.0.1 < position.0.1 + node_half_height {
                *focus_point_status = FocusPointStatus::OnNode(node_id.0);
                is_focus_point_checked = true;
            }
        }

        // 端口基础位置（原始坐标）
        let orig_first_port_upper_y = position.0.1 + node_half_height - port_spacing / 2 + port_half_diameter;

        // 绘制端口并检查聚焦点
        for (i, _) in port_input.0.iter().enumerate() {
            let port_upper_y = orig_first_port_upper_y - i as i32 * port_spacing;

            if !is_focus_point_checked && !is_focus_point_locked.0 {
                if pointer_position.0.0 > orig_node_left_x - port_diameter && pointer_position.0.0 < orig_node_left_x &&
                pointer_position.0.1 < port_upper_y && pointer_position.0.1 > port_upper_y - port_diameter {
                    *focus_point_status = FocusPointStatus::OnInputPort(node_id.0, i as u32, (orig_node_left_x,port_upper_y-port_half_diameter));
                    is_focus_point_checked = true;
                }
            }

            // 绘制端口（动态坐标）
            display.draw_circle(
                scaled_x - half_scaled_width - half_scaled_port_diameter, 
                base_port_y + i as i32 * port_spacing * scale_factor.0 - half_scaled_port_diameter,
                (port_diameter * scale_factor.0) as u32, 
                NODE_PORT_STYLE);
        }

        for (i, _) in port_output.0.iter().enumerate() {
            let port_upper_y = orig_first_port_upper_y - i as i32 * port_spacing;

            if !is_focus_point_checked && !is_focus_point_locked.0 {
                if pointer_position.0.0 > orig_node_right_x && pointer_position.0.0 < orig_node_right_x + port_diameter &&
                pointer_position.0.1 < port_upper_y && pointer_position.0.1 > port_upper_y - port_diameter {
                    *focus_point_status = FocusPointStatus::OnOutputPort(node_id.0, i as u32,(orig_node_right_x,port_upper_y-port_half_diameter));
                    is_focus_point_checked = true;
                }
            }
            // 绘制端口（动态坐标）
            display.draw_circle(
                scaled_x + half_scaled_width - half_scaled_port_diameter, 
                base_port_y + i as i32 * port_spacing * scale_factor.0 - half_scaled_port_diameter,
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
            NODE_OUTLINE_SYTLE,
        );

        // 绘制节点名称
        display.draw_text_center(&node_name.0, scaled_x, scaled_y, NODE_TEXT_STYLE);
    }

    if !is_focus_point_checked && !is_focus_point_locked.0 {
        *focus_point_status = FocusPointStatus::OnCanvas;
    }

}


pub fn draw_connection (
    mut display: NonSendMut<Display>,
    pointer_position: Res<GlobalPointerPosition>,
    scale_factor: Res<GlobalScaleFactor>,
    query_connection: Query<&ConnectionCoordinates>,
){
    let center_x = DISPLAY_WIDTH as i32 / 2;
    let center_y = DISPLAY_HEIGHT as i32 / 2;

    for connection_coords in query_connection.iter() {
        // 应用缩放因子并计算连接的起点和终点
        let scaled_start_x = center_x + ((connection_coords.start_coord.0 - pointer_position.0.0) * scale_factor.0 as i32);
        let scaled_start_y = center_y - ((connection_coords.start_coord.1 - pointer_position.0.1) * scale_factor.0 as i32);
        let scaled_end_x = center_x + ((connection_coords.end_coord.0 - pointer_position.0.0) * scale_factor.0 as i32);
        let scaled_end_y = center_y - ((connection_coords.end_coord.1 - pointer_position.0.1) * scale_factor.0 as i32);

        // 绘制连接线
        display.draw_line(
            scaled_start_x,
            scaled_start_y,
            scaled_end_x,
            scaled_end_y,
            CONNECTION_LINE_STYLE,
        );
    }
}

pub fn draw_temp_connection (
    temp_connection: Res<TempConnection>,
    mut display: NonSendMut<Display>,
    pointer_position: Res<GlobalPointerPosition>,
    scale_factor: Res<GlobalScaleFactor>,
){
    let center_x = DISPLAY_WIDTH as i32 / 2;
    let center_y = DISPLAY_HEIGHT as i32 / 2;

    if temp_connection.is_input_port_set {
        let scaled_start_x = center_x + ((temp_connection.input_port.2.0 - pointer_position.0.0) * scale_factor.0 as i32);
        let scaled_start_y = center_y - ((temp_connection.input_port.2.1 - pointer_position.0.1) * scale_factor.0 as i32);

        display.draw_line(
            scaled_start_x,
            scaled_start_y,
            center_x,
            center_y,
            CONNECTION_LINE_STYLE,
        );
    } else if temp_connection.is_output_port_set {
        let scaled_start_x = center_x + ((temp_connection.output_port.2.0 - pointer_position.0.0) * scale_factor.0 as i32);
        let scaled_start_y = center_y - ((temp_connection.output_port.2.1 - pointer_position.0.1) * scale_factor.0 as i32);

        display.draw_line(
            scaled_start_x,
            scaled_start_y,
            center_x,
            center_y,
            CONNECTION_LINE_STYLE,
        );
    }
}


// pub fn update_node(
//     mut commands: Commands,
//     mut query_node_priority: Query<(Entity, &Type, &NodePriority, &NodeId, &mut PortInput, &mut PortOutput)>,
//     query_connections: Query<(&Connection, &ConnectionCoordinates)>,
// ) {
//     // 收集所有节点并根据优先级排序
//     let mut nodes: Vec<_> = query_node_priority.iter().collect();
//     nodes.sort_by_key(|&(_, _, priority, _, _, _)| priority.0);

//     // 遍历每个节点
//     // 直接处理查询结果，避免先收集到向量中
//     for (entity, node_type, priority, node_id, mut port_input, mut port_output) in query_node_priority.iter_mut() {
//         let inputs = query_connections.iter().filter(|(connection, _)| connection.to_node == node_id.0).collect::<Vec<_>>();
        
//         for (connection, _) in inputs {
//             if let Ok((_, _, _, _, mut source_port_input, source_port_output)) = query_node_priority.get_mut(connection.from_node) {
//                 port_input.0[connection.to_input_port as usize] = source_port_output.0[connection.from_output_port as usize];
//             }
//         }

//         match node_type.0 {
//             NodeType::Add => {
//                 let sum: f32 = port_input.0.iter().sum();
//                 port_output.0.push(sum);
//                 println!("Node {:?} with priority {:?} computed sum: {:?}", entity, priority.0, sum);
//             }
//         }
//     }
// }
