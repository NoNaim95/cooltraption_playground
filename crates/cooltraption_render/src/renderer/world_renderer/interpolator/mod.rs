use crate::world_renderer::interpolator::time::Time;
use cooltraption_assets::asset_bundle::AssetBundle;
use std::collections::BTreeMap;
use std::time::Duration;

use crate::world_renderer::gpu_texture_atlas::GpuTextureAtlas;
pub use drawable::*;

use super::RenderEntity;

mod drawable;
pub mod render_entity;
mod time;

/// Interpolates two world states and creates render entities from the interpolated state
pub struct DrawableInterpolator<I: Iterator<Item = Vec<Drawable>>> {
    state_recv: I,
    time: Time,
    drawables: [BTreeMap<Id, Drawable>; 2],
    render_entities: Vec<RenderEntity>,
}

impl<I: Iterator<Item = Vec<Drawable>>> DrawableInterpolator<I> {
    pub fn new(state_recv: I, fixed_delta_time: Duration) -> Self {
        Self {
            state_recv,
            time: Time::new(fixed_delta_time),
            drawables: Default::default(),
            render_entities: Default::default(),
        }
    }

    /// Interpolates the current state with the previous state and returns the interpolated render entities
    pub fn interpolate(&mut self, texture_atlas_resource: &GpuTextureAtlas, assets: &AssetBundle) {
        while let Some(drawables) = self.state_recv.next() {
            self.update(drawables);
        }

        let amount = self.time.alpha();
        self.create_render_entities(texture_atlas_resource, assets, amount);
    }

    fn create_render_entities(
        &mut self,
        texture_atlas_resource: &GpuTextureAtlas,
        assets: &AssetBundle,
        amount: f32,
    ) {
        self.render_entities = self
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
                         // This prevents entities like bullets from spawning and
                         // not moving for 1 tick
                }
            })
            .collect()
    }

    fn update(&mut self, drawables: Vec<Drawable>) {
        self.time.tick();
        self.drawables.push_new(BTreeMap::from_iter(
            drawables.into_iter().map(|d| (d.id, d)),
        ));
    }

    pub fn render_entities(&self) -> &Vec<RenderEntity> {
        &self.render_entities
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
