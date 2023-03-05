use std::cell::RefCell;
use std::rc::Rc;

use log::error;
use wgpu::SurfaceError;
use winit::event::Event;

use crate::{Context, CooltraptionEvent, EventHandler};
use crate::render::Renderer;

pub struct RenderEventHandler {
    renderers: Vec<Rc<RefCell<dyn Renderer>>>,
}

impl RenderEventHandler {
    pub fn new(renderers: Vec<Rc<RefCell<dyn Renderer>>>) -> Self {
        Self { renderers }
    }

    pub fn add_renderer(&mut self, renderer: Rc<RefCell<dyn Renderer>>) {
        self.renderers.push(renderer);
    }
}

impl EventHandler for RenderEventHandler {
    fn handle_event(&mut self, event: &Event<CooltraptionEvent>, context: &mut Context) {
        match event {
            Event::UserEvent(CooltraptionEvent::Init) => {
                for renderer in &self.renderers {
                    renderer
                        .borrow_mut()
                        .init(context.window, context.wgpu_state);
                }
            }
            Event::UserEvent(CooltraptionEvent::Render) => {
                match context.wgpu_state.create_render_frame(context.window) {
                    Ok(mut render_frame) => {
                        for renderer in &mut self.renderers {
                            renderer.borrow_mut().render(&mut render_frame);
                        }
                        render_frame.present();
                    }
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                        context.wgpu_state.set_size(context.wgpu_state.size)
                    }
                    Err(e) => error!("{}", e),
                }
            }
            _ => {}
        }
    }
}
