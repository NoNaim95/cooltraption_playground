mod camera;
mod instance;
pub mod vertex;
pub mod wgpu_state;

use crate::components::{Drawable, Position};
use bevy_ecs::prelude::*;
use std::mem;

#[derive(StageLabel)]
pub struct RenderStage;

#[derive(Default)]
pub struct RenderMachine {
    previous: RenderWorld,
    current: RenderWorld,
}

#[derive(Default)]
pub struct RenderWorld {
    state: Vec<(Position, Drawable)>,
}

impl RenderMachine {
    pub fn update_state(&mut self, query: Query<(&Position, &Drawable)>) {
        self.previous = mem::take(&mut self.current);

        self.current = RenderWorld {
            state: query.iter().map(|(p, d)| (p.clone(), d.clone())).collect(),
        };
    }

    pub fn render(&self) {
        // draw calls to wgpu
    }
}
