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
        .insert_resource(FocusPointStatus::default())
        .insert_resource(IsFocusPointLocked(false))
        .insert_resource(IsTempConnectionSetting(false))
        .insert_resource(TempConnection::default())
        .insert_resource(FpsInfo::default())
        .add_systems(Startup, (spawn_node_add))
        .add_systems(Update, spwan_connection)
        .add_systems(Update, (move_focus_point, zoom_canvas))
        // .add_systems(Update, update_node)
        .add_systems(Update, (draw_global_axis, draw_node, draw_temp_connection, draw_connection, draw_focus_point).chain())
        .add_systems(PostUpdate, debug_info)
        .run();
}
