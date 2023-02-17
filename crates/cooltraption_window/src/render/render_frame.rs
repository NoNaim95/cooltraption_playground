use crate::render::wgpu_state::WgpuState;
use wgpu::{
    CommandEncoder, CommandEncoderDescriptor, Device, Queue, SurfaceError, SurfaceTexture,
    TextureView, TextureViewDescriptor,
};

pub struct RenderFrame<'a> {
    pub device: &'a Device,
    pub queue: &'a Queue,
    output: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder,
}

impl<'a> RenderFrame<'a> {
    pub fn new(wgpu_state: &'a WgpuState) -> Result<Self, SurfaceError> {
        let output = wgpu_state.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let encoder = wgpu_state
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        Ok(Self {
            device: &wgpu_state.device,
            queue: &wgpu_state.queue,
            output,
            view,
            encoder,
        })
    }

    pub fn present(self) {
        self.queue.submit(std::iter::once(self.encoder.finish()));
        self.output.present();
    }
}
