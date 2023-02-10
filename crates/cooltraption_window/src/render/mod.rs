use crate::asset_bundle::{AssetBundle, LoadAssetBundle};
use crate::render::instance::Instance;
use crate::render::instance_renderer::InstanceRenderer;
use crate::render::texture_atlas::texture_atlas_builder::TextureAtlasBuilder;
use crate::render::wgpu_state::WgpuState;
use cgmath::Vector2;
use log::{debug, error};
use std::error::Error;
use std::sync::mpsc::Receiver;
use wgpu::SurfaceError;
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::Window;

mod camera;
mod instance;
mod instance_renderer;
pub mod keyboard_state;
pub mod texture_atlas;
pub mod vertex;
mod wgpu_state;

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

#[derive(Default, Debug)]
pub struct WorldState {
    state: Vec<Instance>,
}

pub struct WgpuWindowConfig<E: Error> {
    pub asset_loader: Box<dyn LoadAssetBundle<String, E>>,
    pub state_recv: Receiver<WorldState>,
}

pub struct WgpuWindow {
    world_state: [WorldState; 2],
    wgpu_state: WgpuState,
    renderer: InstanceRenderer,
    state_recv: Receiver<WorldState>,
    window: Window,
    assets: Box<AssetBundle<String>>,
}

impl WgpuWindow {
    pub async fn run<E: Error>(options: WgpuWindowConfig<E>) {
        let event_loop = EventLoopBuilder::new().build();
        let window = Window::new(&event_loop).expect("create window");

        let mut wgpu_state = WgpuState::new(&window).await;

        let mut texture_atlas_builder =
            TextureAtlasBuilder::new(&mut wgpu_state.device, &mut wgpu_state.queue);
        let assets = Box::new(
            options
                .asset_loader
                .load(&mut texture_atlas_builder)
                .expect("load assets"),
        );

        let texture_atlas = texture_atlas_builder.build();

        let renderer = InstanceRenderer::new(&wgpu_state, texture_atlas);

        Self {
            world_state: [WorldState::default(), WorldState::default()],
            state_recv: options.state_recv,
            window,
            renderer,
            assets,
            wgpu_state,
        }
        .run_event_loop(event_loop);
    }

    pub fn request_redraw_window(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.wgpu_state.size = new_size;
            self.wgpu_state.config.width = new_size.width;
            self.wgpu_state.config.height = new_size.height;
            self.wgpu_state
                .surface
                .configure(&self.wgpu_state.device, &self.wgpu_state.config);
        }
    }

    pub fn reset_size(&mut self) {
        self.resize(self.wgpu_state.size);
    }

    pub fn update_state(&mut self, new_state: WorldState) {
        self.world_state.swap(0, 1);

        self.world_state[0] = new_state;
    }

    pub fn render(&mut self) {
        // TODO: Identify different render sets and render them one by one

        match self
            .renderer
            .render_all(&self.world_state[0].state, &self.wgpu_state)
        {
            Ok(_) => {}
            Err(SurfaceError::Lost) => self.reset_size(),
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
                        self.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.resize(**new_inner_size);
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
}
