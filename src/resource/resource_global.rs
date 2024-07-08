use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalPointerPosition (pub (i32, i32));

#[derive(Resource)]
pub struct GlobalScaleFactor (pub i32);

#[derive(Resource, Default, Debug)]
pub enum FocusPointStatus {
    #[default]
    OnCanvas,
    OnNode(u32),
    OnInputPort(u32, u32),
    OnOutputPort(u32, u32),
}

#[derive(Resource, Debug)]
pub struct IsFocusPointLocked (pub bool);

// debug
#[derive(Resource, Default)]
pub struct FpsInfo (pub (f64, f64, f64));
