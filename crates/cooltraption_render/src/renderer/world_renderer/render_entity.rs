use cgmath::{Matrix4, Quaternion, Vector3};
use cooltraption_assets::texture_atlas::Rectangle;
use wgpu::BufferAddress;

#[derive(Debug)]
pub struct RenderEntity {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
    pub atlas_region: Rectangle,
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
            region_offset: self.atlas_region.min.to_array(),
            region_size: self.atlas_region.size().to_array(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderEntityRaw {
    transform: [[f32; 3]; 4],
    region_offset: [i32; 2],
    region_size: [i32; 2],
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
                    format: wgpu::VertexFormat::Sint32x2,
                    offset: mem::size_of::<[[f32; 3]; 4]>() as BufferAddress,
                    shader_location: 6,
                },
                // region size
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Sint32x2,
                    offset: mem::size_of::<[[f32; 3]; 4]>() as BufferAddress
                        + mem::size_of::<[i32; 2]>() as BufferAddress,
                    shader_location: 7,
                },
            ],
        }
    }
}
