pub mod texture_atlas_builder;

use guillotiere::Rectangle;
use std::collections::HashMap;
use wgpu::{Sampler, Texture, TextureView};

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
}
