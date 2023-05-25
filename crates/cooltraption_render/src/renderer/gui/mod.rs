use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;

use cooltraption_window::events::EventHandler;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use winit::window::Window;

use crate::renderer::wgpu_state::WgpuState;
use crate::renderer::{BoxedRenderer, RenderError, RenderFrame, Renderer, RendererInitializer};
use cooltraption_window::window::{WindowContext, WinitEvent};
pub use egui;
pub use widget::Widget;

mod widget;

type SharedPlatform = Rc<RefCell<Option<Platform>>>;
type SharedWidgetsMap = Rc<RefCell<HashMap<WidgetId, Box<dyn Widget>>>>;

pub type WidgetId = &'static str;

pub enum GuiCommand {
    Open(Box<dyn Widget>),
    Close(WidgetId),
}

pub struct GuiActionDispatcher {
    command_send: Sender<GuiCommand>,
}

pub fn new() -> (GuiRendererInitializer, GuiEventHandler, GuiActionDispatcher) {
    let platform = SharedPlatform::default();
    let widgets = SharedWidgetsMap::default();

    let (command_send, command_recv) = std::sync::mpsc::channel();

    (
        GuiRendererInitializer {
            platform: platform.clone(),
            widgets: widgets.clone(),
        },
        GuiEventHandler {
            platform,
            widgets,
            command_recv,
        },
        GuiActionDispatcher { command_send },
    )
}

impl GuiActionDispatcher {
    pub fn open(&self, widget: Box<dyn Widget>) -> WidgetId {
        let id = widget.id();
        self.command_send
            .send(GuiCommand::Open(widget))
            .expect("send open command");
        id
    }

    pub fn close(&self, id: WidgetId) {
        self.command_send
            .send(GuiCommand::Close(id))
            .expect("send close command");
    }
}

struct GuiRenderer {
    start_time: Instant,
    render_pass: RenderPass,
    platform: SharedPlatform,
    widgets: SharedWidgetsMap,
}

pub struct GuiEventHandler {
    platform: SharedPlatform,
    widgets: SharedWidgetsMap,
    command_recv: Receiver<GuiCommand>,
}

pub struct GuiRendererInitializer {
    platform: SharedPlatform,
    widgets: SharedWidgetsMap,
}

impl EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for GuiEventHandler {
    fn handle_event(&mut self, event: &mut WinitEvent<'_, '_>, context: &mut WindowContext<'_>) {
        if let Some(platform) = self.platform.borrow_mut().as_mut() {
            platform.handle_event(event.0);

            while let Ok(command) = self.command_recv.try_recv() {
                match command {
                    GuiCommand::Open(widget) => {
                        self.widgets.borrow_mut().insert(widget.id(), widget);
                    }
                    GuiCommand::Close(id) => {
                        self.widgets.borrow_mut().remove(id);
                    }
                }
            }

            for widget in self.widgets.borrow_mut().values_mut() {
                widget.handle_event(event, context);
            }
        }
    }
}

impl Renderer for GuiRenderer {
    fn render(&mut self, render_frame: &mut RenderFrame) -> Result<(), Box<dyn RenderError>> {
        if let Some(platform) = self.platform.borrow_mut().as_mut() {
            // Begin to draw the UI frame.
            platform.update_time(self.start_time.elapsed().as_secs_f64());
            platform.begin_frame();

            // Draw all widgets
            self.widgets
                .borrow_mut()
                .retain(|_, widget| widget.show(&platform.context()));

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
                .add_textures(render_frame.device, render_frame.queue, &t_delta)?;

            self.render_pass.update_buffers(
                render_frame.device,
                render_frame.queue,
                &paint_jobs,
                &screen_descriptor,
            );

            // Record all render passes.
            self.render_pass.execute(
                &mut render_frame.encoder,
                &render_frame.view,
                &paint_jobs,
                &screen_descriptor,
                None,
            )?;

            self.render_pass.remove_textures(t_delta)?;
        }

        Ok(())
    }
}

impl RendererInitializer for GuiRendererInitializer {
    fn init(self: Box<Self>, wgpu_state: &mut WgpuState, window: &Window) -> BoxedRenderer {
        *self.platform.borrow_mut() = Some(Platform::new(PlatformDescriptor {
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
            widgets: self.widgets,
        })
    }
}

#[derive(Debug)]
struct GuiBackendError(egui_wgpu_backend::BackendError);

impl RenderError for GuiBackendError {}

impl Error for GuiBackendError {}

impl Display for GuiBackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GuiBackendError: {}", self.0)
    }
}

impl From<egui_wgpu_backend::BackendError> for Box<dyn RenderError> {
    fn from(err: egui_wgpu_backend::BackendError) -> Self {
        Box::new(GuiBackendError(err))
    }
}
