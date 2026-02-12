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
        EGUIContext, EGUIUI,
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
                ui_state::CreateNewProjectWindowState,
            },
            ui::{
                ui_affect::UIAffect,
            },
        },
    },
};

pub struct CreateNewProjectWindowContext<'c> {
    pub egui_context: &'c EGUIContext,
    pub create_new_project_window_data: &'c mut CreateNewProjectWindowData,
    pub ui_affects: &'c mut UIAffects,
    pub create_new_project_window_state: &'c mut CreateNewProjectWindowState,
}

pub fn create_new_project_window(
    create_new_project_window_context: CreateNewProjectWindowContext
) {
    match create_new_project_window_context.create_new_project_window_state {
        CreateNewProjectWindowState::MainWindow => {
            Modal::new("Create-New-Project-Window".into()).show(
            create_new_project_window_context.egui_context, 
            |window_ui| {
                TopBottomPanel::new(TopBottomSide::Bottom, "Create-New-Project-Window-Bottom-Panel")
                    .show_inside(window_ui, |bottom_ui|{
                        bottom_container(bottom_ui, create_new_project_window_context.ui_affects);
                    });

                CentralPanel::default().show_inside(window_ui, |central_ui|{
                    central_container(
                        central_ui, 
                        create_new_project_window_context.create_new_project_window_data,
                        create_new_project_window_context.ui_affects,
                        create_new_project_window_context.create_new_project_window_state
                    );
                }); 
            });
        },
        CreateNewProjectWindowState::FileDialog => {
            create_new_project_window_context
                .create_new_project_window_data
                .file_dialog
                .pick_directory();
            create_new_project_window_context
                .create_new_project_window_data
                .file_dialog
                .update(create_new_project_window_context.egui_context);
             

            if let Some(project_path) = create_new_project_window_context
                .create_new_project_window_data
                .file_dialog
                .picked() {
                    if let Some(project_path_str) = project_path.to_str() {
                        create_new_project_window_context
                            .create_new_project_window_data
                            .project_path
                            .clear();

                        create_new_project_window_context
                            .create_new_project_window_data
                            .project_path
                            .push_str(project_path_str);

                        *create_new_project_window_context.create_new_project_window_state = CreateNewProjectWindowState::MainWindow;
                    }
                    else {
                        // TODO: Logic for invalid cymbols in path
                        *create_new_project_window_context.create_new_project_window_state = CreateNewProjectWindowState::MainWindow;
                    }
            }
        }
    } 

     
}

fn bottom_container(
    bottom_ui: &mut EGUIUI,
    ui_affects: &mut UIAffects,
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
        // TODO: Logic for creating project
    }
}

fn central_container(
    central_ui: &mut EGUIUI,
    create_new_project_window_data: &mut CreateNewProjectWindowData, 
    ui_affects: &mut UIAffects,
    create_new_project_window_state: &mut CreateNewProjectWindowState 
) {
    central_ui.with_layout(
        Layout::top_down(Align::Center), 
        |vertical_ui| {
            vertical_ui.add(Label::new("Project location"));
             
            vertical_ui.horizontal(|horizontal_ui|{
                if horizontal_ui.add(Button::new("Choose")).clicked() {
                    *create_new_project_window_state = CreateNewProjectWindowState::FileDialog; 
                }  

                  

                horizontal_ui.add_sized(
                    horizontal_ui.available_size(),
                    TextEdit::singleline(&mut create_new_project_window_data.project_path))
            }); 
        }
    ); 
}
