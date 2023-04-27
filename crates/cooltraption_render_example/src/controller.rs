use cgmath::num_traits::*;
use cgmath::*;
use cooltraption_render::events::EventHandler;
use cooltraption_render::camera::controls::{
    ButtonMap, CameraController, CameraControls, KeyboardState, MouseState, VirtualKeyCode,
};
use cooltraption_render::gui::debug_window::DebugWindow;
use cooltraption_render::window::winit::event::{ElementState, MouseScrollDelta};
use cooltraption_render::window::{winit, WindowContext, WindowEvent, WinitEvent};
use std::time::Duration;

#[derive(Default)]
pub struct Controller {
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
}

impl CameraController for Controller {}

impl Controller {
    fn send_controls(&self, context: &mut WindowContext, delta_time: &Duration) {
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

        context.send_event(WindowEvent::CameraControls(controls));
    }
}

impl<'s> EventHandler<'s, WinitEvent<'_, '_>, WindowContext<'_>> for Controller {
    fn handle_event(&'s mut self, event: &mut WinitEvent, context: &mut WindowContext) {
        match event.0 {
            winit::event::Event::WindowEvent { event, window_id } => {
                if window_id != &context.window.id() {
                    return;
                }

                match event {
                    winit::event::WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(vk_code) = input.virtual_keycode {
                            self.keyboard_state
                                .set_btn(&vk_code, input.state == ElementState::Pressed);

                            if vk_code == VirtualKeyCode::F3 && input.state == ElementState::Pressed
                            {
                                context.send_event(WindowEvent::OpenGUI(Some(
                                    Box::<DebugWindow>::default(),
                                )));
                            }
                        }
                    }
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        self.mouse_state
                            .set_pos(Vector2::new(position.x, position.y));
                    }
                    winit::event::WindowEvent::MouseInput { state, button, .. } => {
                        self.mouse_state
                            .set_btn(button, *state == ElementState::Pressed);
                    }
                    winit::event::WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_x, y),
                        ..
                    } => {
                        self.mouse_state.add_scroll(*y);
                    }
                    _ => {}
                }
            }
            winit::event::Event::UserEvent(WindowEvent::Render(delta_time)) => {
                self.send_controls(context, delta_time);
                self.mouse_state.reset();
            }
            _ => {}
        }
    }
}
