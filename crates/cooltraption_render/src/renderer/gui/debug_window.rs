use cooltraption_common::events::EventHandler;
use winit::dpi::PhysicalSize;

use crate::renderer::gui::GuiWindow;
use crate::window::{WindowContext, WindowEvent, WinitEvent};

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

impl<'s> EventHandler<'s, WinitEvent<'_, '_>, WindowContext<'_>> for DebugWindow {
    fn handle_event(&mut self, _event: &mut WinitEvent, context: &mut WindowContext) {
        self.window_size = context.window.inner_size();

        if !self.is_open {
            context.send_event(WindowEvent::CloseGUI(self.id()));
        }
    }
}

impl<'s> GuiWindow<'s> for DebugWindow {
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
