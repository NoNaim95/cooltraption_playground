use cooltraption_render::events::{Event, EventHandler};

use crate::input;
use cooltraption_render::window::winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
use cooltraption_render::window::winit::event_loop::{ControlFlow, EventLoopProxy};
use cooltraption_render::window::winit::window::Window;
use cooltraption_render::window::{winit, BoxedEventHandler, WindowContext, WinitEvent};

struct InputEventHandler {
    handlers: Vec<BoxedInputEventHandler>,
}

pub enum InputEvent {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
}

impl Event for InputEvent {}

impl Default for InputEventHandler {
    fn default() -> Self {
        Self { handlers: vec![] }
    }
}

impl InputEventHandler {
    pub fn new() -> Self {
        Self::default()
    }

    fn keyboard_input(&mut self, input: &mut KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            let mut event = match input.state {
                ElementState::Pressed => InputEvent::KeyPressed(key_code),
                ElementState::Released => InputEvent::KeyReleased(key_code),
            };
            for handler in &mut self.handlers {
                handler.handle_event(&mut event, &mut ());
            }
        }
    }

    pub fn register_event_handler(&mut self, handler: BoxedInputEventHandler) {
        self.handlers.push(handler);
    }
}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for InputEventHandler {
    fn handle_event(&mut self, event: &mut WinitEvent, context: &mut WindowContext) {
        match event.0 {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    self.keyboard_input(input)
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub type BoxedInputEventHandler = Box<dyn EventHandler<InputEvent, ()>>;
