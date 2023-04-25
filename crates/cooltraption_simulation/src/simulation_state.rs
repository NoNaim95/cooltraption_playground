use bevy_ecs::component::Component;
use bevy_ecs::prelude::World;
use bevy_ecs::query::QueryIter;

use crate::{stages::physics_stage::DeltaTime, Actions, Tick};

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

    pub fn load_current_tick(&mut self, current_tick: Tick) {
        self.world_mut().insert_resource(current_tick);
    }

    pub fn load_delta_time(&mut self, dt: DeltaTime) {
        self.world_mut().insert_resource(dt);
    }

    pub fn query<C: Component>(&mut self, mut f: impl FnMut(ComponentIter<C>)) {
        let mut query = self.world.query::<&C>();
        f(ComponentIter(query.iter(&self.world)));
    }

    pub fn load_actions(&mut self, actions: Actions) {
        self.world_mut().insert_resource(actions);
    }

    pub fn current_tick(&self) -> Tick {
        *self.world.get_resource::<Tick>().unwrap()
    }
}
