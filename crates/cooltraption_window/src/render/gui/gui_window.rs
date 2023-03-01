use egui::Context;

pub enum UiState {
    KeepOpen,
    Close,
}

pub trait GuiWindow {
    fn show(&mut self, context: &Context) -> UiState;
}
