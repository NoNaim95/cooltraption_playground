use bevy_ecs::prelude::{World, Component};
use bevy_ecs::query::{QueryIter, WorldQuery};
use cooltraption_common::events::EventFamily;

use crate::{system_sets::physics_set::DeltaTime, Actions, Tick};

pub struct SimulationState {
    world: World,
}

impl Default for SimulationState {
    fn default() -> Self {
        let mut state = Self { world: Default::default() };
        state.load_current_tick(Tick(0));
        state
    }
}

pub struct ComponentIter<'a, C: Component>(QueryIter<'a, 'a, &'a C, ()>);

impl<'a, C: Component> Iterator for ComponentIter<'a, C> {
    type Item = &'a C;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl SimulationState {
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    pub fn advance_tick(&mut self){
        self.load_current_tick(Tick(self.current_tick().0 + 1))
    }

    pub fn load_current_tick(&mut self, current_tick: Tick) {
        self.world_mut().insert_resource(current_tick);
    }

    pub fn load_delta_time(&mut self, dt: DeltaTime) {
        self.world_mut().insert_resource(dt);
    }

    pub fn query<WQ: WorldQuery<ReadOnly = WQ>>(&mut self, mut f: impl FnMut(QueryIter<WQ, ()>)) {
        let mut query = self.world.query::<WQ>();
        f(query.iter(&self.world));
    }

    pub fn load_actions(&mut self, actions: Actions) {
        self.world_mut().insert_resource(actions);
    }

    pub fn current_tick(&self) -> Tick {
        *self.world.get_resource::<Tick>().unwrap()
    }
}
