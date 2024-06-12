use bevy::prelude::*;
#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(windows)]
use crate::core::display_mock::*;
use crate::core::display_style::TILE_MAP_STYLE_TEST;
use crate::core::display_trait::DisplayDraw;
use crate::platform::platform_data::TILE_MAP_DIMENSION;

pub fn draw_tile_map(
    mut display: NonSendMut<Display>
){
    let mut count = 0;
    for i in (0..=480).step_by(TILE_MAP_DIMENSION as usize) {
        display.draw_line(0,i,480,i,TILE_MAP_STYLE_TEST);
        display.draw_line(i,0,i,480,TILE_MAP_STYLE_TEST);
    }
}