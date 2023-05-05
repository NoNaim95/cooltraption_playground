use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use guillotiere::{dump_svg, size2, AllocId, Allocation, AtlasAllocator, Size};
use image::{DynamicImage, GenericImage, RgbaImage};
use log::info;

use super::TextureAtlas;

pub struct TextureAtlasBuilder {
    atlas_allocator: AtlasAllocator,
    alloc_map: HashMap<AllocId, DynamicImage>,
}

impl Default for TextureAtlasBuilder {
    fn default() -> Self {
        Self {
            atlas_allocator: AtlasAllocator::new(size2(512, 512)),
            alloc_map: HashMap::new(),
        }
    }
}

impl TextureAtlasBuilder {
    pub fn add_texture(&mut self, texture: DynamicImage) {
        let texture_size = size2(texture.width() as i32, texture.height() as i32);
        let alloc = self.alloc_size(texture_size);
        self.alloc_map.insert(alloc.id, texture);
    }

    fn alloc_size(&mut self, size: Size) -> Allocation {
        match self.atlas_allocator.allocate(size) {
            None => {
                let min_size = self.atlas_allocator.size().min(size);

                // TODO: Maybe use grow_and_rearrange and handle ChangeList
                // Grows the atlas vertically and set the width to the minimum required width to fit all textures
                self.atlas_allocator
                    .grow(size2(min_size.width, min_size.height + size.height));

                self.alloc_size(size)
            }
            Some(alloc) => alloc,
        }
    }

    pub fn build(&self) -> TextureAtlas {
        let mut rgba = {
            let (width, height) = self.atlas_allocator.size().into();
            RgbaImage::new(width as u32, height as u32)
        };

        for (id, texture) in &self.alloc_map {
            let region = self.atlas_allocator[*id];
            rgba.copy_from(texture, region.min.x as u32, region.min.y as u32)
                .expect("copy texture to allocated region in texture atlas");
        }

        #[cfg(feature = "debug")]
        {
            rgba.save(PathBuf::from("atlas.png")).unwrap();
        }

        let mut texture_map = HashMap::new();
        for (alloc, texture) in &self.alloc_map {
            let texture_hash = {
                let mut hasher = DefaultHasher::new();
                texture.as_bytes().hash(&mut hasher);
                hasher.finish()
            };
            texture_map.insert(texture_hash, self.atlas_allocator[*alloc]);
        }

        TextureAtlas {
            rgba,
            region_map: texture_map,
        }
    }
}
