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
    event_loop_proxy: &'a EventLoopProxy<CooltraptionEvent>,
    event_handlers: &'a mut Vec<Rc<RefCell<dyn EventHandler>>>,
}

impl<'a> Context<'a> {
    pub fn new(
        control_flow: &'a mut ControlFlow,
        window: &'a Window,
        wgpu_state: &'a mut WgpuState,
        event_loop_proxy: &'a EventLoopProxy<CooltraptionEvent>,
        event_handlers: &'a mut Vec<Rc<RefCell<dyn EventHandler>>>,
    ) -> Self {
        Self {
            control_flow,
            window,
            wgpu_state,
            event_loop_proxy,
            event_handlers,
        }
    }

    pub fn register_event_handler(&mut self, handler: Rc<RefCell<dyn EventHandler>>) {
        self.event_handlers.push(handler);
    }

    pub fn send_event(&self, event: CooltraptionEvent) {
        self.event_loop_proxy.send_event(event).expect("send event");
    }
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &mut Event<CooltraptionEvent>, context: &mut Context);
}
