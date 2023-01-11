use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
use std::time::Duration;

use crate::scene::Scene;
use crate::stages::physics_stage::{self, DeltaTime, PhysicsStage};

pub struct RuntimeOptions {
    pub initial_scene: Box<dyn Scene>,
}

pub trait Runtime<'r> {
    fn load_scene<T: Scene + 'r>(&mut self, scene: T);
    fn step_simulation(&mut self, dt: Duration);
}

pub struct RuntimeImpl<'r> {
    scene: Box<dyn Scene + 'r>,
    schedule: Schedule,
}

impl<'r> RuntimeImpl<'r> {
    pub fn new(options: RuntimeOptions) -> Self {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        Self {
            scene: options.initial_scene,
            schedule,
        }
    }
}

impl<'r> Runtime<'r> for RuntimeImpl<'r> {
    fn load_scene<T: Scene + 'r>(&mut self, scene: T) {
        self.scene = Box::new(scene);
    }

    fn step_simulation(&mut self, dt: Duration) {
        self.scene.world_mut().insert_resource(DeltaTime::from(dt));
        self.schedule.run(self.scene.world_mut());
    }
}
