use crate::components::{Drawable, Position};
use bevy_ecs::prelude::*;

#[derive(StageLabel)]
pub struct RenderStage;

/*pub fn render(query: Query<(&Position, &Drawable)>) {
    for (position, render) in &query {}
}*/

pub fn clone_state<T: Component + Clone>(query: Query<&T>) {
    for component in &query {}
}
