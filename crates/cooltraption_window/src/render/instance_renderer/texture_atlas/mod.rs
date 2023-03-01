use std::collections::HashMap;

use guillotiere::Rectangle;
use wgpu::{Sampler, Texture, TextureView};

pub use texture_atlas_builder::TextureAtlasBuilder;

mod texture_atlas_builder;

pub struct TextureAtlas {
    texture: Texture,
    view: TextureView,
    sampler: Sampler,
    texture_map: HashMap<u64, Rectangle>,
}

impl TextureAtlas {
    pub fn texture(&self) -> &Texture {
        &self.texture
    }
    pub fn view(&self) -> &TextureView {
        &self.view
    }
    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }

    pub fn get_texture_region(&self, texture_hash: u64) -> Option<&Rectangle> {
        self.texture_map.get(&texture_hash)
    }
}
