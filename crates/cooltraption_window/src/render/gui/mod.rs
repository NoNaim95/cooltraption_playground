use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use winit::event::Event;
use winit::window::Window;

pub use crate::render::gui::gui_window::{GuiWindow, UiState};
use crate::render::{RenderFrame, Renderer};
use crate::window::WgpuState;
use crate::{Context, CooltraptionEvent, EventHandler};

pub mod debug_window;
mod gui_window;

struct RenderState {
    platform: Platform,
    render_pass: RenderPass,
}

#[derive(Default)]
pub struct Gui {
    render_state: Option<RenderState>,
    windows: Vec<Box<dyn GuiWindow>>,
}

impl Gui {
    pub fn add_window(&mut self, window: Box<dyn GuiWindow>) {
        self.windows.push(window);
    }
}

impl EventHandler for Gui {
    fn handle_event(&mut self, event: &Event<CooltraptionEvent>, _context: &mut Context) {
        if let Some(render_state) = &mut self.render_state {
            render_state.platform.handle_event(event)
        }
    }
}

impl Renderer for Gui {
    fn init(&mut self, window: &Window, wgpu_state: &WgpuState) {
        let render_state = RenderState {
            platform: Platform::new(PlatformDescriptor {
                physical_width: window.inner_size().width,
                physical_height: window.inner_size().height,
                scale_factor: window.scale_factor(),
                font_definitions: Default::default(),
                style: Default::default(),
            }),
            render_pass: RenderPass::new(&wgpu_state.device, wgpu_state.config.format, 1),
        };

        self.render_state = Some(render_state);
    }

    fn render(&mut self, render_frame: &mut RenderFrame) {
        if let Some(render_state) = &mut self.render_state {
            // Begin to draw the UI frame.
            render_state.platform.update_time(0.01);
            render_state.platform.begin_frame();

            // Draw all ui elements
            self.windows.retain_mut(|window| {
                matches!(
                    window.show(&render_state.platform.context()),
                    UiState::KeepOpen
                )
            });

            // End the UI frame. We could now handle the output and draw the UI with the backend.
            let full_output = render_state.platform.end_frame(Some(render_frame.window));
            let paint_jobs = render_state
                .platform
                .context()
                .tessellate(full_output.shapes);

            // Upload all resources for the GPU.
            let screen_descriptor = ScreenDescriptor {
                physical_width: render_frame.window.inner_size().width,
                physical_height: render_frame.window.inner_size().height,
                scale_factor: render_frame.window.scale_factor() as f32,
            };
            let t_delta: egui::TexturesDelta = full_output.textures_delta;
            render_state
                .render_pass
                .add_textures(render_frame.device, render_frame.queue, &t_delta)
                .expect("add texture ok");
            render_state.render_pass.update_buffers(
                render_frame.device,
                render_frame.queue,
                &paint_jobs,
                &screen_descriptor,
            );

            // Record all render passes.
            render_state
                .render_pass
                .execute(
                    &mut render_frame.encoder,
                    &render_frame.view,
                    &paint_jobs,
                    &screen_descriptor,
                    None,
                )
                .unwrap();

            render_state
                .render_pass
                .remove_textures(t_delta)
                .expect("remove texture ok");
        }
    }
}
