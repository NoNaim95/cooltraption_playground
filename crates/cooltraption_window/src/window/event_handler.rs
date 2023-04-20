use std::cell::RefCell;
use std::rc::Rc;

pub use winit::event::*;
pub use winit::event_loop::{ControlFlow, EventLoopProxy};
pub use winit::window::Window;

pub use crate::window::{CooltraptionEvent, WgpuState};

pub struct Context<'a> {
    pub control_flow: &'a mut ControlFlow,
    pub window: &'a Window,
    pub wgpu_state: &'a mut WgpuState,
    pub event_loop_proxy: &'a EventLoopProxy<CooltraptionEvent>,
    pub event_handlers: &'a mut Vec<Rc<RefCell<dyn EventHandler>>>,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &mut Event<CooltraptionEvent>, context: &mut Context);
}
