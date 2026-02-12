use egui::{
    containers::{
        modal::{
            Modal,
        },
    },
    widgets::{
        Button,
    },
};
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
}

pub fn create_new_project_window(
    create_new_project_window_context: CreateNewProjectWindowContext
) {
    Modal::new("Create-New-Project-Window".into()).show(
        create_new_project_window_context.egui_context, 
        |window_ui| {
            if window_ui.add(Button::new("Close")).clicked() {
                create_new_project_window_context
                    .ui_affects
                    .push_back(UIAffect::CloseCreateNewProjectWindowButtonPressed);
            }
        }
    ); 
}
