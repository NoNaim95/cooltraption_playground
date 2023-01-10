use crate::components::{Position, Render};
use bevy_ecs::prelude::*;

#[derive(StageLabel)]
pub struct RenderStage;

pub fn render(query: Query<(&Position, &Render)>) {
    for (position, render) in &query {}
}
