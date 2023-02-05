use crate::asset_bundle;
use crate::asset_bundle::file_asset_loader::FileAssetLoader;
use crate::asset_bundle::{AssetBundle, LoadAssetBundle};
use bevy_ecs::prelude::*;
use log::debug;
use std::error::Error;
use std::hash::Hash;
use std::time::Instant;
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowId};

use crate::components::{Drawable, Position};
use crate::render::wgpu_state::WgpuState;

mod camera;
mod instance;
pub mod vertex;
pub mod wgpu_state;

#[derive(StageLabel)]
pub struct RenderStage;

#[derive(Default)]
pub struct RenderWorld {
    state: Vec<(Position, Drawable)>,
}

pub struct RenderMachine {
    state: [RenderWorld; 2],
    window: Window,
    wgpu_state: WgpuState,
    assets: Box<AssetBundle<String>>,
}

impl RenderMachine {
    pub async fn run<T: LoadAssetBundle<String, E>, E: Error>(asset_loader: &T) {
        let event_loop = EventLoopBuilder::new().build();
        let window = Window::new(&event_loop).expect("Could not create window");
        let mut wgpu_state = WgpuState::new(&window).await;
        let assets = Box::new(
            asset_loader
                .load::<String>(&mut wgpu_state)
                .expect("load assets"),
        );

        Self {
            state: [RenderWorld::default(), RenderWorld::default()],
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

    pub fn update_state(&mut self, query: Query<(&Position, &Drawable)>) {
        self.state.swap(0, 1);

        self.state[0] = RenderWorld {
            state: query.iter().map(|(p, d)| (p.clone(), d.clone())).collect(),
        };
    }

    pub fn render(&mut self) {
        // TODO: Identify different render sets and render them one by one

        for (position, drawable) in &self.state[0].state {
            self.wgpu_state.render_object(position, drawable);
        }
    }

    fn run_event_loop(mut self, event_loop: EventLoop<()>) {
        let mut start_time = Instant::now();
        let mut frame_time = start_time - Instant::now();
        let window_id = self.window.id();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id: event_window_id,
                } if event_window_id == window_id => {
                    if self.wgpu_state.input(event) {
                        match event {
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
                        }
                    }
                }
                Event::RedrawRequested(event_window_id) if window_id == event_window_id => {
                    self.request_redraw_window();
                }
                Event::RedrawEventsCleared => {
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
