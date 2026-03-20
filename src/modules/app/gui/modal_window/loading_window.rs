
use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::modal::Modal;
use eframe::egui::widgets::Spinner;


pub struct LoadingWindow {

}


impl LoadingWindow {
    pub fn new() -> Self {
        Self {  }
    }
    pub fn prepare(
        &mut self,
        context: &EGUIContext,
    ) {
        Modal::new("Loading-Window".into()).show(context, |loading_window_ui|{
            loading_window_ui.add(Spinner::new());
        });
    }
}
