use std::fmt::{Debug, Formatter};
use std::time::Duration;

use crate::events::{Context, Event, EventHandler};
pub use winit;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
use winit::window::{Window, WindowBuilder};

pub use self::window_event_handler::WindowEventHandler;

mod window_event_handler;

pub struct WinitEventLoopHandler {
    event_loop: EventLoop<WindowEvent>,
    event_loop_proxy: EventLoopProxy<WindowEvent>,
    handlers: Vec<BoxedEventHandler>,
    window: Window,
}

pub struct WinitEvent<'a, 'b>(pub &'a mut winit::event::Event<'b, WindowEvent>);

impl Event for WinitEvent<'_, '_> {}

pub enum WindowEvent {
    Init,
    Render(Duration),
}

impl Debug for WindowEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowEvent::Init => write!(f, "Init"),
            WindowEvent::Render(duration) => write!(f, "Render({:?})", duration),
        }
    }
}

impl Default for WinitEventLoopHandler {
    fn default() -> Self {
        let event_loop = EventLoopBuilder::with_user_event().build();
        let event_loop_proxy = event_loop.create_proxy();
        let window = WindowBuilder::new()
            .with_title("Cooltraption Playground - Render Example")
            .with_inner_size(PhysicalSize::new(1200, 800))
            .with_min_inner_size(PhysicalSize::new(800, 600))
            .build(&event_loop)
            .expect("create window");

        Self {
            event_loop,
            event_loop_proxy,
            handlers: vec![],
            window,
        }
    }
}

impl WinitEventLoopHandler {
    pub fn register_event_handler(&mut self, handler: BoxedEventHandler) {
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
                &self.event_loop_proxy,
                &mut new_event_handlers,
            );

            let mut winit_event = WinitEvent(&mut event);

            self.handlers.iter_mut().for_each(|handler| {
                handler.handle_event(&mut winit_event, &mut context);
            });

            self.handlers.append(&mut new_event_handlers);
        });
    }
}

pub struct WindowContext<'a> {
    pub control_flow: &'a mut ControlFlow,
    pub window: &'a Window,
    event_loop_proxy: &'a EventLoopProxy<WindowEvent>,
    event_handlers: &'a mut Vec<BoxedEventHandler>,
}

impl Context for WindowContext<'_> {}

impl<'a> WindowContext<'a> {
    pub fn new(
        control_flow: &'a mut ControlFlow,
        window: &'a Window,
        event_loop_proxy: &'a EventLoopProxy<WindowEvent>,
        event_handlers: &'a mut Vec<BoxedEventHandler>,
    ) -> Self {
        Self {
            control_flow,
            window,
            event_loop_proxy,
            event_handlers,
        }
    }

    pub fn register_event_handler(&mut self, handler: BoxedEventHandler) {
        self.event_handlers.push(handler);
    }

    pub fn send_event(&self, event: WindowEvent) {
        self.event_loop_proxy.send_event(event).expect("send event");
    }
}

pub type BoxedEventHandler =
    Box<dyn for<'a, 'b, 'c> EventHandler<WinitEvent<'a, 'b>, WindowContext<'c>>>;
