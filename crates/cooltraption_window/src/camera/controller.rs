use cgmath::{InnerSpace, Vector2};
use num_traits::{Pow, Zero};
use winit::event::{ElementState, Event, MouseScrollDelta, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoopProxy;

use crate::camera::input_device::{ButtonState, KeyboardState, MouseState};
use crate::{Context, CooltraptionEvent, EventHandler};

#[derive(Clone, Copy, Debug)]
pub struct CameraControls {
    pub move_vec: Vector2<f32>,
    pub zoom: f32,
}

impl Default for CameraControls {
    fn default() -> Self {
        Self {
            move_vec: Vector2::zero(),
            zoom: 1.0,
        }
    }
}

#[derive(Default)]
pub struct CameraController {
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
}

impl CameraController {
    fn send_controls(&self, event_loop_proxy: &EventLoopProxy<CooltraptionEvent>) {
        let mut controls = CameraControls::default();

        let zoom_speed: f32 = 1.06;
        let move_speed = 0.01;

        controls.zoom *= zoom_speed.pow(self.mouse_state.scroll());

        if self.keyboard_state.is_down(VirtualKeyCode::W) {
            controls.move_vec.y += 1.0;
        }
        if self.keyboard_state.is_down(VirtualKeyCode::A) {
            controls.move_vec.x -= 1.0;
        }
        if self.keyboard_state.is_down(VirtualKeyCode::S) {
            controls.move_vec.y -= 1.0;
        }
        if self.keyboard_state.is_down(VirtualKeyCode::D) {
            controls.move_vec.x += 1.0;
        }

        if controls.move_vec.magnitude() > 0.0 {
            controls.move_vec = controls.move_vec.normalize_to(move_speed);
        }

        event_loop_proxy
            .send_event(CooltraptionEvent::CameraControls(controls))
            .expect("Send camera controls event");
    }
}

impl EventHandler for CameraController {
    fn handle_event(&mut self, event: &Event<CooltraptionEvent>, context: &mut Context) {
        match event {
            Event::WindowEvent { event, window_id } => {
                if window_id != &context.window.id() {
                    return;
                }

                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(vk_code) = input.virtual_keycode {
                            match input.state {
                                ElementState::Pressed => self.keyboard_state += vk_code,
                                ElementState::Released => self.keyboard_state -= vk_code,
                            }

                            if vk_code == VirtualKeyCode::F3 && input.state == ElementState::Pressed
                            {
                                context
                                    .event_loop_proxy
                                    .send_event(CooltraptionEvent::OpenGUI)
                                    .expect("Send OpenGUI event");
                            }
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.mouse_state
                            .set_pos(Vector2::new(position.x, position.y));
                    }
                    WindowEvent::MouseInput { state, button, .. } => match state {
                        ElementState::Pressed => self.mouse_state += *button,
                        ElementState::Released => self.mouse_state -= *button,
                    },
                    WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_x, y),
                        ..
                    } => {
                        self.mouse_state.add_scroll(*y);
                    }
                    _ => {}
                }
            }
            Event::UserEvent(CooltraptionEvent::Render) => {
                self.send_controls(context.event_loop_proxy);
                self.mouse_state.reset();
            }
            _ => {}
        }
    }
}
