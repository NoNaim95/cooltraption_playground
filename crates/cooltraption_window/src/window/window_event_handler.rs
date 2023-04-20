use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

use crate::window::{Context, CooltraptionEvent, EventHandler};

pub struct WindowEventHandler {}

impl EventHandler for WindowEventHandler {
    fn handle_event(&mut self, event: &mut Event<CooltraptionEvent>, context: &mut Context) {
        match event {
            Event::WindowEvent {
                ref event,
                window_id: event_window_id,
            } if event_window_id == &context.window.id() => match event {
                WindowEvent::CloseRequested => *context.control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    context.wgpu_state.set_size(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    context.wgpu_state.set_size(**new_inner_size);
                }
                _ => {}
            },
            Event::MainEventsCleared => {}
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::NewEvents(_) => {}
            _ => {}
        }
    }
}
