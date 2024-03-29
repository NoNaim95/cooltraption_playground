mod keyboard_state;
mod mouse_state;

use crate::input::keyboard_state::KeyboardState;
use crate::input::mouse_state::MouseState;
use cooltraption_window::events::EventHandler;
use cooltraption_window::window::winit::dpi::PhysicalPosition;
use cooltraption_window::window::winit::event::{
    ElementState, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode,
};
use cooltraption_window::window::{winit, WindowContext, WinitEvent};
use winit::dpi::PhysicalSize;

type InputEventCallback = Box<dyn FnMut(&InputEvent, &InputState)>;

#[derive(Default)]
pub struct InputEventHandler {
    callbacks: Vec<InputEventCallback>,
    input_state: InputState,
}

#[derive(Debug, Default, Clone)]
pub struct InputState {
    pub keyboard_state: KeyboardState,
    pub mouse_state: MouseState,
    pub modifier_state: ModifiersState,
    pub window_size: PhysicalSize<u32>,
}

#[derive(Debug)]
pub enum InputEvent {
    KeyboardInputEvent(KeyboardInputEvent),
    MouseButtonEvent(MouseButtonEvent),
    MouseMoved(PhysicalPosition<f64>),
}

#[derive(Debug)]
pub enum KeyboardInputEvent {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
}

#[derive(Debug)]
pub enum MouseButtonEvent {
    KeyPressed(MouseButton),
    KeyReleased(MouseButton),
}

impl InputEventHandler {
    pub fn new(callbacks: Vec<InputEventCallback>) -> Self {
        Self {
            callbacks,
            input_state: InputState::default(),
        }
    }

    fn keyboard_input(&mut self, input: &mut KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            let event = match input.state {
                ElementState::Pressed => {
                    self.input_state.keyboard_state.set_button(&key_code, true);
                    InputEvent::KeyboardInputEvent(KeyboardInputEvent::KeyPressed(key_code))
                }
                ElementState::Released => {
                    self.input_state.keyboard_state.set_button(&key_code, false);
                    InputEvent::KeyboardInputEvent(KeyboardInputEvent::KeyReleased(key_code))
                }
            };
            for callback in &mut self.callbacks {
                callback(&event, &self.input_state);
            }
        }
    }

    fn mouse_input(&mut self, button: MouseButton, state: &mut ElementState) {
        let event = match state {
            ElementState::Pressed => {
                self.input_state.mouse_state.set_button(&button, true);
                InputEvent::MouseButtonEvent(MouseButtonEvent::KeyPressed(button))
            }
            ElementState::Released => {
                self.input_state.mouse_state.set_button(&button, false);
                InputEvent::MouseButtonEvent(MouseButtonEvent::KeyReleased(button))
            }
        };
        for callback in &mut self.callbacks {
            callback(&event, &self.input_state);
        }
    }

    fn mouse_moved(&mut self, pos: &mut PhysicalPosition<f64>, dimensions: PhysicalSize<u32>) {
        self.input_state.mouse_state.set_mouse_position(*pos);
        self.input_state.window_size = dimensions;
        let event = InputEvent::MouseMoved(*pos);
        for callback in &mut self.callbacks {
            callback(&event, &self.input_state);
        }
    }
}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for InputEventHandler {
    fn handle_event(&mut self, event: &mut WinitEvent, context: &mut WindowContext) {
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
                    self.mouse_moved(position, context.window.inner_size())
                }
                _ => {}
            }
        }
    }
}
