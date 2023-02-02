use bevy_ecs::prelude::*;
use winit::event_loop::{EventLoop, EventLoopBuilder};
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
    window: Window,
    wgpu_state: WgpuState,
}

impl RenderMachine {
    pub async fn create_window() -> (Self, EventLoop<()>) {
        let event_loop = EventLoopBuilder::new().build();
        let window = Window::new(&event_loop).expect("Could not create window");
        let wgpu_state = WgpuState::new(&window).await;

        (
            Self {
                state: [RenderWorld::default(), RenderWorld::default()],
                window,
                wgpu_state,
            },
            event_loop,
        )
    }

    pub fn update_state(&mut self, query: Query<(&Position, &Drawable)>) {
        self.state.swap(0, 1);

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
