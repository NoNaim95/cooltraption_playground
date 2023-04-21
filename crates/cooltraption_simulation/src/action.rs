use bevy_ecs::system::Resource;

use crate::components::Position;
use crate::stages::physics_stage::Float;
use crate::Tick;


#[derive(Clone, Copy)]
pub enum ActionRequest {
    SpawnBall { requested_position: (Float, Float) },
}

#[derive(Resource, Clone, Copy)]
pub struct ActionPacket {
    pub tick: Tick,
    pub action: Action,
}

impl ActionPacket {
    pub fn new(tick: Tick, action: Action) -> Self {
        Self { tick, action }
    }
}

#[derive(Resource, Clone, Debug, Copy)]
pub enum Action {
    SpawnBall(SpawnBallAction),
    OutwardForce(OutwardForceAction),
}

#[derive(Debug, Clone, Copy)]
pub struct SpawnBallAction {
    pub position: Position,
}

#[derive(Debug, Clone, Copy)]
pub struct OutwardForceAction {
    pub position: Position,
    pub strength: Float,
}

