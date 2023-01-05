use bevy_ecs::{
    schedule::{Schedule, Stage, SystemStage},
    system::Resource,
    world::World,
};
use std::time::Duration;

const MICROS_TO_SECONDS: f64 = 1.0 / 1000000.0; // µs to s factor

pub mod physics_engine;

#[derive(Resource, Default)]
pub struct DeltaTime {
    seconds: f64,
}

impl From<Duration> for DeltaTime {
    fn from(duration: Duration) -> Self {
        Self {
            seconds: (duration.as_micros() as f64 * MICROS_TO_SECONDS),
        }
    }
}

pub trait Engine {
    fn load_world<T: Into<World>>(&mut self, world: T);
    fn step_simulation(&mut self, dt: Duration);
}

pub struct EngineImpl {
    world: World,
    schedule: Schedule,
}

impl EngineImpl {
    pub fn new(world: World) -> Self {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            "PhysicsEngine",
            SystemStage::parallel().with_system(physics_engine::solve_movement),
        );
        Self { world, schedule }
    }
}

impl Engine for EngineImpl {
    fn load_world<T: Into<World>>(&mut self, world: T) {
        self.world = world.into();
    }

    fn step_simulation(&mut self, dt: Duration) {
        self.world.insert_resource(DeltaTime::from(dt));
        self.schedule.run(&mut self.world);
    }
}
