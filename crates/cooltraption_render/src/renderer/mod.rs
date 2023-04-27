use std::cell::RefCell;
use std::rc::Rc;

use crate::window::WindowContext;
use render_frame::RenderFrame;

pub mod gui;
pub mod render_event_handler;
pub mod render_frame;
pub mod vertex;
pub mod world_renderer;

pub type SharedRenderer = Rc<RefCell<dyn Renderer>>;
pub type BoxedRendererInitializer = Box<dyn RendererInitializer>;

pub trait Renderer {
    fn render(&mut self, render_frame: &mut RenderFrame);
}

pub trait RendererInitializer {
    fn init(self: Box<Self>, context: &mut WindowContext) -> SharedRenderer;
}
