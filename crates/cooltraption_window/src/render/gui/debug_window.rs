use crate::{CooltraptionEvent, EventHandler};
use egui::{Context, Window};
use winit::dpi::PhysicalSize;
use winit::event::Event;

use crate::render::gui::gui_window::UiState;
use crate::render::gui::GuiWindow;

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
    fn handle_event(&mut self, event: &Event<CooltraptionEvent>, context: &mut crate::Context) {
        self.window_size = context.window.inner_size();
    }
}

impl GuiWindow for DebugWindow {
    fn show(&mut self, context: &Context) -> UiState {
        Window::new("Debug")
            .open(&mut self.is_open)
            .resizable(false)
            .show(context, |ui| {
                ui.label(format!("{:?}", self.window_size));
            });

        match self.is_open {
            true => UiState::KeepOpen,
            false => UiState::Close,
        }
    }
}
