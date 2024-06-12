use bevy::prelude::*;
use std::collections::BTreeMap;

#[derive(Resource)]
pub struct NodePriorityList(pub BTreeMap<i32,Vec<Entity>>);

pub fn print_node_priority_list(list: Res<NodePriorityList>) {
    for (x, entities) in list.0.iter() {
        println!("x = {}: {:?}", x, entities);
    }
}