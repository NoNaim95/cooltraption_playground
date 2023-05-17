use bevy_ecs::system::Resource;


use crate::components::Position;
use crate::system_sets::physics_set::Float;
use crate::Tick;

use serde::{Serialize, Deserialize};


#[derive(Clone, Copy)]
pub enum ActionRequest {
    SpawnBall { requested_position: (Float, Float) },
}

#[derive(Debug, Resource, Clone, Copy, Serialize, Deserialize)]
pub struct ActionPacket {
    pub tick: Tick,
    pub action: Action,
}

impl ActionPacket {
    pub fn new(tick: Tick, action: Action) -> Self {
        Self { tick, action }
    }
}

#[derive(Resource, Clone, Debug, Copy, Serialize, Deserialize)]
pub enum Action {
    SpawnBall(SpawnBallAction),
    OutwardForce(OutwardForceAction),
    CircularForce(CircularForceAction),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpawnBallAction {
    pub position: Position,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CircularForceAction {
    pub position: Position,
    pub strength: Float,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OutwardForceAction {
    pub position: Position,
    pub strength: Float,
}

