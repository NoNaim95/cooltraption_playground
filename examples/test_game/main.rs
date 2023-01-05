#[allow(unused, dead_code)]
use cooltraption_playground::engine::physics_engine::*;
use cooltraption_playground::engine::{Engine, EngineImpl};

use bevy_ecs::prelude::*;
use std::time::Duration;

mod entities;

fn main() {
    let mut world = World::new();

    let ent = world
        .spawn((
            Acceleration::default(),
            Velocity::default(),
            Position::default(),
        ))
        .id();
    let mut ent_mut = world.get_entity_mut(ent).unwrap();
    let mut vel = ent_mut.get_mut::<Velocity>().unwrap();
    vel.0.x = 3.0;
    vel.0.y = 1.0;

    let mut engine = EngineImpl::new(world);
    for i in 0..3 {
        engine.step_simulation(Duration::from_secs(i));
    }
}
