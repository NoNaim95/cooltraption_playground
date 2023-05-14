mod keyboard_state;
mod mouse_state;

use crate::input::keyboard_state::KeyboardState;
use crate::input::mouse_state::MouseState;
use cooltraption_common::events::EventPublisher;
use cooltraption_window::events::EventHandler;
use cooltraption_window::window::winit::dpi::PhysicalPosition;
use cooltraption_window::window::winit::event::{
    ElementState, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode,
};
use cooltraption_window::window::{winit, WindowContext, WinitEvent};

pub struct InputEventHandler<'a> {
    event_publisher: EventPublisher<'a, InputEvent>,
    input_state: InputState,
}

#[derive(Debug, Default, Clone)]
pub struct InputState {
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
    modifier_state: ModifiersState,
}

#[derive(Debug)]
pub enum InputEvent {
    KeyboardInputEvent(KeyboardInputEvent),
    MouseButtonEvent(MouseButtonEvent),
    MouseMoved(PhysicalPosition<f64>, InputState),
}

#[derive(Debug)]
pub enum KeyboardInputEvent {
    KeyPressed(VirtualKeyCode, InputState),
    KeyReleased(VirtualKeyCode, InputState),
}

#[derive(Debug)]
pub enum MouseButtonEvent {
    KeyPressed(MouseButton, InputState),
    KeyReleased(MouseButton, InputState),
}

impl<'a> InputEventHandler<'a> {
    pub fn new(event_publisher: EventPublisher<'a, InputEvent>) -> Self {
        Self {
            event_publisher,
            input_state: InputState::default(),
        }
    }

    fn keyboard_input(&mut self, input: &mut KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            let event = match input.state {
                ElementState::Pressed => {
                    self.input_state.keyboard_state.set_button(&key_code, true);
                    InputEvent::KeyboardInputEvent(KeyboardInputEvent::KeyPressed(
                        key_code,
                        self.input_state.clone(),
                    ))
                }
                ElementState::Released => {
                    self.input_state.keyboard_state.set_button(&key_code, false);
                    InputEvent::KeyboardInputEvent(KeyboardInputEvent::KeyReleased(
                        key_code,
                        self.input_state.clone(),
                    ))
                }
            };
            self.event_publisher.publish(&event);
        }
    }

    fn mouse_input(&mut self, button: MouseButton, state: &mut ElementState) {
        let event = match state {
            ElementState::Pressed => {
                self.input_state.mouse_state.set_button(&button, true);
                InputEvent::MouseButtonEvent(MouseButtonEvent::KeyPressed(
                    button,
                    self.input_state.clone(),
                ))
            }
            ElementState::Released => {
                self.input_state.mouse_state.set_button(&button, false);
                InputEvent::MouseButtonEvent(MouseButtonEvent::KeyReleased(
                    button,
                    self.input_state.clone(),
                ))
            }
        };
        self.event_publisher.publish(&event);
    }

    fn mouse_moved(&mut self, pos: &mut PhysicalPosition<f64>) {
        self.input_state.mouse_state.set_mouse_position(*pos);
        let event = InputEvent::MouseMoved(*pos, self.input_state.clone());
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
                winit::event::WindowEvent::ModifiersChanged(modifiers_state) => {
                    self.input_state.modifier_state = *modifiers_state;
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_moved(position)
                }
                _ => {}
            }
        }
    }
}
