use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::texture_atlas::TextureAtlasBuilder;

#[derive(Debug)]
pub struct SpriteAsset {
    pub texture_hash: u64,
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
                write!(f, "io error occurred during texture loading: {}", e)
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

impl SpriteAsset {
    pub fn decode(
        bytes: &[u8],
        atlas_builder: &mut TextureAtlasBuilder,
    ) -> Result<SpriteAsset, LoadTextureError> {
        let diffuse_image = image::load_from_memory(bytes)?;

        let asset = {
            let mut hasher = DefaultHasher::new();
            diffuse_image.as_bytes().hash(&mut hasher);
            let texture_hash = hasher.finish();

            SpriteAsset { texture_hash }
        };

        atlas_builder.add_texture(diffuse_image);

        Ok(asset)
    }
}
