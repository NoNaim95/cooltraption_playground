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

        for (position, drawable) in query.iter() {
            self.current
                .state
                .push((position.clone(), drawable.clone()))
        }
    }

    pub fn render(&self) {
        // draw calls to wgpu
    }
}
