use cooltraption_common::events::EventPublisher;
use cooltraption_render::events::EventHandler;
use cooltraption_render::window::winit::event::{
    ElementState, KeyboardInput, MouseButton, VirtualKeyCode,
};
use cooltraption_render::window::{winit, WindowContext, WinitEvent};

#[derive(Default)]
struct InputEventHandler<'a> {
    event_publisher: EventPublisher<'a, InputEvent>,
}

pub enum InputEvent {
    KeyboardInputEvent(KeyboardInputEvent),
    MouseButtonEvent(MouseButtonEvent),
}

pub enum KeyboardInputEvent {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
}

pub enum MouseButtonEvent {
    KeyPressed(MouseButton),
    KeyReleased(MouseButton),
}

impl<'a> InputEventHandler<'a> {
    pub fn new(event_publisher: EventPublisher<'a, InputEvent>) -> Self {
        Self { event_publisher }
    }

    fn keyboard_input(&mut self, input: &mut KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            let event = match input.state {
                ElementState::Pressed => {
                    InputEvent::KeyboardInputEvent(KeyboardInputEvent::KeyPressed(key_code))
                }
                ElementState::Released => {
                    InputEvent::KeyboardInputEvent(KeyboardInputEvent::KeyReleased(key_code))
                }
            };
            self.event_publisher.publish(&event);
        }
    }

    fn mouse_input(&mut self, button: MouseButton, state: &mut ElementState) {
        let event = match state {
            ElementState::Pressed => {
                InputEvent::MouseButtonEvent(MouseButtonEvent::KeyPressed(button))
            }
            ElementState::Released => {
                InputEvent::MouseButtonEvent(MouseButtonEvent::KeyReleased(button))
            }
        };
        self.event_publisher.publish(&event);
    }
}

impl<'a> EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for InputEventHandler<'a> {
    fn handle_event(&mut self, event: &mut WinitEvent, _context: &mut WindowContext) {
        if let winit::event::Event::WindowEvent { event, .. } = event.0 {
            match event {
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    self.keyboard_input(input)
                }
                winit::event::WindowEvent::MouseInput { button, state, .. } => {
                    self.mouse_input(*button, state)
                }
                _ => {}
            }
        }
    }
}
