use crate::world_renderer::gpu_texture_atlas::GpuTextureAtlas;
use crate::world_renderer::interpolator::Transform;
use cgmath::{Matrix4, Quaternion, Rad, Rotation3, Vector3};
use cooltraption_assets::asset_bundle::{Asset, AssetBundle};
use wgpu::BufferAddress;

#[derive(Debug)]
pub struct RenderEntity {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
    pub texture_index: usize,
}

impl RenderEntity {
    pub(crate) fn to_raw(&self) -> RenderEntityRaw {
        let transform: [[f32; 4]; 4] = (Matrix4::from_translation(self.position)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
            * Matrix4::from(self.rotation))
        .into();

        let transform: [[f32; 3]; 4] = transform.map(|row| [row[0], row[1], row[2]]); // Cut last row that is always [0, 0, 0, 1]

        RenderEntityRaw {
            transform,
            texture_index: self.texture_index as u32,
        }
    }

    pub fn try_from(
        transform: &Transform,
        asset_name: &str,
        texture_atlas_resource: &GpuTextureAtlas,
        assets: &AssetBundle,
    ) -> Option<Self> {
        if let Some(Asset::Sprite(asset)) = assets.get_asset(asset_name) {
            let texture_index = texture_atlas_resource.get_texture_index(asset.texture_hash)?;
            let pos = &transform.position;
            let scale = &transform.scale;
            let rot = &transform.rot;

            Some(RenderEntity {
                position: Vector3::new(pos.0.x, pos.0.y, 0.0),
                scale: Vector3::new(scale.0.x, scale.0.y, 1.0),
                rotation: Quaternion::from_angle_z(Rad(rot.0)),
                texture_index,
            })
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderEntityRaw {
    transform: [[f32; 3]; 4],
    texture_index: u32,
}

impl RenderEntityRaw {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // transformation matrix
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 2 * mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 3 * mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 5,
                },
                // region offset
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: mem::size_of::<[[f32; 3]; 4]>() as BufferAddress,
                    shader_location: 6,
                },
            ],
        }
    }
}
