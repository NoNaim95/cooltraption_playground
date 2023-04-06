use bevy_ecs::system::Resource;

use crate::components::Position;
use crate::stages::physics_stage::Float;
use crate::Tick;

pub enum ActionRequest {
    SpawnBall { requested_position: (Float, Float) },
}

#[derive(Resource, Clone)]
pub struct ActionPacket {
    pub tick: Tick,
    pub action: Action,
}

impl ActionPacket {
    pub fn new(tick: Tick, action: Action) -> Self {
        Self { tick, action }
    }
}

#[derive(Resource, Clone, Debug)]
pub enum Action {
    SpawnBall { pos: Position },
}
