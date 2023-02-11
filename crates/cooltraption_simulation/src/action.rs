use bevy_ecs::system::Resource;

use crate::components::Position;
use crate::stages::physics_stage::Float;

pub enum ActionRequest {
    SpawnBall { requested_position: (Float, Float) },
}

#[derive(Resource, Clone)]
pub struct ActionPacket {
    pub tick_no: u64,
    pub action: Action,
}

#[derive(Resource, Clone)]
pub enum Action {
    SpawnBall { pos: Position },
}
