use bevy_ecs::prelude::World;
use bevy_ecs::query::{QueryIter, WorldQuery};

use crate::{stages::physics_stage::DeltaTime, Actions, Tick};

#[derive(Default)]
pub struct SimulationState {
    world: World,
}

pub type ComponentIter<'a, C> = QueryIter<'a, 'a, C, ()>;

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
        self.world_mut().insert_resource(dt);
    }

    pub fn query<C: WorldQuery<ReadOnly = C>>(&mut self, mut f: impl FnMut(ComponentIter<C>)) {
        let mut query = self.world.query::<C>();
        f(query.iter(&self.world));
    }

    pub fn load_actions(&mut self, actions: Actions) {
        self.world_mut().insert_resource(actions);
    }

    pub fn current_tick(&self) -> Tick {
        *self.world.get_resource::<Tick>().unwrap_or(&Tick(420))
    }
}
