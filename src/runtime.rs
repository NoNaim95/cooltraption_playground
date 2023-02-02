use std::error::Error;
use std::time::{Duration, Instant};

use bevy_ecs::prelude::Query;
use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
use log::debug;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::components::{Drawable, Position};
use crate::render::RenderMachine;
use crate::render::RenderStage;
use crate::scene::{LoadScene, Scene};
use crate::stages::physics_stage;
use crate::stages::physics_stage::{DeltaTime, PhysicsStage};

pub struct RuntimeOptions<S: Scene, E: Error> {
    pub scene_loader: Box<dyn LoadScene<S, E>>,
}

pub trait Runtime {
    fn load_scene<T: Scene + 'static>(&mut self, scene: T);
    fn step_simulation(&mut self, dt: Duration);
}

pub struct RuntimeImpl {
    scene: Box<dyn Scene + 'static>,
    schedule: Schedule,
}

impl RuntimeImpl {
    pub async fn run<S: Scene + 'static, E: Error>(options: &RuntimeOptions<S, E>) {
        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        let (mut render_machine, event_loop) = RenderMachine::create_window().await;

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

        let runtime = RuntimeImpl { scene, schedule };

        runtime.run_event_loop(event_loop);
    }

    fn run_event_loop(mut self, event_loop: EventLoop<()>) {
        let start_time = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            use winit::event::Event::*;
            match event {
                RedrawRequested(_) => {
                    self.step_simulation(Instant::now() - start_time);
                }
                NewEvents(_) => {}
                MainEventsCleared => {}
                RedrawEventsCleared => {}
                _ => debug!("Received event: {:?}", &event),
            }
        });
    }
}

impl Runtime for RuntimeImpl {
    fn load_scene<T: Scene + 'static>(&mut self, scene: T) {
        self.scene = Box::new(scene);
    }

    fn step_simulation(&mut self, dt: Duration) {
        self.scene.world_mut().insert_resource(DeltaTime::from(dt));
        self.schedule.run(self.scene.world_mut());
    }
}
