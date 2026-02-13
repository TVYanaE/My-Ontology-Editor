mod file_dialog_state;
mod main_state;
mod notification_state;

use crate::{
    aliases::{
        EGUIContext,
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
        },
    },
};
use self::{
    file_dialog_state::file_dialog_state,
    main_state::main_state,
    notification_state::notification_state,
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
        CreateNewProjectWindowState::Main => {
            main_state(create_new_project_window_context); 
        },
        CreateNewProjectWindowState::FileDialog => { 
            file_dialog_state(create_new_project_window_context);      
        }
        CreateNewProjectWindowState::Notification(text) => {
            let text_clone = text.clone();
            notification_state(&text_clone, create_new_project_window_context); 
        }
    }  
}
