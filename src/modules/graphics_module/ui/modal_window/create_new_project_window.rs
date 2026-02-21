use std::{
    path::PathBuf,
};
use egui::{
    containers::{
        modal::{
            Modal,
        },
        panel::{
            TopBottomPanel, TopBottomSide,
            CentralPanel,
        },
        Sides,
    },
    widgets::{
        text_edit::{
            TextEdit,
        },
        Button, Label,
    },
    Layout, Align,
};
use crate::{
    aliases::{
        EGUIUI, EGUIContext,
    }, 
    modules::{
        graphics_module::{
            ui::{
                events::{UIEvents, UIEvent},
                ui_error::UIError,
            },
        },
    },
};


pub struct CreateNewProjectWindowData {
    pub project_path: String,
    pub project_name: String,
}

impl Default for CreateNewProjectWindowData {
    fn default() -> Self {
        Self { 
            project_path: String::with_capacity(64), 
            project_name: String::with_capacity(32),
        }
    }
}

#[derive(Default)]
pub struct CreateNewProjectWindow {
    data: CreateNewProjectWindowData,
}

impl CreateNewProjectWindow {
    pub(super) fn prepare(
        &mut self,
        egui_context: &EGUIContext,
    ) -> Result<UIEvents, UIError> {
        let mut ui_events = UIEvents::with_capacity(4);
        
        Modal::new("Create-New-Project-Window".into()).show(
            egui_context, |window_ui| {
            TopBottomPanel::new(TopBottomSide::Bottom, "Create-New-Project-Window-Bottom-Panel")
                .show_inside(window_ui, |bottom_ui|{
                    bottom_container(
                        bottom_ui, 
                        &mut ui_events,
                        &mut self.data,
                    );
                });

            CentralPanel::default().show_inside(window_ui, |central_ui|{
                central_container(
                    central_ui, 
                    &mut ui_events,
                    &mut self.data,
                );
            }); 
        });

        Ok(ui_events)
    }

    pub fn set_project_dir(
        &mut self, 
        project_dir: String
    ) {
        self.data.project_path = project_dir;
    }
}


fn bottom_container(
    bottom_ui: &mut EGUIUI,
    ui_events: &mut Vec<UIEvent>,
    create_new_project_window_data: &mut CreateNewProjectWindowData,
) {
    let (left_resp, right_resp) = Sides::new().show(bottom_ui, 
        |left_ui|{
            left_ui.add(Button::new("Close")) 
        }, 
        |right_ui|{
            right_ui.add(Button::new("Create"))
        }
    ); 

    if left_resp.clicked() {
        ui_events.push(UIEvent::CloseCreateNewProjectWindow);
    }
    if right_resp.clicked() {
        create_new_project(
            create_new_project_window_data, 
            ui_events,
        );     
    }
}

fn central_container(
    central_ui: &mut EGUIUI,
    ui_events: &mut Vec<UIEvent>,
    create_new_project_window_data: &mut CreateNewProjectWindowData, 
) {
    central_ui.with_layout(
        Layout::top_down(Align::Center), 
        |vertical_ui| {
            vertical_ui.add(Label::new("Project Name"));
             
            vertical_ui.horizontal(|horizontal_ui|{
                horizontal_ui.add_sized(
                    horizontal_ui.available_size(),
                    TextEdit::singleline(&mut create_new_project_window_data.project_name))
            });

            vertical_ui.add(Label::new("Project location"));
             
            vertical_ui.horizontal(|horizontal_ui|{
                if horizontal_ui.add(Button::new("Choose")).clicked() {
                    ui_events.push(UIEvent::OpenFileDialogReq); 
                }   

                horizontal_ui.add_sized(
                    horizontal_ui.available_size(),
                    TextEdit::singleline(&mut create_new_project_window_data.project_path))
            });
        }
    ); 
}

fn create_new_project(
    create_new_project_window_data: &mut CreateNewProjectWindowData, 
    ui_events: &mut Vec<UIEvent>
) {
    let project_dir = PathBuf::from(&create_new_project_window_data.project_path); 
        match project_dir.metadata() {
            Ok(meta) => {
                if !meta.is_dir() {
                    let error_text = format!("Invalid Path: Is not directory");

                    ui_events.push(UIEvent::ShowNotification(error_text));  
                    return;
                }
            },
            Err(error) => {
                let error_text = format!("Invalid Path: {error}");
                
                ui_events.push(UIEvent::ShowNotification(error_text));
                return;
            }
        }
    ui_events.push(UIEvent::CreateProjectReq{ 
        project_name: create_new_project_window_data.project_name.clone(), 
        project_dir: project_dir 
    });

    create_new_project_window_data.project_name.clear();
    create_new_project_window_data.project_path.clear();
}
