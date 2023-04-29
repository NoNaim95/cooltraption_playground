use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::events::EventHandler;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use winit::event::Event;
use winit::window::Window;

pub use crate::renderer::gui::gui_window::GuiWindow;
use crate::renderer::wgpu_state::WgpuState;
use crate::renderer::{BoxedRenderer, RenderFrame, Renderer, RendererInitializer};
use crate::window::{WindowContext, WindowEvent, WinitEvent};

pub mod debug_window;
mod gui_window;

type PlatformRef = Arc<Mutex<Option<Platform>>>;
type WindowsMapRef = Arc<Mutex<HashMap<&'static str, Box<dyn GuiWindow>>>>;

struct GuiRenderer {
    start_time: Instant,
    render_pass: RenderPass,
    platform: PlatformRef,
    windows: WindowsMapRef,
}

pub struct GuiEventHandler {
    platform: PlatformRef,
    windows: WindowsMapRef,
}

pub struct GuiInitializer {
    platform: PlatformRef,
    windows: WindowsMapRef,
}

impl GuiInitializer {
    pub fn new() -> (Self, GuiEventHandler) {
        let platform = PlatformRef::default();
        let windows = WindowsMapRef::default();

        (
            Self {
                platform: platform.clone(),
                windows: windows.clone(),
            },
            GuiEventHandler { platform, windows },
        )
    }
}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for GuiEventHandler {
    fn handle_event(&mut self, event: &mut WinitEvent<'_, '_>, context: &mut WindowContext<'_>) {
        if let Some(platform) = &mut *self.platform.lock().expect("lock platform") {
            platform.handle_event(event.0);

            if let Event::UserEvent(event) = event.0 {
                match event {
                    WindowEvent::OpenGUI(window) => match window.take() {
                        None => {}
                        Some(window) => {
                            self.windows
                                .lock()
                                .expect("lock windows")
                                .insert(window.id(), window);
                        }
                    },
                    WindowEvent::CloseGUI(id) => {
                        self.windows.lock().expect("lock windows").remove(id);
                    }
                    _ => {}
                }
            }
        }

        for window in self.windows.lock().expect("lock windows").values_mut() {
            window.handle_event(event, context);
        }
    }
}

impl Renderer for GuiRenderer {
    fn render(&mut self, render_frame: &mut RenderFrame) {
        if let Some(platform) = &mut *self.platform.lock().expect("lock platform") {
            // Begin to draw the UI frame.
            platform.update_time(self.start_time.elapsed().as_secs_f64());
            platform.begin_frame();

            // Draw all ui elements
            for window in self.windows.lock().expect("lock windows").values_mut() {
                window.show(&platform.context());
            }

            // End the UI frame. We could now handle the output and draw the UI with the backend.
            let full_output = platform.end_frame(Some(render_frame.window));
            let paint_jobs = platform.context().tessellate(full_output.shapes);

            // Upload all resources for the GPU.
            let screen_descriptor = ScreenDescriptor {
                physical_width: render_frame.window.inner_size().width,
                physical_height: render_frame.window.inner_size().height,
                scale_factor: render_frame.window.scale_factor() as f32,
            };
            let t_delta: egui::TexturesDelta = full_output.textures_delta;
            self.render_pass
                .add_textures(render_frame.device, render_frame.queue, &t_delta)
                .expect("add texture ok");
            self.render_pass.update_buffers(
                render_frame.device,
                render_frame.queue,
                &paint_jobs,
                &screen_descriptor,
            );

            // Record all render passes.
            self.render_pass
                .execute(
                    &mut render_frame.encoder,
                    &render_frame.view,
                    &paint_jobs,
                    &screen_descriptor,
                    None,
                )
                .unwrap();

            self.render_pass
                .remove_textures(t_delta)
                .expect("remove texture ok");
        }
    }
}

impl RendererInitializer for GuiInitializer {
    fn init(self: Box<Self>, wgpu_state: &mut WgpuState, window: &Window) -> BoxedRenderer {
        *self.platform.lock().expect("lock platform") = Some(Platform::new(PlatformDescriptor {
            physical_width: wgpu_state.size.width,
            physical_height: wgpu_state.size.height,
            scale_factor: window.scale_factor(),
            font_definitions: Default::default(),
            style: Default::default(),
        }));

        Box::new(GuiRenderer {
            start_time: Instant::now(),
            platform: self.platform,
            render_pass: RenderPass::new(&wgpu_state.device, wgpu_state.config.format, 1),
            windows: self.windows,
        })
    }
}
