use crate::asset_bundle::{AssetBundle, LoadAssetBundle};
use crate::render::texture_atlas_builder::TextureAtlasBuilder;
use crate::render::uninitialized_wgpu_state::UninitializedWgpuState;
use bevy_ecs::prelude::*;
use cgmath::Vector2;
use log::{debug, error};
use std::error::Error;
use std::sync::mpsc::Receiver;
use wgpu::SurfaceError;
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowId};

use crate::render::wgpu_state::WgpuState;

mod camera;
mod instance;
mod render_batch;
pub mod texture_atlas_builder;
mod uninitialized_wgpu_state;
pub mod vertex;
pub mod wgpu_state;

#[derive(Clone, Debug)]
pub struct Position(pub Vector2<f64>);

impl Default for Position {
    fn default() -> Self {
        Self {
            0: Vector2::new(0.0, 0.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Drawable {
    pub asset: String,
}

#[derive(StageLabel)]
pub struct RenderStage;

#[derive(Default, Debug)]
pub struct RenderWorld {
    state: Vec<(Position, Drawable)>,
}

impl RenderWorld {
    pub fn new<'a, T: Iterator<Item = (&'a Position, &'a Drawable)>>(drawables: T) -> Self {
        RenderWorld {
            state: drawables.map(|(p, d)| (p.clone(), d.clone())).collect(),
        }
    }
}

pub struct RenderMachineOptions<E: Error> {
    pub asset_loader: Box<dyn LoadAssetBundle<String, E>>,
    pub state_recv: Receiver<RenderWorld>,
}

pub struct RenderMachine {
    state: [RenderWorld; 2],
    state_recv: Receiver<RenderWorld>,
    window: Window,
    wgpu_state: WgpuState,
    assets: Box<AssetBundle<String>>,
}

impl RenderMachine {
    pub async fn run<E: Error>(options: RenderMachineOptions<E>) {
        let event_loop = EventLoopBuilder::new().build();
        let window = Window::new(&event_loop).expect("Could not create window");

        let mut uninitialized_wgpu_state = UninitializedWgpuState::new(&window).await;

        let mut atlas_builder = TextureAtlasBuilder::new(
            &mut uninitialized_wgpu_state.device,
            &mut uninitialized_wgpu_state.queue,
        );
        let assets = Box::new(
            options
                .asset_loader
                .load(&mut atlas_builder)
                .expect("load assets"),
        );

        let wgpu_state = WgpuState::from(uninitialized_wgpu_state);

        Self {
            state: [RenderWorld::default(), RenderWorld::default()],
            state_recv: options.state_recv,
            window,
            wgpu_state,
            assets,
        }
        .run_event_loop(event_loop);
    }

    pub fn request_redraw_window(&self) {
        self.window.request_redraw();
    }

    pub fn resize_window(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.wgpu_state.resize(new_size);
    }

    pub fn update_state(&mut self, new_state: RenderWorld) {
        self.state.swap(0, 1);

        self.state[0] = new_state;
    }

    pub fn render(&mut self) {
        // TODO: Identify different render sets and render them one by one

        match self.wgpu_state.render_all(&self.state[0].state) {
            Ok(_) => {}
            Err(SurfaceError::Lost) => self.wgpu_state.resize(self.wgpu_state.size()),
            Err(e) => error!("{}", e),
        }
    }

    fn run_event_loop(mut self, event_loop: EventLoop<()>) {
        let window_id = self.window.id();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id: event_window_id,
                } if event_window_id == window_id => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: winit::event::ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        self.wgpu_state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.wgpu_state.resize(**new_inner_size);
                    }
                    _ => {}
                },
                Event::RedrawRequested(event_window_id) if window_id == event_window_id => {
                    self.request_redraw_window();
                }
                Event::RedrawEventsCleared => {
                    while let Ok(state) = self.state_recv.try_recv() {
                        self.update_state(state);
                    }

                    self.render();
                }
                Event::MainEventsCleared => {}
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::NewEvents(_) => {}
                _ => debug!("Received event: {:?}", &event),
            }
        });
    }

    pub fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub fn wgpu_state(&self) -> &WgpuState {
        &self.wgpu_state
    }

    pub fn wgpu_state_mut(&mut self) -> &mut WgpuState {
        &mut self.wgpu_state
    }
}
