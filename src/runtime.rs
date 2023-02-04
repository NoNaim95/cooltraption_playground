use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use bevy_ecs::prelude::Query;
use bevy_ecs::schedule::{Schedule, Stage, SystemStage};
use fixed_macro::fixed;
use log::debug;
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use winit::event::ElementState::Pressed;

use crate::components::{Drawable, Position};
use crate::render::RenderMachine;
use crate::render::RenderStage;
use crate::scene::{LoadScene, Scene};
use crate::stages::physics_stage;
use crate::stages::physics_stage::{DeltaTime, PhysicsStage};

macro_rules! render_machine {
    ($self:ident) => {
        $self.render_machine.lock().unwrap()
    };
}

pub struct RuntimeOptions<S: Scene, E: Error> {
    pub scene_loader: Box<dyn LoadScene<S, E>>,
}

pub trait Runtime<'r> {
    fn load_scene<T: Scene + 'r>(&mut self, scene: T);
    fn step_simulation(&mut self, dt: Duration);
}

pub struct RuntimeImpl<'r> {
    scene: Box<dyn Scene + 'r>,
    render_machine: Arc<Mutex<RenderMachine>>,
    schedule: Schedule,
}

impl RuntimeImpl<'static> {
    pub async fn run<S: Scene + 'static, E: Error>(options: &RuntimeOptions<S, E>) {
        let (mut render_machine, event_loop) = RenderMachine::create_window().await;

        let scene = Box::new(
            options
                .scene_loader
                .load(render_machine.wgpu_state_mut())
                .expect("valid scene object"),
        );

        let mut schedule = Schedule::default();
        schedule.add_stage(
            PhysicsStage,
            SystemStage::parallel().with_system(physics_stage::solve_movement),
        );

        let render_machine = Arc::new(Mutex::new(render_machine));

        let render_machine_capture = Arc::clone(&render_machine);
        schedule.add_stage_after(
            PhysicsStage,
            RenderStage,
            SystemStage::parallel().with_system(move |query: Query<(&Position, &Drawable)>| {
                let mut render_machine = render_machine_capture.lock().unwrap();
                render_machine.update_state(query);
                render_machine.render();
            }),
        );

        RuntimeImpl {
            scene,
            render_machine,
            schedule,
        }
        .run_event_loop(event_loop)
    }

    fn run_event_loop(mut self, event_loop: EventLoop<()>) {
        let mut start_time = Instant::now();
        let mut frame_time = start_time - Instant::now();
        let window_id = render_machine!(self).window_id();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id: event_window_id,
                } if event_window_id == window_id => {
                    if !render_machine!(self).wgpu_state_mut().input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                render_machine!(self)
                                    .wgpu_state_mut()
                                    .resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                render_machine!(self)
                                    .wgpu_state_mut()
                                    .resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(event_window_id) if window_id == event_window_id => {
                    render_machine!(self).request_redraw_window();
                }
                Event::RedrawEventsCleared => {
                    self.step_simulation(frame_time);
                    frame_time = Instant::now() - start_time;
                    start_time = Instant::now();
                }
                Event::MainEventsCleared => {}
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::NewEvents(_) => {}
                _ => debug!("Received event: {:?}", &event),
            }
        });
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
