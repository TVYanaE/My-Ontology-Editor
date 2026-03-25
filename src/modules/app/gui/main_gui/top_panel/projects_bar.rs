
use eframe::egui::Ui as EGUIUi;
use eframe::egui::RichText;
use eframe::egui::Color32;
use eframe::egui::widgets::Button;

use crate::modules::app::gui::gui_event::{GUIEventBuffer, GUIEvent};

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
        event_buffer: &mut GUIEventBuffer,
        project_views: &[(&ProjectID, &ProjectView)],
        selected_project: Option<&ProjectView>,
    ) {
        ui.horizontal(|horizontal_ui|{
            if let Some (selected_project) = selected_project {
                for (current_project_id, project_view) in project_views {  
                    if **current_project_id == selected_project.get_project_id() {  
                        horizontal_ui.add(
                            Button::new(
                                RichText::new(project_view.get_project_name())
                            )
                        );
                    } else { 
                        let resp = horizontal_ui.add(
                            Button::new(
                                RichText::new(project_view.get_project_name())
                                    .color(Color32::DARK_GRAY)
                            )
                        ); 

                        if resp.clicked() {
                            event_buffer.push(
                                GUIEvent::SwitchProjectRequest { 
                                    project_id: (*current_project_id).clone(),
                                }
                            );
                        }
                    }
                }
            }
        });
    }
}
