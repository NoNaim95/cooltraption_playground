use crate::scene::Scene;
use crate::stages::physics_stage;
use crate::stages::physics_stage::PhysicsStage;
use bevy_ecs::{
    schedule::{Schedule, Stage, SystemStage},
    system::Resource,
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

pub trait Runtime<'r> {
    fn load_scene<T>(&mut self, scene: T)
    where
        T: Scene + 'r;
    fn step_simulation(&mut self, dt: Duration);
}

pub struct RuntimeImpl<'r> {
    scene: Box<dyn Scene + 'r>,
    schedule: Schedule,
}

impl<'r> RuntimeImpl<'r> {
    pub fn new<T>(scene: T) -> Self
    where
        T: Scene + 'r,
    {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        Self {
            scene: Box::new(scene),
            schedule,
        }
    }
}

impl<'r> Runtime<'r> for RuntimeImpl<'r> {
    fn load_scene<T>(&mut self, scene: T)
    where
        T: Scene + 'r,
    {
        self.scene = Box::new(scene);
    }

    fn step_simulation(&mut self, dt: Duration) {
        self.scene.world_mut().insert_resource(DeltaTime::from(dt));
        self.schedule.run(self.scene.world_mut());
    }
}
