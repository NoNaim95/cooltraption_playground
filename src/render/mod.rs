use std::sync::Mutex;
use std::time::Instant;

use bevy_ecs::prelude::*;
use log::info;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::components::{Drawable, Position};
use crate::render::wgpu_state::WgpuState;

mod camera;
mod instance;
pub mod vertex;
pub mod wgpu_state;

#[derive(StageLabel)]
pub struct RenderStage;

#[derive(Default)]
pub struct RenderWorld {
    state: Vec<(Position, Drawable)>,
}

pub struct RenderMachine {
    state: [RenderWorld; 2],
    events: UnboundedReceiver<Event<'static, ()>>,
    wgpu_state: WgpuState,
}

impl RenderMachine {
    pub async fn create_window() -> Self {
        let (window, events) = Self::open_window().await;
        let wgpu_state = WgpuState::new(&window).await;

        Self {
            state: [RenderWorld::default(), RenderWorld::default()],
            events,
            wgpu_state,
        }
    }

    async fn open_window() -> (Window, UnboundedReceiver<Event<'static, ()>>) {
        let (window_send, window_recv) = oneshot::channel::<Window>();
        let (event_send, event_recv) = mpsc::unbounded_channel::<Event<()>>();

        task::spawn(async {
            let event_loop = EventLoop::new();
            let window = Window::new(&event_loop).expect("Could not create window");

            window_send
                .send(window)
                .expect("Send window handle back to main thread");

            event_loop.run(move |event, _, _control_flow| {
                if let Some(event) = event.to_static() {
                    event_send.send(event).expect("Send event to main thread");
                }
            });
        });

        (window_recv.await.unwrap(), event_recv)
    }

    pub fn update_state(&mut self, query: Query<(&Position, &Drawable)>) {
        self.state.swap(0, 1);

        while let Ok(event) = self.events.try_recv() {
            info!("Received event: {:?}", event);
        }

        self.state[0] = RenderWorld {
            state: query.iter().map(|(p, d)| (p.clone(), d.clone())).collect(),
        };
    }

    pub fn render(&mut self) {
        // TODO: Identify different render sets and render them one by one
        for (position, drawable) in &self.state[0].state {
            self.wgpu_state.render_object(position, drawable);
        }
    }

    pub fn wgpu_state(&self) -> &WgpuState {
        &self.wgpu_state
    }

    pub fn wgpu_state_mut(&mut self) -> &mut WgpuState {
        &mut self.wgpu_state
    }
}
