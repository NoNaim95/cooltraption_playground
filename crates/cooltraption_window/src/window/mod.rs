use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};

use cgmath::Vector3;
use log::{debug, error};
use wgpu::SurfaceError;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};

use camera::Camera;

use crate::asset_bundle::LoadAssetBundle;
use crate::render::gui::Gui;
use crate::render::instance_renderer::{InstanceRenderer, WorldState};
use crate::render::instance_renderer::texture_atlas::TextureAtlasBuilder;
use crate::render::Renderer;
use crate::window::controls::CameraControls;
use crate::window::keyboard_state::KeyboardState;

pub use self::wgpu_state::WgpuState;

pub mod camera;
pub mod controls;
pub mod keyboard_state;
pub mod wgpu_state;

pub struct WgpuWindowConfig<E: Error> {
    pub asset_loader: Box<dyn LoadAssetBundle<E>>,
    pub state_recv: Receiver<WorldState>,
    pub keyboard_send: Sender<KeyboardState>,
    pub controls_recv: Receiver<CameraControls>,
}

pub struct WgpuWindow {
    wgpu_state: WgpuState,
    renderers: Vec<Box<dyn Renderer>>,
    window: Window,
    camera: Camera,
    keyboard_state: KeyboardState,
    keyboard_send: Sender<KeyboardState>,
    controls_recv: Receiver<CameraControls>,
}

impl WgpuWindow {
    pub async fn run<E: Error>(options: WgpuWindowConfig<E>) {
        let event_loop = EventLoopBuilder::new().build();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1200, 800))
            .build(&event_loop)
            .expect("create window");

        let wgpu_state = WgpuState::new(&window).await;
        let camera = Camera::new(wgpu_state.aspect());

        let renderer = {
            let mut texture_atlas_builder = TextureAtlasBuilder::default();
            let assets = options
                .asset_loader
                .load(&mut texture_atlas_builder)
                .expect("load assets");

            let texture_atlas = texture_atlas_builder.build(&wgpu_state.device, &wgpu_state.queue);

            InstanceRenderer::new(&wgpu_state, assets, texture_atlas, options.state_recv)
        };
        let gui = Gui::new(&window, &wgpu_state);

        Self {
            window,
            renderers: vec![Box::new(renderer), Box::new(gui)],
            keyboard_state: KeyboardState::default(),
            keyboard_send: options.keyboard_send,
            controls_recv: options.controls_recv,
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

    pub fn render(&mut self) {
        self.wgpu_state.update_camera_buffer(&self.camera);

        match self.wgpu_state.create_render_frame(&self.window) {
            Ok(mut render_frame) => {
                for renderer in &mut self.renderers {
                    renderer.render(&mut render_frame);
                }

                render_frame.present()
            }
            Err(SurfaceError::Lost | SurfaceError::Outdated) => self.reset_size(),
            Err(e) => error!("{}", e),
        }
    }

    fn run_event_loop(mut self, event_loop: EventLoop<()>) {
        let window_id = self.window.id();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            // TODO: Make gui handle events
            //self.gui.handle_event(&event);

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id: event_window_id,
                } if event_window_id == window_id => self.handle_window_event(event, control_flow),
                Event::RedrawRequested(event_window_id) if window_id == event_window_id => {
                    self.request_redraw_window();
                }
                Event::RedrawEventsCleared => {
                    while let Ok(controls) = self.controls_recv.try_recv() {
                        self.handle_controls(&controls);
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
                    self.keyboard_send
                        .send(self.keyboard_state.clone())
                        .expect("Send keyboard state");
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

    fn handle_controls(&mut self, controls: &CameraControls) {
        self.camera.target += Vector3::new(controls.move_vec.x, controls.move_vec.y, 0.0);
        self.camera.zoom *= controls.zoom;
    }
}
