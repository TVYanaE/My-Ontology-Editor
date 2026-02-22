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
                events::{UIEvents, UIEvent, ChosedModalWindow},
                ui_error::UIError,
            },
        },
    },
};


pub struct CreateNewProjectWindow {
    project_name: String,
    project_path: String,
}

impl Default for CreateNewProjectWindow {
    fn default() -> Self {
        Self { 
            project_path: String::with_capacity(64), 
            project_name: String::with_capacity(32),
        }
    }
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
                        &mut self.project_name,
                        &mut self.project_path,
                    );
                });

            CentralPanel::default().show_inside(window_ui, |central_ui|{
                central_container(
                    central_ui, 
                    &mut ui_events,
                    &mut self.project_name,
                    &mut self.project_path,
                );
            }); 
        });

        Ok(ui_events)
    }

    pub fn set_project_path(
        &mut self, 
        project_path: &str
    ) {
        self.project_path.clear();
        self.project_path.push_str(project_path);
    }
    
    pub fn set_project_name(
        &mut self,
        project_name: &str
    ) {
        self.project_name.clear();
        self.project_name.push_str(project_name);
    }
}


fn bottom_container(
    bottom_ui: &mut EGUIUI,
    ui_events: &mut Vec<UIEvent>,
    project_name: &mut String,
    project_path: &mut String,
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
        ui_events.push(UIEvent::ShowMainUI);
    }
    if right_resp.clicked() {
        create_new_project(
            project_name,
            project_path,
            ui_events,
        );     
    }
}

fn central_container(
    central_ui: &mut EGUIUI,
    ui_events: &mut Vec<UIEvent>,
    project_name: &mut String,
    project_path: &mut String,
) {
    central_ui.with_layout(
        Layout::top_down(Align::Center), 
        |vertical_ui| {
            vertical_ui.add(Label::new("Project Name"));
             
            vertical_ui.horizontal(|horizontal_ui|{
                horizontal_ui.add_sized(
                    horizontal_ui.available_size(),
                    TextEdit::singleline(project_name))
            });

            vertical_ui.add(Label::new("Project location"));
             
            vertical_ui.horizontal(|horizontal_ui|{
                if horizontal_ui.add(Button::new("Choose")).clicked() {
                    ui_events.push(UIEvent::ShowModalWindow(ChosedModalWindow::FileDialog)); 
                }   

                horizontal_ui.add_sized(
                    horizontal_ui.available_size(),
                    TextEdit::singleline(project_path))
            });
        }
    ); 
}

fn create_new_project(
    project_name: &mut String,
    project_path: &mut String,
    ui_events: &mut Vec<UIEvent>
) {
    let project_path_for_send = PathBuf::from(&project_path); 
        
    ui_events.push(UIEvent::CreateProjectReq{ 
        project_name: project_name.clone(), 
        project_path: project_path_for_send, 
    });

    project_name.clear();
    project_path.clear();
}
