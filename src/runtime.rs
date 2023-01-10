use std::cell::RefCell;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::components::{Drawable, Position};
use crate::render::RenderMachine;
use bevy_ecs::prelude::Query;
use bevy_ecs::{
    schedule::{Schedule, Stage, SystemStage},
    system::Resource,
};

use crate::scene::Scene;
use crate::stages::physics_stage::PhysicsStage;
use crate::stages::render_stage::RenderStage;
use crate::stages::{physics_stage, render_stage};

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

pub struct RuntimeOptions {
    pub initial_scene: Box<dyn Scene>,
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
    render_machine: Arc<Mutex<RenderMachine>>,
}

impl<'r> RuntimeImpl<'r> {
    pub fn new(options: RuntimeOptions) -> Self {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        let render_machine = Arc::new(Mutex::new(RenderMachine::default()));

        let mut render_machine_capture = Arc::clone(&render_machine);

        schedule.add_stage_after(
            PhysicsStage,
            RenderStage,
            SystemStage::parallel().with_system(move |query: Query<(&Position, &Drawable)>| {
                let x = Arc::get_mut(&mut render_machine_capture).unwrap();
                x.get_mut().unwrap().update_state(query)
            }),
        );

        Self {
            scene: options.initial_scene,
            schedule,
            render_machine,
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
