mod core;
mod resource;
mod plugins;
mod platform;
mod systems;
mod components;

use std::collections::BTreeMap;
#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(any(windows, target_os = "macos"))]
use crate::core::display_mock::*;
use crate::core::display_trait::*;
use crate::core::display_style::*;
use crate::resource::resource_image::*;

use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::{app::ScheduleRunnerPlugin, prelude::*, utils::Duration};
use plugins::plugin_input::PluginInput;
use resource::resource_coordinate::GlobalPointerPosition;
use resource::resource_coordinate::GlobalScaleFactor;
use systems::system_axis::draw_focus_point;
use systems::system_axis::draw_global_axis;
use systems::system_axis::update_pointer_coordinate;
use systems::system_debug;
use systems::system_debug::debug_info;
use systems::system_interaction::move_focus_point;
use systems::system_interaction::zoom_canvas;
use systems::system_node::*;
use crate::plugins::fps_check::FpsCheck;
use crate::plugins::plugin_display::PluginDisplay;
use crate::resource::resource_node_priority::{NodePriorityList, print_node_priority_list};
use crate::systems::system_tilemap::draw_tile_map;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 30.0,))),
        )
        .add_plugins((PluginDisplay, FpsCheck))
        .add_plugins(PluginInput)
        .insert_resource(GlobalPointerPosition((0,0)))
        .insert_resource(GlobalScaleFactor(4))
        // .insert_resource(NodePriorityList(BTreeMap::new()))
        // .add_systems(Startup, spawn_test_node)
        .add_systems(Startup, (spawn_node_add, spwan_connection))
        // .add_systems(Update, (draw_tile_map, update_test_node, draw_test_node).chain())
        .add_systems(Update, (draw_global_axis, draw_connection, draw_node, draw_focus_point, zoom_canvas).chain())
        .add_systems(Update, move_focus_point)
        .add_systems(PostUpdate, debug_info)
        .run();
}

