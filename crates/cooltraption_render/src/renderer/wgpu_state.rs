use pollster::FutureExt;
use wgpu::*;
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(Debug)]
pub struct WgpuState {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
}

impl WgpuState {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        let surface = unsafe { instance.create_surface(window) }.expect("Create surface");

        let (surface, adapter, device, queue) = tokio::spawn(async move {
            let adapter = instance
                .request_adapter(&RequestAdapterOptions {
                    power_preference: PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(
                    &DeviceDescriptor {
                        features: Features::empty(),
                        // WebGL doesn't support all of wgpu's features, so if
                        // we're building for the web we'll have to disable some.
                        limits: if cfg!(target_arch = "wasm32") {
                            Limits::downlevel_webgl2_defaults()
                        } else {
                            Limits::default()
                        },
                        label: None,
                    },
                    None, // Trace path
                )
                .await
                .unwrap();

            (surface, adapter, device, queue)
        })
        .block_on()
        .expect("Request adapter and device");

        let formats: Vec<TextureFormat> = surface
            .get_capabilities(&adapter)
            .formats
            .into_iter()
            .filter(|format| {
                !matches!(
                    format,
                    TextureFormat::Rgba8Unorm
                        | TextureFormat::Rgba8UnormSrgb
                        | TextureFormat::Rgba8Snorm
                        | TextureFormat::Rgba8Uint
                        | TextureFormat::Rgba8Sint
                        | TextureFormat::Rgba16Uint
                        | TextureFormat::Rgba16Sint
                        | TextureFormat::Rgba16Unorm
                        | TextureFormat::Rgba16Snorm
                        | TextureFormat::Rgba16Float
                        | TextureFormat::Rgba32Uint
                        | TextureFormat::Rgba32Sint
                        | TextureFormat::Rgba32Float
                )
            })
            .collect();

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: formats[0],
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoNoVsync,
            //present_mode: PresentMode::AutoVsync,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: formats,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn set_size(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
