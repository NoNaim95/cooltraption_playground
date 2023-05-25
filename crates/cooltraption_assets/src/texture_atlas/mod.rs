use std::collections::HashMap;

pub use guillotiere::Rectangle;
use image::RgbaImage;

pub use texture_atlas_builder::TextureAtlasBuilder;

mod texture_atlas_builder;

pub struct TextureAtlas {
    rgba: RgbaImage,
    region_map: HashMap<u64, Rectangle>,
}

impl TextureAtlas {
    pub fn rgba(&self) -> &RgbaImage {
        &self.rgba
    }

    pub fn region_map(&self) -> &HashMap<u64, Rectangle> {
        &self.region_map
    }

    pub fn get_texture_region(&self, texture_hash: u64) -> Option<&Rectangle> {
        self.region_map.get(&texture_hash)
    }
}
