use egui::{
    containers::{
        menu::{
            MenuBar, MenuButton,
        },
    },
    widgets::{
        Button,
    },
};
use crate::{
    aliases::{
        EGUIUI,
    }, 
};
use super::{
    UIEvent
};

#[derive(Default)]
pub struct MainBar;

impl MainBar {
    pub(super) fn prepare(
        &mut self,
        egui_ui: &mut EGUIUI
    ) -> Vec<UIEvent> {
        let mut ui_affects = Vec::with_capacity(4);
        
        MenuBar::new().ui(egui_ui, |menu_bar_ui|{ 
            MenuButton::new("File").ui(menu_bar_ui, |file_menu_ui|{
                if file_menu_ui.add(Button::new("Create New Project")).clicked() {
                    ui_affects.push(UIEvent::CreateNewProjectButtonPressed);
                }

                if file_menu_ui.add(Button::new("Quit")).clicked() {
                    ui_affects.push(UIEvent::QuitButtonPressed);
                };
            }); 
        }); 

        ui_affects
    }
}
