mod core;
mod resource;
mod plugins;
mod platform;
mod systems;
mod components;

use std::collections::BTreeMap;
#[cfg(all(target_os = "linux", target_arch = "arm"))]
use crate::core::display_arm::Display;
#[cfg(windows)]
use crate::core::display_mock::*;
use crate::core::display_trait::*;
use crate::core::display_style::*;
use crate::resource::resource_image::*;

use bevy::{app::ScheduleRunnerPlugin, prelude::*, utils::Duration};
use crate::plugins::fps_check::FpsCheck;
use crate::plugins::plugin_display::PluginDisplay;
use crate::resource::resource_node_priority::{NodePriorityList, print_node_priority_list};
use crate::systems::system_node::{draw_test_node, spawn_test_node, update_test_node};
use crate::systems::system_tilemap::draw_tile_map;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 30.0,
            ))),
        )
        .add_plugins((PluginDisplay, FpsCheck))
        .insert_resource(NodePriorityList(BTreeMap::new()))
        .add_systems(Startup, spawn_test_node)
        .add_systems(Update, (draw_tile_map, update_test_node, draw_test_node).chain())
        .run();
}
