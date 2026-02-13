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
        EGUIUI,
    },
    modules::{
        graphics::{
            events::{
                UIAffects,
            },
            graphics_data::{
                ui_data::{
                    modal_windows_data::{
                        create_new_project_window_data::CreateNewProjectWindowData,
                    },
                },
            },
            graphics_states::{
                ui_state::{
                    create_new_project_window_state::CreateNewProjectWindowState,
                },
            },
            ui::{
                ui_affect::{UIAffect, CreateProjectRequest},
            },
        },
    },
};
use super::{
    CreateNewProjectWindowContext
};

pub fn main_state(
    create_new_project_window_context: CreateNewProjectWindowContext
) {
    Modal::new("Create-New-Project-Window".into()).show(
        create_new_project_window_context.egui_context, 
        |window_ui| {
            TopBottomPanel::new(TopBottomSide::Bottom, "Create-New-Project-Window-Bottom-Panel")
                .show_inside(window_ui, |bottom_ui|{
                    bottom_container(
                        bottom_ui, 
                        create_new_project_window_context.ui_affects,
                        create_new_project_window_context.create_new_project_window_data,
                        create_new_project_window_context.create_new_project_window_state,
                    );
                });

            CentralPanel::default().show_inside(window_ui, |central_ui|{
                central_container(
                    central_ui, 
                    create_new_project_window_context.create_new_project_window_data,
                    create_new_project_window_context.create_new_project_window_state
                );
            }); 
        });
}

fn bottom_container(
    bottom_ui: &mut EGUIUI,
    ui_affects: &mut UIAffects,
    create_new_project_window_data: &mut CreateNewProjectWindowData,
    create_new_project_window_state: &mut CreateNewProjectWindowState,
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
        ui_affects.push_back(UIAffect::CloseCreateNewProjectWindowButtonPressed);
    }
    if right_resp.clicked() {
        create_new_project(
            create_new_project_window_data, 
            create_new_project_window_state,
            ui_affects,
        );     
    }
}

fn central_container(
    central_ui: &mut EGUIUI,
    create_new_project_window_data: &mut CreateNewProjectWindowData, 
    create_new_project_window_state: &mut CreateNewProjectWindowState 
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
                    create_new_project_window_data.prev_state = Some(
                        create_new_project_window_state.clone()
                    );

                    *create_new_project_window_state = CreateNewProjectWindowState::FileDialog; 
                    create_new_project_window_data.file_dialog.pick_directory();
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
    create_new_project_window_state: &mut CreateNewProjectWindowState,
    ui_affects: &mut UIAffects
) {
    let project_dir = PathBuf::from(&create_new_project_window_data.project_path); 
        match project_dir.metadata() {
            Ok(meta) => {
                if !meta.is_dir() {
                    let error_text = format!("Invalid Path: Is not directory");

                    let current_state = std::mem::replace(
                        create_new_project_window_state, 
                        CreateNewProjectWindowState::Notification(error_text)
                    );
                    create_new_project_window_data.prev_state = Some(current_state);    
                    return;
                }
            },
            Err(error) => {
                let error_text = format!("Invalid Path: {error}");

                let current_state = std::mem::replace(
                    create_new_project_window_state, 
                    CreateNewProjectWindowState::Notification(error_text)
                );
                create_new_project_window_data.prev_state = Some(current_state);
                return;
            }
        }
    ui_affects.push_back(UIAffect::CreateProjectReq(
        CreateProjectRequest { 
            project_name: create_new_project_window_data.project_name.clone(), 
            project_dir: project_dir 
        }
    ));

    create_new_project_window_data.project_name.clear();
    create_new_project_window_data.project_path.clear();
}
