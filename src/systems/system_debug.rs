use bevy::prelude::*;
#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(any(windows, target_os = "macos"))]
use crate::core::display_mock::*;
use crate::core::display_style::*;
use crate::core::display_trait::DisplayDraw;
use crate::resource::resource_global::GlobalPointerPosition;


pub fn debug_info (
    mut display: NonSendMut<Display>,
    pointer_postion: Res<GlobalPointerPosition>
){
    let focus_point_text = format!("Focus Point: ({}, {})" , pointer_postion.0.0, pointer_postion.0.1); 
    display.draw_text(&focus_point_text, 10, 10, TEXT_STYLE_SMALL);
}