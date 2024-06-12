use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore};

pub struct FpsCheck;

impl Plugin for FpsCheck {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Update, print_fps.run_if( common_conditions::on_timer(Duration::from_secs(5))));
    }
}

fn print_fps( diagnostics: Res<DiagnosticsStore>, mut c:Local<u32> ) {
    let (fps,avg,smoothed) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        .map(|x|(x.value().unwrap_or_default(),x.average().unwrap_or_default(),x.smoothed().unwrap_or_default()))
        .unwrap_or_default();

    println!("{}: {fps:.0} {avg:.0} {smoothed:.0}",*c);
    *c+=1;
}