use crate::controls::{ButtonMap, KeyboardState, MouseState};
use crate::debug_widget::DebugWidget;
use cgmath::num_traits::*;
use cgmath::*;
use cooltraption_render::gui::{GuiActionDispatcher, WidgetId};
use cooltraption_render::world_renderer::camera::controls::*;
use cooltraption_render::world_renderer::gizmos::{BoundingBox, Color, Origin, Shape};
use cooltraption_render::{ellipse, unique_id};
use cooltraption_window::events::EventHandler;
use cooltraption_window::window::winit::event::{
    ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode,
};
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
}

impl Controller {
    pub fn new(gui: GuiActionDispatcher) -> (Self, InputStateEventHandler) {
        let (send, recv) = std::sync::mpsc::channel();

        let controller = Controller { recv };
        let event_handler = InputStateEventHandler {
            keyboard_state: Default::default(),
            mouse_state: Default::default(),
            gui,
            debug_widget: None,
            target_pos: Point2::origin(),
            target_zoom: 0.25,
            view: Default::default(),
            send,
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

        let move_speed = 0.5 * delta_time.as_secs_f32();
        let move_hardness = 25.0 * delta_time.as_secs_f32();
        let zoom_speed = 0.2;
        let zoom_hardness = 35.0 * delta_time.as_secs_f32();

        let mouse_pos = self.mouse_state.pos();
        ellipse!(
            BoundingBox::Sized(Origin::Center(mouse_pos.into()), (0.1, 0.1)),
            Color::MAGENTA
        );

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

        self.target_pos += move_vec;
        self.view.position =
            self.view.position + (self.target_pos - self.view.position) * move_hardness;

        self.target_zoom *= 2.0_f32.pow(self.mouse_state.scroll() * zoom_speed);
        self.view.zoom = (self.view.zoom.ln()
            + (self.target_zoom.ln() - self.view.zoom.ln()) * zoom_hardness)
            .exp();

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
                        let window_pos = Point2::new(position.x as f32, position.y as f32);
                        let window_size = Vector2::new(
                            context.window.inner_size().width as f32,
                            context.window.inner_size().height as f32,
                        );
                        let world_pos = self.view.world_pos(window_pos, window_size);
                        self.mouse_state.set_pos(world_pos);
                    }
                    winit::event::WindowEvent::MouseInput { state, button, .. } => {
                        self.mouse_state
                            .set_btn(button, state == &ElementState::Pressed);

                        if button == &MouseButton::Left && state == &ElementState::Pressed {
                            self.target_pos = self.mouse_state.pos();
                        }

                        ellipse!(
                            BoundingBox::Sized(
                                Origin::Center(self.mouse_state.pos().into()),
                                (0.15, 0.15),
                            ),
                            Color::GREEN
                        );
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
