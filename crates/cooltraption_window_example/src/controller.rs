use cgmath::num_traits::*;
use cgmath::*;
use cooltraption_window::camera::controls::{
    ButtonMap, CameraController, CameraControls, KeyboardState, MouseState, VirtualKeyCode,
};
use cooltraption_window::window::event_handler::{
    Context, ElementState, Event, EventHandler, EventLoopProxy, MouseScrollDelta, WindowEvent,
};
use cooltraption_window::window::CooltraptionEvent;
use std::time::Duration;

#[derive(Default)]
pub struct Controller {
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
}

impl CameraController for Controller {}

impl Controller {
    fn send_controls(
        &self,
        event_loop_proxy: &EventLoopProxy<CooltraptionEvent>,
        delta_time: &Duration,
    ) {
        let mut controls = CameraControls::default();

        let zoom_speed = 60.0 * delta_time.as_secs_f32();
        let move_speed = 2.0 * delta_time.as_secs_f32();

        controls.zoom *= (1.0 + zoom_speed).pow(self.mouse_state.scroll());

        if self.keyboard_state.is_down(&VirtualKeyCode::W) {
            controls.move_vec.y += 1.0;
        }
        if self.keyboard_state.is_down(&VirtualKeyCode::A) {
            controls.move_vec.x -= 1.0;
        }
        if self.keyboard_state.is_down(&VirtualKeyCode::S) {
            controls.move_vec.y -= 1.0;
        }
        if self.keyboard_state.is_down(&VirtualKeyCode::D) {
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

impl EventHandler for Controller {
    fn handle_event(&mut self, event: &Event<CooltraptionEvent>, context: &mut Context) {
        match event {
            Event::WindowEvent { event, window_id } => {
                if window_id != &context.window.id() {
                    return;
                }

                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(vk_code) = input.virtual_keycode {
                            self.keyboard_state
                                .set_btn(&vk_code, input.state == ElementState::Pressed);

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
                    WindowEvent::MouseInput { state, button, .. } => {
                        self.mouse_state
                            .set_btn(button, *state == ElementState::Pressed);
                    }
                    WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_x, y),
                        ..
                    } => {
                        self.mouse_state.add_scroll(*y);
                    }
                    _ => {}
                }
            }
            Event::UserEvent(CooltraptionEvent::Render(delta_time)) => {
                self.send_controls(context.event_loop_proxy, delta_time);
                self.mouse_state.reset();
            }
            _ => {}
        }
    }
}
