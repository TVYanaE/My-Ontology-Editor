use egui::{
    containers::{
        modal::{
            Modal,
        },
        
    },
    widgets::{
        
        Button, Label,
    },
};
use crate::{
    modules::{
        graphics::{
            graphics_states::{
                ui_state::{
                    create_new_project_window_state::CreateNewProjectWindowState,
                },
            },
        },
    },
};
use super::{
    CreateNewProjectWindowContext
};

pub fn notification_state(
    text: &String,
    create_new_project_window_context: CreateNewProjectWindowContext
) {
    Modal::new("Create-New-Project-Window-Notification".into()).show(
        create_new_project_window_context.egui_context, 
        |notificatio_ui| {
            notificatio_ui.add(Label::new(text));
            if notificatio_ui.add(Button::new("Ok")).clicked() {
                if let Some(prev_state) = create_new_project_window_context
                    .create_new_project_window_data
                    .prev_state
                    .as_ref() {
                        let current_state = std::mem::replace(
                            create_new_project_window_context.create_new_project_window_state, 
                            prev_state.clone()
                        );
                        create_new_project_window_context
                            .create_new_project_window_data.prev_state = Some(current_state);
                }
                else {
                    let current_state = std::mem::replace(
                        create_new_project_window_context.create_new_project_window_state, 
                        CreateNewProjectWindowState::Main
                    );

                    create_new_project_window_context
                        .create_new_project_window_data.prev_state = Some(current_state);
                }
            }
        }
    ); 
}
