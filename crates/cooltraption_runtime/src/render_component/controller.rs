use super::controls::{ButtonMap, KeyboardState, MouseState};
use super::debug_widget::DebugWidget;
use super::CameraViewHandler;
use cgmath::num_traits::*;
use cgmath::*;
use cooltraption_render::gui::{GuiActionDispatcher, WidgetId};
use cooltraption_render::world_renderer::camera::controls::*;
use cooltraption_window::events::EventHandler;
use cooltraption_window::window::winit::event::{ElementState, MouseScrollDelta, VirtualKeyCode};
use cooltraption_window::window::{winit, WindowContext, WindowEvent, WinitEvent};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub struct Controller {
    recv: Receiver<CameraView>,
}

pub struct InputStateEventHandler {
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
    gui: GuiActionDispatcher,
    debug_widget: Option<WidgetId>,
    target_pos: Point2<f32>,
    target_zoom: f32,
    view: CameraView,
    send: Sender<CameraView>,
    camera_moved_callbacks: Vec<CameraViewHandler>,
}

impl Controller {
    pub fn new(
        gui: GuiActionDispatcher,
        camera_moved_event_publisher: Vec<CameraViewHandler>,
    ) -> (Self, InputStateEventHandler) {
        let (send, recv) = std::sync::mpsc::channel();

        let controller = Controller { recv };
        let event_handler = InputStateEventHandler {
            keyboard_state: Default::default(),
            mouse_state: Default::default(),
            gui,
            debug_widget: None,
            target_pos: Point2::origin(),
            target_zoom: 1.0,
            view: Default::default(),
            send,
            camera_moved_callbacks: camera_moved_event_publisher,
        };

        (controller, event_handler)
    }
}

impl CameraController for Controller {
    fn get_view(&self) -> Option<CameraView> {
        self.recv.try_recv().ok()
    }
}

impl InputStateEventHandler {
    fn send_controls(&mut self, delta_time: &Duration) {
        let mut move_vec = Vector2::zero();

        let move_speed = 2.0 * delta_time.as_secs_f32();
        let move_hardness = 25.0 * delta_time.as_secs_f32();
        let zoom_speed = 0.1;
        let zoom_hardness = 35.0 * delta_time.as_secs_f32();

        if self.keyboard_state.is_down(&VirtualKeyCode::W) {
            move_vec.y += 1.0;
        }
        if self.keyboard_state.is_down(&VirtualKeyCode::A) {
            move_vec.x -= 1.0;
        }
        if self.keyboard_state.is_down(&VirtualKeyCode::S) {
            move_vec.y -= 1.0;
        }
        if self.keyboard_state.is_down(&VirtualKeyCode::D) {
            move_vec.x += 1.0;
        }
        if move_vec.magnitude() > 0.0 {
            move_vec = move_vec.normalize_to(move_speed / self.view.zoom);
        }
        let old_view = self.view;
        self.target_pos += move_vec;
        self.view.position =
            self.view.position + (self.target_pos - self.view.position) * move_hardness;

        self.target_zoom *= 2.0_f32.pow(self.mouse_state.scroll() * zoom_speed);
        self.view.zoom = (self.view.zoom.ln()
            + (self.target_zoom.ln() - self.view.zoom.ln()) * zoom_hardness)
            .exp();

        if old_view != self.view {
            for callback in &mut self.camera_moved_callbacks {
                callback(&self.view);
            }
        }

        self.send.send(self.view).expect("Send controls");
    }
}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for InputStateEventHandler {
    fn handle_event(&mut self, event: &mut WinitEvent, context: &mut WindowContext) {
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
                                // Toggle debug_window
                                if let Some(debug_widget) = self.debug_widget {
                                    self.gui.close(debug_widget);
                                    self.debug_widget = None;
                                } else {
                                    self.debug_widget =
                                        Some(self.gui.open(Box::<DebugWidget>::default()));
                                }
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
                self.send_controls(delta_time);
                self.mouse_state.reset();
            }
            _ => {}
        }
    }
}
