use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use log::error;
use wgpu::{CommandEncoderDescriptor, SurfaceError, TextureViewDescriptor};
use winit::event::Event;
use winit::window::Window;

use crate::render::{RenderFrame, Renderer, RendererInitializer};
use crate::{Context, CooltraptionEvent, EventHandler, WgpuState};

pub struct RenderEventHandler {
    prev_frame_time: Instant,
    initializers: Vec<Box<dyn RendererInitializer>>,
    renderers: Vec<Rc<RefCell<dyn Renderer>>>,
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
    pub fn add_initializer(&mut self, initializer: Box<dyn RendererInitializer>) {
        self.initializers.push(initializer);
    }

    pub fn add_renderer(&mut self, renderer: Rc<RefCell<dyn Renderer>>) {
        self.renderers.push(renderer);
    }
}

impl EventHandler for RenderEventHandler {
    fn handle_event(&mut self, event: &Event<CooltraptionEvent>, context: &mut Context) {
        match event {
            Event::UserEvent(CooltraptionEvent::Init) => {
                for initializer in self.initializers.drain(0..self.initializers.len()) {
                    let renderer = initializer.init(context);
                    self.renderers.push(renderer);
                }

                self.prev_frame_time = Instant::now();
            }
            Event::RedrawRequested(event_window_id) if &context.window.id() == event_window_id => {
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

                context
                    .event_loop_proxy
                    .send_event(CooltraptionEvent::Render(
                        Instant::now() - self.prev_frame_time,
                    ))
                    .expect("Send render event");

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
