mod core;
mod resource;
mod plugins;
mod platform;
mod systems;

use bevy::{app::ScheduleRunnerPlugin, prelude::*, utils::Duration};
use resource::resource_global::*;
use systems::system_axis::*;
use systems::system_debug::*;
use systems::system_interaction::*;
use systems::system_node::*;
use plugins::fps_check::FpsCheck;
use plugins::plugin_display::PluginDisplay;
use plugins::plugin_input::PluginInput;

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

