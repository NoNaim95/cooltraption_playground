use std::cell::RefCell;
use std::rc::Rc;

use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoopProxy};
use winit::window::Window;

use crate::{CooltraptionEvent, WgpuState};

pub struct Context<'a> {
    pub control_flow: &'a mut ControlFlow,
    pub window: &'a Window,
    pub wgpu_state: &'a mut WgpuState,
    pub event_loop_proxy: &'a EventLoopProxy<CooltraptionEvent>,
    pub event_handlers: &'a mut Vec<Rc<RefCell<dyn EventHandler>>>,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event<CooltraptionEvent>, context: &mut Context);
}
