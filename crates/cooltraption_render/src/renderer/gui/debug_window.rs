use crate::events::EventHandler;
use std::time::Instant;
use winit::dpi::PhysicalSize;

use crate::renderer::gui::GuiWindow;
use crate::window::{WindowContext, WindowEvent, WinitEvent};

struct FpsCounter {
    min_fps: f32,
    max_fps: f32,
    avg_fps: f32,
    frame_count: u32,
    start_time: Instant,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            min_fps: f32::MAX,
            max_fps: 0.0,
            avg_fps: 0.0,
            frame_count: 0,
            start_time: Instant::now(),
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        let elapsed_time = self.start_time.elapsed();
        if elapsed_time.as_secs() >= 1 {
            let fps = self.frame_count as f32 / elapsed_time.as_secs_f32();
            if fps < self.min_fps {
                self.min_fps = fps;
            }
            if fps > self.max_fps {
                self.max_fps = fps;
            }
            self.avg_fps = (self.avg_fps * (elapsed_time.as_secs_f32() - 1.0) + fps)
                / elapsed_time.as_secs_f32();
            self.frame_count = 0;
            self.start_time = Instant::now();
        }
    }
}

pub struct DebugWindow {
    window_size: PhysicalSize<u32>,
    tps: FpsCounter,
    fps: FpsCounter,
    is_open: bool,
}

impl Default for DebugWindow {
    fn default() -> Self {
        Self {
            is_open: true,
            window_size: Default::default(),
            tps: FpsCounter::new(),
            fps: FpsCounter::new(),
        }
    }
}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for DebugWindow {
    fn handle_event(&mut self, _event: &mut WinitEvent, context: &mut WindowContext) {
        self.window_size = context.window.inner_size();

        // Update tps
        self.tps.tick();

        if !self.is_open {
            context.send_event(WindowEvent::CloseGUI(self.id()));
        }
    }
}

impl GuiWindow for DebugWindow {
    fn show(&mut self, context: &egui::Context) {
        // Update fps
        self.fps.tick();

        egui::Window::new("Debug")
            .open(&mut self.is_open)
            .resizable(false)
            .show(context, |ui| {
                ui.label(format!("{:?}", self.window_size));
                ui.label(format!("FPS: {:.2}", self.fps.avg_fps));
                ui.label(format!("TPS {:.2}", self.tps.avg_fps));
            });
    }

    fn id(&self) -> &'static str {
        "debug"
    }
}
