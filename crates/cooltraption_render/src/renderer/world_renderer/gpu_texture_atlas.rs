use cooltraption_assets::texture_atlas::{Rectangle, TextureAtlas};
use std::collections::HashMap;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;

/// A GPU allocated texture atlas
///
/// Created using a [`TextureAtlas`]
pub struct GpuTextureAtlas {
    index_map: HashMap<u64, usize>, // Maps a texture hash to an index in the regions vector in the shader
    bind_groups: BindGroups,
}

impl GpuTextureAtlas {
    /// Allocate a texture atlas on the GPU
    pub fn allocate(
        texture_atlas: TextureAtlas,
        device: &Device,
        queue: &Queue,
    ) -> (Self, BindGroupLayouts) {
        let texture_size = Extent3d {
            width: texture_atlas.rgba().width(),
            height: texture_atlas.rgba().height(),
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: Some("atlas_texture"),
            view_formats: &[TextureFormat::Rgba8UnormSrgb],
        });

        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            texture_atlas.rgba(),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * texture_atlas.rgba().width()),
                rows_per_image: std::num::NonZeroU32::new(texture_atlas.rgba().height()),
            },
            texture_size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: false },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                        count: None,
                    },
                ],
                label: Some("atlas_texture_bind_group_layout"),
            });

        let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("atlas_texture_bind_group"),
        });

        let (index_map, region_bind_group_layout, region_bind_group) =
            texture_atlas_to_buffer(&texture_atlas, device);

        (
            Self {
                index_map,
                bind_groups: BindGroups {
                    texture: texture_bind_group,
                    region: region_bind_group,
                },
            },
            BindGroupLayouts {
                texture: texture_bind_group_layout,
                region: region_bind_group_layout,
            },
        )
    }

    pub fn get_texture_index(&self, texture_hash: u64) -> Option<usize> {
        self.index_map.get(&texture_hash).copied()
    }

    pub fn region_bind_group(&self) -> &BindGroup {
        &self.bind_groups.region
    }

    pub fn texture_bind_group(&self) -> &BindGroup {
        &self.bind_groups.texture
    }
}

pub struct BindGroupLayouts {
    pub texture: BindGroupLayout,
    pub region: BindGroupLayout,
}

struct BindGroups {
    texture: BindGroup,
    region: BindGroup,
}

fn texture_atlas_to_buffer(
    texture_atlas: &TextureAtlas,
    device: &Device,
) -> (HashMap<u64, usize>, BindGroupLayout, BindGroup) {
    let mut regions: Vec<RegionRaw> = Vec::with_capacity(texture_atlas.region_map().len());
    let mut index_map: HashMap<u64, usize> = HashMap::new();

    for (index, (texture_hash, region)) in texture_atlas.region_map().iter().enumerate() {
        regions.push((*region).into());
        index_map.insert(*texture_hash, index);
    }

    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("region_buffer"),
        contents: bytemuck::cast_slice(&regions),
        usage: BufferUsages::STORAGE,
    });

    let buffer_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("region_bind_group_layout"),
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        layout: &buffer_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
        label: Some("region_bind_group"),
    });

    (index_map, buffer_bind_group_layout, bind_group)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RegionRaw {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl From<Rectangle> for RegionRaw {
    fn from(rect: Rectangle) -> Self {
        Self {
            x: rect.min.x,
            y: rect.min.y,
            width: rect.width(),
            height: rect.height(),
        }
    }
}
