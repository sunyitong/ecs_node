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
    OnInputPort(u32, u32, (i32, i32)),
    OnOutputPort(u32, u32, (i32, i32)),
}

#[derive(Resource, Debug)]
pub struct IsFocusPointLocked (pub bool);

#[derive(Resource, Debug)]
pub struct IsTempConnectionSetting (pub bool);

#[derive(Resource, Default, Debug)]
pub struct TempConnection{
    pub is_input_port_set: bool,
    pub is_output_port_set: bool,
    pub input_port: (u32, u32, (i32, i32)),
    pub output_port: (u32, u32, (i32, i32)),
}


// debug
#[derive(Resource, Default)]
pub struct FpsInfo (pub (f64, f64, f64));
