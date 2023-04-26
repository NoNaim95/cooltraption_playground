use winit::dpi::PhysicalSize;
use winit::event::Event;

use crate::renderer::gui::GuiWindow;
use crate::window::event_handler::{Context, EventHandler};
use crate::window::CooltraptionEvent;

pub struct DebugWindow {
    window_size: PhysicalSize<u32>,
    is_open: bool,
}

impl Default for DebugWindow {
    fn default() -> Self {
        Self {
            is_open: true,
            window_size: Default::default(),
        }
    }
}

impl EventHandler for DebugWindow {
    fn handle_event(&mut self, _event: &mut Event<CooltraptionEvent>, context: &mut Context) {
        self.window_size = context.window.inner_size();

        if !self.is_open {
            context.send_event(CooltraptionEvent::CloseGUI(self.id()));
        }
    }
}

impl GuiWindow for DebugWindow {
    fn show(&mut self, context: &egui::Context) {
        egui::Window::new("Debug")
            .open(&mut self.is_open)
            .resizable(false)
            .show(context, |ui| {
                ui.label(format!("{:?}", self.window_size));
            });
    }

    fn id(&self) -> &'static str {
        "debug"
    }
}
