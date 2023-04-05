use crate::action::Action;
use bevy_ecs::prelude::World;

#[derive(Default)]
pub struct SimulationState {
    pub world: World,
    pub actions: Vec<Action>,
}

impl SimulationState {
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
