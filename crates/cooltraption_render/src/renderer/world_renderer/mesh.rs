use wgpu::util::DeviceExt;
use wgpu::{util, Buffer, BufferUsages, Device};

/// A GPU allocated mesh
pub struct Mesh {
    vertices: Buffer,
    indices: Buffer,
    num_indices: u32,
}

impl Mesh {
    pub fn quad(device: &Device) -> Mesh {
        Self::new(device, QUAD_VERTICES, QUAD_INDICES, "Quad")
    }

    pub fn new<V: bytemuck::Pod>(
        device: &Device,
        vertices: &[V],
        indices: &[u16],
        label: &'static str,
    ) -> Mesh {
        let vertex_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some(format!("{} Vertex Buffer", label).as_str()),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some(format!("{} Index Buffer", label).as_str()),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        Self {
            vertices: vertex_buffer,
            indices: index_buffer,
            num_indices,
        }
    }

    pub fn vertices(&self) -> &Buffer {
        &self.vertices
    }
    pub fn indices(&self) -> &Buffer {
        &self.indices
    }
    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

const QUAD_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
];

const QUAD_INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];
