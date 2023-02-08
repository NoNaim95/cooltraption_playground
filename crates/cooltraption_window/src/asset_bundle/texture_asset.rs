use crate::asset_bundle::Asset;
use crate::render::texture_atlas_builder::TextureAtlasBuilder;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TextureAsset {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

#[derive(Debug)]
pub enum LoadTextureError {
    IOError(std::io::Error),
    DecodeError(image::ImageError),
}

impl Display for LoadTextureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadTextureError::IOError(e) => {
                write!(f, "io error occured during texture loading: {}", e)
            }
            LoadTextureError::DecodeError(e) => {
                write!(f, "could not decode texture from file: {}", e)
            }
        }
    }
}

impl Error for LoadTextureError {}

impl From<std::io::Error> for LoadTextureError {
    fn from(e: std::io::Error) -> Self {
        LoadTextureError::IOError(e)
    }
}

impl From<image::ImageError> for LoadTextureError {
    fn from(e: image::ImageError) -> Self {
        LoadTextureError::DecodeError(e)
    }
}

impl Asset for TextureAsset {}

impl TextureAsset {
    pub fn load(
        path: PathBuf,
        atlas_builder: &TextureAtlasBuilder,
    ) -> Result<Self, LoadTextureError> {
        let diffuse_bytes = fs::read(path)?;
        let diffuse_image = image::load_from_memory(diffuse_bytes.as_slice())?;
        let diffuse_rgba = diffuse_image.to_rgba8();

        let texture_size = wgpu::Extent3d {
            width: diffuse_image.width(),
            height: diffuse_image.height(),
            depth_or_array_layers: 1,
        };

        // TODO: Make properties configurable using yaml
        let diffuse_texture = atlas_builder
            .device()
            .create_texture(&wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
            });

        atlas_builder.queue().write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * diffuse_image.width()),
                rows_per_image: std::num::NonZeroU32::new(diffuse_image.height()),
            },
            texture_size,
        );

        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = atlas_builder
            .device()
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

        Ok(Self {
            texture: diffuse_texture,
            view: diffuse_texture_view,
            sampler: diffuse_sampler,
        })
    }
}
