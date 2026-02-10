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
        EGUIUI
    },
    modules::{
        graphics::{
            events::UIAffects,
            ui::{
                ui_affect::UIAffect,
            },
        },
    },
};

pub fn main_bar(ui: &mut EGUIUI, ui_affects: &mut UIAffects) {
    MenuBar::new().ui(ui, |menu_bar_ui|{
        MenuButton::new("File").ui(menu_bar_ui, |file_menu_ui|{
            if file_menu_ui.add(Button::new("Quit")).clicked() {
                ui_affects.push_back(UIAffect::QuitButtonPushed);
            }
        });
    }); 
}
