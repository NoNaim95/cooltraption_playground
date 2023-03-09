use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::camera::controls::CameraControls;
use crate::window::event_handler::{Context, EventHandler};
use winit::dpi::PhysicalSize;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
use winit::window::{Window, WindowBuilder};

pub use self::wgpu_state::WgpuState;
pub use self::window_event_handler::WindowEventHandler;

pub mod event_handler;
mod wgpu_state;
mod window_event_handler;

pub struct EventLoopHandler {
    event_loop: EventLoop<CooltraptionEvent>,
    event_loop_proxy: EventLoopProxy<CooltraptionEvent>,
    handlers: Vec<Rc<RefCell<dyn EventHandler>>>,
    wgpu_state: WgpuState,
    window: Window,
}

#[derive(Debug, Copy, Clone)]
pub enum CooltraptionEvent {
    Init,
    Render(Duration),
    CameraControls(CameraControls),
    OpenGUI, // TODO: GuiCommand to specify window
}

impl EventLoopHandler {
    pub async fn new() -> Self {
        let event_loop = EventLoopBuilder::with_user_event().build();
        let event_loop_proxy = event_loop.create_proxy();
        let window = WindowBuilder::new()
            .with_title("Cooltraption Playground - Render Example")
            .with_inner_size(PhysicalSize::new(1200, 800))
            .with_min_inner_size(PhysicalSize::new(800, 600))
            .build(&event_loop)
            .expect("create window");

        let wgpu_state = WgpuState::new(&window).await;

        Self {
            event_loop,
            event_loop_proxy,
            handlers: vec![],
            window,
            wgpu_state,
        }
    }

    pub fn add_handler(&mut self, handler: Rc<RefCell<dyn EventHandler>>) {
        self.handlers.push(handler);
    }

    pub fn run_event_loop(mut self) {
        self.event_loop_proxy
            .send_event(CooltraptionEvent::Init)
            .expect("Send init event");

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            let mut new_event_handlers = vec![];

            for event_handler in &self.handlers {
                let mut context = Context {
                    control_flow,
                    window: &self.window,
                    wgpu_state: &mut self.wgpu_state,
                    event_loop_proxy: &self.event_loop_proxy,
                    event_handlers: &mut new_event_handlers,
                };

                event_handler
                    .borrow_mut()
                    .handle_event(&event, &mut context);
            }

            self.handlers.append(&mut new_event_handlers);
        });
    }
}
