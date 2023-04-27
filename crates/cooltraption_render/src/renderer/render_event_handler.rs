use std::time::{Duration, Instant};

use crate::events::EventHandler;
use log::error;
use wgpu::{CommandEncoderDescriptor, SurfaceError, TextureViewDescriptor};
use winit::event::Event;
use winit::window::Window;

use crate::renderer::{BoxedRendererInitializer, RenderFrame, SharedRenderer};
use crate::window::{WgpuState, WindowContext, WindowEvent, WinitEvent};

pub struct RenderEventHandler {
    prev_frame_time: Instant,
    initializers: Vec<BoxedRendererInitializer>,
    renderers: Vec<SharedRenderer>,
}

impl Default for RenderEventHandler {
    fn default() -> Self {
        Self {
            prev_frame_time: Instant::now(),
            initializers: vec![],
            renderers: vec![],
        }
    }
}

impl RenderEventHandler {
    pub fn add_initializer(&mut self, initializer: BoxedRendererInitializer) {
        self.initializers.push(initializer);
    }
}

impl<'s> EventHandler<'s, WinitEvent<'_, '_>, WindowContext<'_>> for RenderEventHandler {
    fn handle_event(&'s mut self, event: &mut WinitEvent, context: &mut WindowContext) {
        match event.0 {
            Event::UserEvent(WindowEvent::Init) => {
                for initializer in self.initializers.drain(0..self.initializers.len()) {
                    let renderer = initializer.init(context);
                    self.renderers.push(renderer);
                }

                self.prev_frame_time = Instant::now();
            }
            Event::RedrawRequested(ref event_window_id)
                if &context.window.id() == event_window_id =>
            {
                context.window.request_redraw();
            }
            Event::RedrawEventsCleared => {
                let delta_time = Instant::now() - self.prev_frame_time;

                match create_render_frame(context.window, context.wgpu_state, &delta_time) {
                    Ok(mut render_frame) => {
                        for renderer in &mut self.renderers {
                            renderer.borrow_mut().render(&mut render_frame);
                        }
                        render_frame.present();
                    }
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                        let size = context.wgpu_state.size;
                        context.wgpu_state.set_size(size)
                    }
                    Err(e) => error!("{}", e),
                }

                context.send_event(WindowEvent::Render(Instant::now() - self.prev_frame_time));

                self.prev_frame_time = Instant::now();
            }
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
