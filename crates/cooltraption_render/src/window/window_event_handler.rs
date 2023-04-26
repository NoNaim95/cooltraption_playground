use crate::EventHandler;
use winit::event_loop::ControlFlow;

use crate::window::{WindowContext, WindowEvent, WinitEvent};

pub struct WindowEventHandler {}

impl<'s> EventHandler<'s, WinitEvent<'_, WindowEvent>, WindowContext<'_>> for WindowEventHandler {
    fn handle_event(
        &'s mut self,
        event: &mut WinitEvent<WindowEvent>,
        context: &mut WindowContext,
    ) {
        match event {
            WinitEvent::WindowEvent {
                ref event,
                window_id: event_window_id,
            } if event_window_id == &context.window.id() => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *context.control_flow = ControlFlow::Exit
                }
                winit::event::WindowEvent::Resized(physical_size) => {
                    context.wgpu_state.set_size(*physical_size);
                }
                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    context.wgpu_state.set_size(**new_inner_size);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
