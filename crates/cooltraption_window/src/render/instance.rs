use cgmath::{Matrix4, Quaternion, Vector3};
use guillotiere::Rectangle;
use wgpu::BufferAddress;

#[derive(Debug)]
pub struct Instance {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub atlas_region: Rectangle,
}

impl Instance {
    pub(crate) fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            pos_rot: (Matrix4::from_translation(self.position) * Matrix4::from(self.rotation))
                .into(),
            region_offset: self.atlas_region.min.to_array(),
            region_size: self.atlas_region.size().to_array(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pos_rot: [[f32; 4]; 4],
    region_offset: [i32; 2],
    region_size: [i32; 2],
}

impl InstanceRaw {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // pos_rot matrix
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 5,
                },
                // region matrix
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Sint32x2,
                    offset: mem::size_of::<[f32; 16]>() as BufferAddress,
                    shader_location: 6,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Sint32x2,
                    offset: mem::size_of::<[f32; 16]>() as BufferAddress
                        + mem::size_of::<[i32; 2]>() as BufferAddress,
                    shader_location: 7,
                },
            ],
        }
    }
}
