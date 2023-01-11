use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::components::{Drawable, Position};
use crate::render::RenderMachine;
use bevy_ecs::prelude::Query;
use bevy_ecs::schedule::{Schedule, Stage, SystemStage};

use crate::render::RenderStage;
use crate::scene::Scene;
use crate::stages::physics_stage;
use crate::stages::physics_stage::{DeltaTime, PhysicsStage};

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

        let render_machine_capture = Arc::clone(&render_machine);

        schedule.add_stage_after(
            PhysicsStage,
            RenderStage,
            SystemStage::parallel().with_system(move |query: Query<(&Position, &Drawable)>| {
                let mutex = Arc::as_ref(&render_machine_capture);
                mutex.lock().unwrap().update_state(query)
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
    fn load_scene<T: Scene + 'r>(&mut self, scene: T) {
        self.scene = Box::new(scene);
    }

    fn step_simulation(&mut self, dt: Duration) {
        self.scene.world_mut().insert_resource(DeltaTime::from(dt));
        self.schedule.run(self.scene.world_mut());
    }
}
