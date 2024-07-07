use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalPointerPosition (pub (i32, i32));

#[derive(Resource)]
pub struct GlobalScaleFactor (pub i32);
