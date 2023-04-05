use crate::{action::Action, stages::physics_stage::DeltaTime, Actions, Tick};
use bevy_ecs::prelude::World;

#[derive(Default)]
pub struct SimulationState {
    world: World,
}

impl SimulationState {
    pub fn world(&self) -> &World {
        &self.world
    }
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    pub fn load_current_tick(&mut self, current_tick: Tick) {
        self.world_mut().insert_resource(current_tick);
    }
    pub fn load_delta_time(&mut self, dt: DeltaTime) {
        self.world_mut().insert_resource(DeltaTime::from(dt));
    }

    pub fn load_actions(&mut self, actions: Actions) {
        self.world_mut().insert_resource(actions);
    }
    pub fn current_tick(&self) -> Tick {
        *self.world.get_resource::<Tick>().unwrap_or(&Tick(420))
    }
}
