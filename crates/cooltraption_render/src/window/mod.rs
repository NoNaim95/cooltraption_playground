use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::time::Duration;

use crate::camera::controls::CameraControls;
use crate::events::{Context, Event, EventHandler};
pub use winit;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
use winit::window::{Window, WindowBuilder};

pub use self::wgpu_state::WgpuState;
pub use self::window_event_handler::WindowEventHandler;

mod wgpu_state;
mod window_event_handler;

pub struct WinitEventLoopHandler {
    event_loop: EventLoop<WindowEvent>,
    event_loop_proxy: EventLoopProxy<WindowEvent>,
    handlers: Vec<SharedEventHandler>,
    wgpu_state: WgpuState,
    window: Window,
}

pub struct WinitEvent<'a, 'b>(pub &'a mut winit::event::Event<'b, WindowEvent>);

impl Event for WinitEvent<'_, '_> {}

pub enum WindowEvent {
    Init,
    Render(Duration),
    CameraControls(CameraControls),
    OpenGUI(Option<Box<dyn for<'a> crate::gui::GuiWindow<'a>>>),
    CloseGUI(&'static str),
}

impl Debug for WindowEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowEvent::Init => write!(f, "Init"),
            WindowEvent::Render(duration) => write!(f, "Render({:?})", duration),
            WindowEvent::CameraControls(controls) => {
                write!(f, "CameraControls({:?})", controls)
            }
            WindowEvent::OpenGUI(_) => write!(f, "OpenGUI"),
            WindowEvent::CloseGUI(id) => write!(f, "CloseGUI({})", id),
        }
    }
}
/*
impl<'s>
    EventProxy<
        's,
        winit::event::Event<'_, CooltraptionEvent>,
        WindowContext<'_>,
        SharedEventHandler,
    > for WinitEventLoopHandler
{

}*/

impl WinitEventLoopHandler {
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

    pub fn register_event_handler(&mut self, handler: SharedEventHandler) {
        self.handlers.push(handler);
    }

    pub fn run_event_loop(mut self) {
        self.event_loop_proxy
            .send_event(WindowEvent::Init)
            .expect("Send init event");

        self.event_loop.run(move |mut event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            let mut new_event_handlers = vec![];

            let mut context = WindowContext::new(
                control_flow,
                &self.window,
                &mut self.wgpu_state,
                &self.event_loop_proxy,
                &mut new_event_handlers,
            );

            let mut winit_event = WinitEvent(&mut event);

            self.handlers.iter().for_each(|handler| {
                handler
                    .borrow_mut()
                    .handle_event(&mut winit_event, &mut context);
            });

            self.handlers.append(&mut new_event_handlers);
        });
    }
}

pub struct WindowContext<'a> {
    pub control_flow: &'a mut ControlFlow,
    pub window: &'a Window,
    pub wgpu_state: &'a mut WgpuState,
    event_loop_proxy: &'a EventLoopProxy<WindowEvent>,
    event_handlers: &'a mut Vec<SharedEventHandler>,
}

impl Context for WindowContext<'_> {}

impl<'a> WindowContext<'a> {
    pub fn new(
        control_flow: &'a mut ControlFlow,
        window: &'a Window,
        wgpu_state: &'a mut WgpuState,
        event_loop_proxy: &'a EventLoopProxy<WindowEvent>,
        event_handlers: &'a mut Vec<SharedEventHandler>,
    ) -> Self {
        Self {
            control_flow,
            window,
            wgpu_state,
            event_loop_proxy,
            event_handlers,
        }
    }

    pub fn register_event_handler(&mut self, handler: SharedEventHandler) {
        self.event_handlers.push(handler);
    }

    pub fn send_event(&self, event: WindowEvent) {
        self.event_loop_proxy.send_event(event).expect("send event");
    }
}

pub type SharedEventHandler =
    Rc<RefCell<dyn for<'s, 'a, 'b, 'c> EventHandler<'s, WinitEvent<'a, 'b>, WindowContext<'c>>>>;
