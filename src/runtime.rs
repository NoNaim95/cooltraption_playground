use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy_ecs::prelude::Query;
use bevy_ecs::schedule::{Schedule, Stage, SystemStage};

use crate::components::{Drawable, Position};
use crate::render::RenderMachine;
use crate::render::RenderStage;
use crate::scene::{LoadScene, Scene};
use crate::stages::physics_stage;
use crate::stages::physics_stage::{DeltaTime, PhysicsStage};

pub struct RuntimeOptions<S: Scene, E: Error> {
    pub scene_loader: Box<dyn LoadScene<S, E>>,
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
    pub async fn start<S: Scene, E: Error>(options: &'r RuntimeOptions<S, E>) -> RuntimeImpl<'r> {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        let mut render_machine = RenderMachine::create_window().await;

        let scene = Box::new(
            options
                .scene_loader
                .load(render_machine.wgpu_state_mut())
                .expect("valid scene object"),
        );

        schedule.add_stage_after(
            PhysicsStage,
            RenderStage,
            SystemStage::parallel().with_system(move |query: Query<(&Position, &Drawable)>| {
                render_machine.update_state(query);
            }),
        );

        Self { scene, schedule }
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
