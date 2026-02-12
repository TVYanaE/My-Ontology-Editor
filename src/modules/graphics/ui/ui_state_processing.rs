

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
                    UIData,
                },
            },
            graphics_states::{
                ui_state::{
                    ModalWindow, UIState
                },
            },
            ui::{
                modal_windows::{
                    create_new_project_window::{create_new_project_window, CreateNewProjectWindowContext},
                },
            },
        },
    },
};

pub struct UIStateProcessingContext<'c> {
    pub ui_state: &'c UIState,
    pub egui_context: &'c EGUIContext,
    pub ui_data: &'c mut UIData,
    pub ui_affects: &'c mut UIAffects,
}

pub fn ui_state_processing(
    ui_state_processing_context: UIStateProcessingContext,
) {
    match &ui_state_processing_context.ui_state {
        UIState::Idle => {},
        UIState::ModalWindowOpen(modal_window) => {
            match modal_window {
                ModalWindow::CreateNewProjectWindow => {
                    create_new_project_window(
                        CreateNewProjectWindowContext { 
                            egui_context: ui_state_processing_context.egui_context, 
                            create_new_project_window_data: &mut ui_state_processing_context
                                .ui_data
                                .modal_windows_data
                                .create_new_project_window_data, 
                            ui_affects: ui_state_processing_context.ui_affects
                        }
                    ); 
                },
            }
        }
    }
}
