use bevy_ecs::prelude::World;
use std::error::Error;

pub mod file_simulation_loader;

pub trait SimulationState {
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;
}

pub struct SimulationStateImpl {
    pub world: World,
}

impl SimulationState for SimulationStateImpl {
    fn world(&self) -> &World {
        &self.world
    }

    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}

pub trait LoadSimulation<T: SimulationState, E: Error> {
    fn load(&self) -> Result<T, E>;
}

pub trait SaveSimulation<T: SimulationState> {
    fn save(&self, simulation: T);
}
