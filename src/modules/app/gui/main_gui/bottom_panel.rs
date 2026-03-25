
use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::panel::{TopBottomPanel, TopBottomSide};

use crate::modules::app::gui::gui_event::GUIEventBuffer;

use crate::modules::app::project::project_view::ProjectView;

pub struct BottomPanel {

}

impl BottomPanel {
    pub fn new() -> Self {
        Self {  
        }
    }
    pub fn prepare(
        &mut self,
        ctx: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
        selected_project: Option<&ProjectView>,
    ) {
        TopBottomPanel::new(TopBottomSide::Bottom, "Bottom-Panel")
            .show(ctx, |bottom_panel_ui|{
                if let Some(project_view) = selected_project {
                    let project_id = project_view.get_project_id();

                    bottom_panel_ui.label(project_id.get_str());
                }
            });
    } 
}
