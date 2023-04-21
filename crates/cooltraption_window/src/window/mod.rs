use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::time::Duration;

use crate::camera::controls::CameraControls;
use crate::render::render_frame::RenderFrame;
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

pub enum CooltraptionEvent {
    Init,
    Render(Duration),
    CameraControls(CameraControls),
    OpenGUI(Option<Box<dyn crate::gui::GuiWindow>>),
    CloseGUI(&'static str),
}

impl Debug for CooltraptionEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CooltraptionEvent::Init => write!(f, "Init"),
            CooltraptionEvent::Render(duration) => write!(f, "Render({:?})", duration),
            CooltraptionEvent::CameraControls(controls) => {
                write!(f, "CameraControls({:?})", controls)
            }
            CooltraptionEvent::OpenGUI(_) => write!(f, "OpenGUI"),
            CooltraptionEvent::CloseGUI(id) => write!(f, "CloseGUI({})", id),
        }
    }
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

        self.event_loop.run(move |mut event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            let mut new_event_handlers = vec![];

            let mut context = Context::new(
                control_flow,
                &self.window,
                &mut self.wgpu_state,
                &self.event_loop_proxy,
                &mut new_event_handlers,
            );

            self.handlers.iter().for_each(|handler| {
                handler.borrow_mut().handle_event(&mut event, &mut context);
            });

            self.handlers.append(&mut new_event_handlers);
        });
    }
}
