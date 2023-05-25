use cooltraption_render::gui::{egui, Widget, WidgetId};
use cooltraption_window::events::EventHandler;
use cooltraption_window::window::winit::dpi::PhysicalSize;
use cooltraption_window::window::{WindowContext, WinitEvent};
use std::fmt::{Display, Formatter};
use std::time::Instant;

struct FpsCounter {
    min: f32,
    max: f32,
    avg: f32,
    frame_count: u32,
    start_time: Instant,
}

impl Display for FpsCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.min == f32::MAX {
            write!(f, "(MIN / MAX / AVG): - / - / -")
        } else {
            write!(
                f,
                "(MIN / MAX / AVG): {:.0} / {:.0} / {:.0}",
                self.min, self.max, self.avg
            )
        }
    }
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            min: f32::MAX,
            max: 0.0,
            avg: 0.0,
            frame_count: 0,
            start_time: Instant::now(),
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        let elapsed_time = self.start_time.elapsed();
        if elapsed_time.as_secs() >= 1 {
            let fps = self.frame_count as f32 / elapsed_time.as_secs_f32();
            if fps < self.min {
                self.min = fps;
            }
            if fps > self.max {
                self.max = fps;
            }
            self.avg =
                (self.avg * (elapsed_time.as_secs_f32() - 1.0) + fps) / elapsed_time.as_secs_f32();
            self.frame_count = 0;
            self.start_time = Instant::now();
        }
    }
}

pub struct DebugWidget {
    window_size: PhysicalSize<u32>,
    fps: FpsCounter,
    eps: FpsCounter,
    is_open: bool,
}

impl Default for DebugWidget {
    fn default() -> Self {
        Self {
            is_open: true,
            window_size: Default::default(),
            fps: FpsCounter::new(),
            eps: FpsCounter::new(),
        }
    }
}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for DebugWidget {
    fn handle_event(&mut self, _event: &mut WinitEvent, context: &mut WindowContext) {
        self.window_size = context.window.inner_size();

        // Update tps
        self.eps.tick();
    }
}

impl Widget for DebugWidget {
    fn show(&mut self, context: &egui::Context) -> bool {
        // Update fps
        self.fps.tick();

        egui::Window::new("Debug")
            .open(&mut self.is_open)
            .resizable(false)
            .show(context, |ui| {
                ui.label(format!("{:?}", self.window_size));

                ui.add_space(10.0);

                ui.label(format!("FPS {}", self.fps));
                ui.label(format!("EPS {}", self.eps));
            });

        self.is_open
    }

    fn id(&self) -> WidgetId {
        "debug"
    }
}
