use crate::action::Action;
use bevy_ecs::prelude::World;

pub trait SimulationState: Sync {
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;
}

#[derive(Default)]
pub struct SimulationStateImpl {
    pub world: World,
    pub actions: Vec<Action>,
}

impl SimulationState for SimulationStateImpl {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}
