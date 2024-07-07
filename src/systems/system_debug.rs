use bevy::prelude::*;
#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(any(windows, target_os = "macos"))]
use crate::core::display_mock::*;
use crate::core::display_style::*;
use crate::core::display_trait::DisplayDraw;
use crate::resource::resource_global::*;


pub fn debug_info (
    mut display: NonSendMut<Display>,
    pointer_postion: Res<GlobalPointerPosition>,
    focus_point_status: Res<FocusPointStatus>,
    fps_info: Res<FpsInfo>,
){
    let focus_point_text = format!("Focus Point: ({}, {})" , pointer_postion.0.0, pointer_postion.0.1); 
    display.draw_text(&focus_point_text, 10, 15, TEXT_STYLE_SMALL);

    let focus_point_status_text = format!("Focus Point Status: {:?}" , focus_point_status); 
    display.draw_text(&focus_point_status_text, 10, 30, TEXT_STYLE_SMALL);

    let fps_info_text = format!("FPS: {:.0}, AVG: {:.0}, SMOOTHED: {:.0}" , fps_info.0.0, fps_info.0.1, fps_info.0.2);
    display.draw_text(&fps_info_text, 10, 45, TEXT_STYLE_SMALL);
}