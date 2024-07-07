use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore};

use crate::resource::resource_global::FpsInfo;

pub struct FpsCheck;

impl Plugin for FpsCheck {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Update, print_fps.run_if( common_conditions::on_timer(Duration::from_secs(5))));
    }
}

fn print_fps(
    diagnostics: Res<DiagnosticsStore>, 
    mut fps_info: ResMut<FpsInfo>,
) {
    fps_info.0 = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        .map(|x|(x.value().unwrap_or_default(),x.average().unwrap_or_default(),x.smoothed().unwrap_or_default()))
        .unwrap_or_default();
}