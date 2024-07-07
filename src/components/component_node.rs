use bevy::prelude::*;

#[derive(Component)]
pub struct NodeTilePosition(pub (i32,i32));

#[derive(Component)]
pub struct NodeInputs(pub Vec<f32>);

#[derive(Component)]
pub struct NodeOutputs(pub Vec<f32>);