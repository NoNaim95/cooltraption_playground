use crate::asset_bundle::texture_asset::TextureAsset;
use guillotiere::euclid::default::Size2D;
use guillotiere::{size2, AllocId, AtlasAllocator};
use std::collections::HashMap;
use wgpu::{Device, Queue};

pub struct TextureAtlasBuilder<'a> {
    device: &'a mut Device,
    queue: &'a mut Queue,
    atlas_allocator: AtlasAllocator,
    texture_map: HashMap<AllocId, &'a TextureAsset>,
}

impl<'a> TextureAtlasBuilder<'a> {
    pub fn new(device: &'a mut Device, queue: &'a mut Queue) -> Self {
        Self {
            device,
            queue,
            atlas_allocator: AtlasAllocator::new(Size2D::new(5000, 5000)),
            texture_map: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, texture: &'a TextureAsset) {
        let alloc = self.atlas_allocator.allocate(size2(100, 100)).unwrap();
        self.texture_map.insert(alloc.id, texture);
    }

    pub fn device(&self) -> &'a mut Device {
        self.device
    }
    pub fn queue(&self) -> &'a mut Queue {
        self.queue
    }
}
