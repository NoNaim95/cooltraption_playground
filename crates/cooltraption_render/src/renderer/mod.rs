use crate::renderer::wgpu_state::WgpuState;
use cooltraption_window::events::EventHandler;
use cooltraption_window::window::{WindowContext, WindowEvent, WinitEvent};
use log::error;
use render_frame::RenderFrame;
use std::time::{Duration, Instant};
use wgpu::{CommandEncoderDescriptor, SurfaceError, TextureViewDescriptor};
use winit::event::Event;
use winit::window::Window;

pub mod gui;
pub mod render_frame;
pub mod wgpu_state;
pub mod world_renderer;

pub type BoxedRenderer = Box<dyn Renderer>;
pub type BoxedRendererInitializer = Box<dyn RendererInitializer>;

pub trait RenderError: std::error::Error {}

pub trait Renderer {
    fn render(&mut self, render_frame: &mut RenderFrame) -> Result<(), Box<dyn RenderError>>;
}

pub trait RendererInitializer {
    fn init(self: Box<Self>, wgpu_state: &mut WgpuState, window: &Window) -> BoxedRenderer;
}

#[derive(Default)]
pub struct WgpuInitializer {
    initializers: Vec<BoxedRendererInitializer>,
}

pub struct WgpuWindowRenderer {
    wgpu_state: WgpuState,
    renderers: Vec<BoxedRenderer>,
    prev_frame_time: Instant,
}

impl WgpuInitializer {
    pub fn add_initializer(&mut self, initializer: BoxedRendererInitializer) {
        self.initializers.push(initializer);
    }
}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for WgpuInitializer {
    fn handle_event(&mut self, event: &mut WinitEvent, context: &mut WindowContext) {
        if let Event::UserEvent(WindowEvent::Init) = event.0 {
            let mut wgpu_state = WgpuState::new(context.window);
            let renderers: Vec<_> = self
                .initializers
                .drain(0..self.initializers.len())
                .map(|initializer| initializer.init(&mut wgpu_state, context.window))
                .collect();

            let window_renderer = WgpuWindowRenderer {
                wgpu_state,
                renderers,
                prev_frame_time: Instant::now(),
            };

            context.register_event_handler(Box::new(window_renderer));
        }
    }
}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for WgpuWindowRenderer {
    fn handle_event(&mut self, event: &mut WinitEvent, context: &mut WindowContext) {
        match event.0 {
            Event::RedrawRequested(ref event_window_id)
                if &context.window.id() == event_window_id =>
            {
                context.window.request_redraw();
            }
            Event::RedrawEventsCleared => {
                let delta_time = Instant::now() - self.prev_frame_time;

                match create_render_frame(context.window, &self.wgpu_state, &delta_time) {
                    Ok(mut render_frame) => {
                        self.renderers.retain_mut(|renderer| {
                            renderer
                                .render(&mut render_frame)
                                .map_err(|e| error!("{}", e)) // Remove renderer if error
                                .is_ok()
                        });
                        render_frame.present();
                    }
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                        let size = self.wgpu_state.size;
                        self.wgpu_state.set_size(size)
                    }
                    Err(e) => error!("{}", e),
                }

                context.send_event(WindowEvent::Render(Instant::now() - self.prev_frame_time));

                self.prev_frame_time = Instant::now();
            }
            Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    self.wgpu_state.set_size(*physical_size);
                }
                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.wgpu_state.set_size(**new_inner_size);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn create_render_frame<'a>(
    window: &'a Window,
    wgpu_state: &'a WgpuState,
    delta_time: &'a Duration,
) -> Result<RenderFrame<'a>, SurfaceError> {
    let output = wgpu_state.surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&TextureViewDescriptor::default());

    let encoder = wgpu_state
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    Ok(RenderFrame {
        delta_time,
        window,
        device: &wgpu_state.device,
        queue: &wgpu_state.queue,
        output,
        view,
        encoder,
    })
}
