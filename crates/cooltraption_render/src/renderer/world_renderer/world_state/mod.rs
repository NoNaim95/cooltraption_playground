use crate::world_renderer::world_state::time::Time;
use cooltraption_assets::asset_bundle::AssetBundle;
use cooltraption_assets::texture_atlas::TextureAtlas;
use std::collections::BTreeMap;
use std::time::Duration;

use crate::world_renderer::texture_atlas_resource::TextureAtlasResource;
pub use drawable::*;

use super::RenderEntity;

mod drawable;
pub mod render_entity;
mod time;

pub struct WorldState {
    time: Time,
    drawables: [BTreeMap<Id, Drawable>; 2],
}

impl WorldState {
    pub fn new(fixed_delta_time: Duration) -> Self {
        Self {
            time: Time::new(fixed_delta_time),
            drawables: Default::default(),
        }
    }

    pub fn get_render_entities(
        &self,
        texture_atlas_resource: &TextureAtlasResource,
        assets: &AssetBundle,
    ) -> Vec<RenderEntity> {
        let amount = self.time.alpha();

        let render_entities = self
            .drawables
            .current()
            .values()
            .filter_map(|current| {
                if let Some(previous) = self.drawables.previous().get(&current.id) {
                    let transform = previous.transform.lerp(&current.transform, amount);
                    RenderEntity::try_from(
                        &transform,
                        &current.asset_name,
                        texture_atlas_resource,
                        assets,
                    )
                } else {
                    None // Don't render entities that haven't been present before
                }
            })
            .collect();

        render_entities
    }

    pub fn update(&mut self, drawables: Vec<Drawable>) {
        self.time.tick();
        self.drawables.push_new(BTreeMap::from_iter(
            drawables.into_iter().map(|d| (d.id, d)),
        ));
    }
}

trait BiState<T> {
    fn push_new(&mut self, state: T);

    fn current(&self) -> &T;
    fn previous(&self) -> &T;
}

impl BiState<BTreeMap<Id, Drawable>> for [BTreeMap<Id, Drawable>; 2] {
    fn push_new(&mut self, state: BTreeMap<Id, Drawable>) {
        self.swap(0, 1);
        self[0] = state;
    }

    fn current(&self) -> &BTreeMap<Id, Drawable> {
        &self[0]
    }

    fn previous(&self) -> &BTreeMap<Id, Drawable> {
        &self[1]
    }
}
