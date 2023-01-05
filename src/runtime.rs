use crate::stages::physics_stage;
use crate::stages::physics_stage::PhysicsStage;
use bevy_ecs::{
    schedule::{Schedule, Stage, SystemStage},
    system::Resource,
    world::World,
};
use std::time::Duration;

const MICROS_TO_SECONDS: f64 = 1.0 / 1000000.0; // µs to s factor

#[derive(Resource, Default)]
pub struct DeltaTime {
    pub seconds: f64,
}

impl From<Duration> for DeltaTime {
    fn from(duration: Duration) -> Self {
        Self {
            seconds: (duration.as_micros() as f64 * MICROS_TO_SECONDS),
        }
    }
}

pub trait Runtime {
    fn load_world<T: Into<World>>(&mut self, world: T);
    fn step_simulation(&mut self, dt: Duration);
}

pub struct RuntimeImpl {
    world: World,
    schedule: Schedule,
}

impl RuntimeImpl {
    pub fn new(world: World) -> Self {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        Self { world, schedule }
    }
}

impl Runtime for RuntimeImpl {
    fn load_world<T: Into<World>>(&mut self, world: T) {
        self.world = world.into();
    }

    fn step_simulation(&mut self, dt: Duration) {
        self.world.insert_resource(DeltaTime::from(dt));
        self.schedule.run(&mut self.world);
    }
}
