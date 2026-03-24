
use eframe::egui::Ui as EGUIUi;

use crate::modules::app::gui::gui_event::GUIEventBuffer;

use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::project_view::ProjectView;


pub struct ProjectsBar {
     
}

impl ProjectsBar {  
    pub fn new() -> Self { 
        Self {  
            
        }
    }
    pub fn prepare(
        &mut self,
        ui: &mut EGUIUi,
        _event_buffer: &mut GUIEventBuffer,
        project_views: &[(&ProjectID, &mut ProjectView)],
    ) {
        ui.horizontal(|horizontal_ui|{
            for (_, view) in project_views {
                horizontal_ui.label(view.get_project_name());
            } 
        });
    }
}
