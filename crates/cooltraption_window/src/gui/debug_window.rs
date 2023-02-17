use crate::gui::{GuiWindow, UiState};
use egui::Context;
use std::sync::mpsc::Receiver;

pub struct Fps(f32);

pub struct DebugWindow {
    fps: Fps,
    fps_recv: Receiver<Fps>,
}

impl DebugWindow {
    pub fn new(fps_recv: Receiver<Fps>) -> Self {
        Self {
            fps: Fps(0.0),
            fps_recv,
        }
    }
}

impl GuiWindow for DebugWindow {
    fn show(&mut self, context: &Context) -> UiState {
        if let Ok(fps) = self.fps_recv.try_recv() {
            self.fps = fps;
        } else {
            return UiState::Close;
        }

        egui::Window::new("Debug").show(context, |ui| {
            ui.label(format!("FPS: {}", self.fps.0));
        });

        UiState::KeepOpen
    }
}
