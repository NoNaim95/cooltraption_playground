use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use guillotiere::{size2, AllocId, Allocation, AtlasAllocator, Size};
use image::{DynamicImage, GenericImageView, RgbaImage};
use wgpu::{Device, Queue};

use crate::render::texture_atlas::TextureAtlas;

pub struct TextureAtlasBuilder<'a> {
    device: &'a mut Device,
    queue: &'a mut Queue,
    atlas_allocator: AtlasAllocator,
    alloc_map: HashMap<AllocId, DynamicImage>,
}

impl<'a> TextureAtlasBuilder<'a> {
    pub fn new(device: &'a mut Device, queue: &'a mut Queue) -> Self {
        Self {
            device,
            queue,
            atlas_allocator: AtlasAllocator::new(size2(1000, 1000)),
            alloc_map: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, texture: DynamicImage) {
        let texture_size = size2(texture.width() as i32, texture.height() as i32);
        let alloc = self.alloc_size(texture_size);
        self.alloc_map.insert(alloc.id, texture);
    }

    fn alloc_size(&mut self, size: Size) -> Allocation {
        match self.atlas_allocator.allocate(size) {
            None => {
                let new_size = self.atlas_allocator.size().max(size);
                // resize to make sufficient space vertically in the texture atlas
                // TODO: Use resize_and_rearrange and handle ChangeList correctly
                self.atlas_allocator.grow(new_size + size2(0, size.height));

                self.alloc_size(size)
            }
            Some(alloc) => alloc,
        }
    }

    pub fn build(&self) -> TextureAtlas {
        let mut atlas_rgba = {
            let (width, height) = self.atlas_allocator.size().into();
            RgbaImage::new(width as u32, height as u32)
        };

        // TODO: Use bulk copy operations
        for (id, texture) in &self.alloc_map {
            let region = self.atlas_allocator[*id];
            for source_x in 0..texture.width() {
                for source_y in 0..texture.height() {
                    atlas_rgba.put_pixel(
                        source_x + region.min.x as u32,
                        source_y + region.min.y as u32,
                        texture.get_pixel(source_x, source_y),
                    );
                }
            }
        }

        atlas_rgba.save(PathBuf::from("atlas.png")).unwrap();

        let texture_size = wgpu::Extent3d {
            width: atlas_rgba.width(),
            height: atlas_rgba.height(),
            depth_or_array_layers: 1,
        };

        // TODO: Make properties configurable using yaml
        let diffuse_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &atlas_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * atlas_rgba.width()),
                rows_per_image: std::num::NonZeroU32::new(atlas_rgba.height()),
            },
            texture_size,
        );

        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

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
            texture: diffuse_texture,
            view: diffuse_texture_view,
            sampler: diffuse_sampler,
            texture_map,
        }
    }
}
