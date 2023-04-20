use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use winit::event::Event;

pub use crate::render::gui::gui_window::{GuiWindow, UiState};
use crate::render::{RenderFrame, Renderer, RendererInitializer};
use crate::window::event_handler::{Context, EventHandler};
use crate::window::CooltraptionEvent;

pub mod debug_window;
mod gui_window;

struct Gui {
    start_time: Instant,
    platform: Platform,
    render_pass: RenderPass,
    windows: HashMap<&'static str, Box<dyn GuiWindow>>,
}

pub struct GuiInitializer {}

impl Gui {
    #[allow(unused)]
    pub fn add_window(&mut self, window: Box<dyn GuiWindow>) {
        self.windows.insert(window.id(), window);
    }
}

impl EventHandler for Gui {
    fn handle_event(&mut self, event: &mut Event<CooltraptionEvent>, context: &mut Context) {
        self.platform.handle_event(event);

        if let Event::UserEvent(event) = event {
            match event {
                CooltraptionEvent::OpenGUI(window) => match window.take() {
                    None => {}
                    Some(window) => {
                        self.add_window(window);
                    }
                },
                CooltraptionEvent::CloseGUI(id) => {
                    self.windows.remove(id);
                }
                _ => {}
            }
        }

        for window in self.windows.values_mut() {
            window.handle_event(event, context);
        }
    }
}

impl Renderer for Gui {
    fn render(&mut self, render_frame: &mut RenderFrame) {
        // Begin to draw the UI frame.
        self.platform
            .update_time(self.start_time.elapsed().as_secs_f64());
        self.platform.begin_frame();

        // Draw all ui elements
        self.windows.retain(|_id, window| {
            matches!(window.show(&self.platform.context()), UiState::KeepOpen)
        });

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let full_output = self.platform.end_frame(Some(render_frame.window));
        let paint_jobs = self.platform.context().tessellate(full_output.shapes);

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

impl RendererInitializer for GuiInitializer {
    fn init(self: Box<Self>, context: &mut Context) -> Rc<RefCell<dyn Renderer>> {
        let gui = Rc::new(RefCell::new(Gui {
            start_time: Instant::now(),
            platform: Platform::new(PlatformDescriptor {
                physical_width: context.window.inner_size().width,
                physical_height: context.window.inner_size().height,
                scale_factor: context.window.scale_factor(),
                font_definitions: Default::default(),
                style: Default::default(),
            }),
            render_pass: RenderPass::new(
                &context.wgpu_state.device,
                context.wgpu_state.config.format,
                1,
            ),
            windows: HashMap::new(),
        }));

        context.event_handlers.push(gui.clone());

        gui
    }
}
