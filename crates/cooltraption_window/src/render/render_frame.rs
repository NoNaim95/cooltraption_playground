use wgpu::CommandEncoder;
use wgpu::Device;
use wgpu::Queue;
use wgpu::SurfaceTexture;
use wgpu::TextureView;
use winit::window::Window;

#[derive(Debug)]
pub struct RenderFrame<'a> {
    pub window: &'a Window,
    pub device: &'a Device,
    pub queue: &'a Queue,
    pub output: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder,
}

impl<'a> RenderFrame<'a> {
    pub fn present(self) {
        self.queue.submit(std::iter::once(self.encoder.finish()));
        self.output.present();
    }
}
