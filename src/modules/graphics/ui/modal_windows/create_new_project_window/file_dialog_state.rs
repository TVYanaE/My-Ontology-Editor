use egui_file_dialog::{
    DialogState,
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

pub fn file_dialog_state(
    create_new_project_window_context: CreateNewProjectWindowContext
) {
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
                
                let current_state = std::mem::replace(
                    create_new_project_window_context.create_new_project_window_state, 
                    CreateNewProjectWindowState::Main
                ); 
 
                create_new_project_window_context
                    .create_new_project_window_data
                    .prev_state = Some(current_state); 
                
            }
            else {
                create_new_project_window_context
                    .create_new_project_window_data
                    .prev_state = Some(create_new_project_window_context
                    .create_new_project_window_state.clone());    

                *create_new_project_window_context
                    .create_new_project_window_state = CreateNewProjectWindowState::Notification(
                    "Invalid Symbols was found in path. Please Use UTF-8 Symbols Only".to_string()
                );
            }
    }
    else {
        // For close and cancel handling
        match create_new_project_window_context.create_new_project_window_data.file_dialog.state() {
            DialogState::Closed | DialogState::Cancelled => {
                let current_state = std::mem::replace(
                    create_new_project_window_context.create_new_project_window_state, 
                    CreateNewProjectWindowState::Main
                ); 
 
                create_new_project_window_context
                    .create_new_project_window_data
                    .prev_state = Some(current_state);
            },
            _ => {}
        }
    }
    
}
