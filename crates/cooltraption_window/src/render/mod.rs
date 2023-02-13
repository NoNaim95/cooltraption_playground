use std::error::Error;
use std::sync::mpsc::Receiver;

use log::{debug, error};
use wgpu::SurfaceError;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};

use crate::asset_bundle::{AssetBundle, LoadAssetBundle};
use crate::render::camera::Camera;
use crate::render::instance_renderer::InstanceRenderer;
use crate::render::keyboard_state::KeyboardState;
use crate::render::texture_atlas::texture_atlas_builder::TextureAtlasBuilder;
use crate::render::wgpu_state::WgpuState;
pub use crate::render::world_state::*;

mod camera;
mod instance;
mod instance_renderer;
pub mod keyboard_state;
pub mod texture_atlas;
pub mod vertex;
mod wgpu_state;
mod world_state;

pub struct WgpuWindowConfig<E: Error> {
    pub asset_loader: Box<dyn LoadAssetBundle<E>>,
    pub state_recv: Receiver<WorldState>,
}

pub struct WgpuWindow {
    world_state: [WorldState; 2],
    wgpu_state: WgpuState,
    renderer: InstanceRenderer,
    state_recv: Receiver<WorldState>,
    window: Window,
    assets: Box<AssetBundle>,
    keyboard_state: KeyboardState,
    camera: Camera,
}

impl WgpuWindow {
    pub async fn run<E: Error>(options: WgpuWindowConfig<E>) {
        let event_loop = EventLoopBuilder::new().build();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1200, 800))
            .build(&event_loop)
            .expect("create window");

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

        let camera = Camera::new(wgpu_state.aspect());

        Self {
            world_state: [WorldState::default(), WorldState::default()],
            state_recv: options.state_recv,
            window,
            renderer,
            keyboard_state: KeyboardState::default(),
            assets,
            wgpu_state,
            camera,
        }
        .run_event_loop(event_loop);
    }

    pub fn request_redraw_window(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.wgpu_state.set_size(new_size);
            self.camera.aspect = self.wgpu_state.aspect();
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
        self.wgpu_state.update_camera_buffer(&self.camera);

        let instances = self.world_state[1].interpolate(
            &self.world_state[0],
            &self.assets,
            self.renderer.texture_atlas(),
        );

        match self
            .renderer
            .render_all(instances.as_slice(), &self.wgpu_state)
        {
            Ok(_) => {}
            Err(SurfaceError::Lost | SurfaceError::Outdated) => self.reset_size(),
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
                } if event_window_id == window_id => self.handle_window_event(event, control_flow),
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

    fn handle_window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(vk_code) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => self.keyboard_state += vk_code,
                        ElementState::Released => self.keyboard_state -= vk_code,
                    }
                }
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize(**new_inner_size);
            }
            _ => {}
        }
    }
}
