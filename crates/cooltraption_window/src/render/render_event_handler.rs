use std::cell::RefCell;
use std::rc::Rc;

use log::error;
use wgpu::SurfaceError;
use winit::event::Event;

use crate::render::{Renderer, RendererInitializer};
use crate::{Context, CooltraptionEvent, EventHandler};

#[derive(Default)]
pub struct RenderEventHandler {
    initializers: Vec<Box<dyn RendererInitializer>>,
    renderers: Vec<Rc<RefCell<dyn Renderer>>>,
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
                        let size = context.wgpu_state.size;
                        context.wgpu_state.set_size(size)
                    }
                    Err(e) => error!("{}", e),
                }
            }
            _ => {}
        }
    }
}
