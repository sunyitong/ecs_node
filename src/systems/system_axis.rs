use bevy::prelude::*;
#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(any(windows, target_os = "macos"))]
use crate::core::display_mock::*;
use crate::core::display_style::*;
use crate::core::display_trait::DisplayDraw;
use crate::platform::platform_data::*;
use crate::resource::resource_coordinate::{GlobalPointerPosition, GlobalScaleFactor};

pub fn draw_global_axis(
    mut display: NonSendMut<Display>,
    pointer_position: Res<GlobalPointerPosition>,
    scale_factor: Res<GlobalScaleFactor>
){
    let center_x = DISPLAY_WIDTH as i32 / 2;
    let center_y = DISPLAY_HEIGHT as i32 / 2;

    // 计算缩放前的屏幕坐标
    let original_x = center_x - pointer_position.0.0;
    let original_y = center_y + pointer_position.0.1;

    // 应用缩放因子
    let scaled_x = center_x + (original_x - center_x) * scale_factor.0 as i32;
    let scaled_y = center_y + (original_y - center_y) * scale_factor.0 as i32;

    // 绘制水平坐标轴
    display.draw_line(
        0,
        scaled_y,
        DISPLAY_WIDTH as i32,
        scaled_y,
        GLOBAL_AXIS_X
    );

    // 绘制垂直坐标轴
    display.draw_line(
        scaled_x,
        0,
        scaled_x,
        DISPLAY_HEIGHT as i32,
        GLOBAL_AXIS_Y
    );
}


pub fn draw_focus_point(
    mut display: NonSendMut<Display>,
){
    display.draw_circle(DISPLAY_WIDTH as i32 /2-5, DISPLAY_HEIGHT as i32 /2-5, 11, FOCUS_POINT)
}

pub fn update_pointer_coordinate(
    mut pointer_position: ResMut<GlobalPointerPosition>,
    mut scale: ResMut<GlobalScaleFactor>,
    mut counter: Local<i32>,
    mut quadrant: Local<u8>,
) {
    // 根据当前的象限设置x和y的坐标
    match *quadrant {
        1 => { // 第一象限：x和y都增加
            pointer_position.0.0 = *counter;
            pointer_position.0.1 = *counter;
        },
        2 => { // 第二象限：x减少，y增加
            pointer_position.0.0 = -*counter;
            pointer_position.0.1 = *counter;
        },
        3 => { // 第三象限：x和y都减少
            pointer_position.0.0 = -*counter;
            pointer_position.0.1 = -*counter;
        },
        4 => { // 第四象限：x增加，y减少
            pointer_position.0.0 = *counter;
            pointer_position.0.1 = -*counter;
        },
        _ => {
            pointer_position.0.0 = *counter;
            pointer_position.0.1 = *counter;
        }, // 处理意外的象限值
    }

    // 更新counter，用于控制指针移动的速度和方向
    if *counter == 200 {
        *counter = 0;
        *quadrant += 1; // 移动到下一个象限
        if *quadrant > 4 {
            *quadrant = 1; // 重置象限到第一象限
        }
    }

    *counter += 1;

    if *counter >= 50{
        scale.0 = 3;
    } else {
        scale.0 = 1;
    }
}
