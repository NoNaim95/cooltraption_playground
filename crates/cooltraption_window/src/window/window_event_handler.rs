use crate::events::EventHandler;
use crate::window::{WindowContext, WinitEvent};
use winit::event_loop::ControlFlow;

pub struct WindowEventHandler {}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for WindowEventHandler {
    fn handle_event(&mut self, event: &mut WinitEvent, context: &mut WindowContext) {
        match event.0 {
            winit::event::Event::WindowEvent {
                ref event,
                window_id: event_window_id,
            } if event_window_id == &context.window.id() => {
                if event == &winit::event::WindowEvent::CloseRequested {
                    *context.control_flow = ControlFlow::Exit
                }
            }
            _ => {}
        }
    }
}
