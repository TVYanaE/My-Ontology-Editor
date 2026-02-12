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
    modules::{
        graphics::{
            events::UIAffects,
            
            ui::{
                ui_affect::UIAffect,
            },
        },
    },
};

pub struct MainBarContext<'c> {
    pub ui: &'c mut EGUIUI, 
    pub ui_affects: &'c mut UIAffects,
} 

pub fn main_bar(    
    main_bar_context: MainBarContext,
) {
    MenuBar::new().ui(main_bar_context.ui, |menu_bar_ui|{
        let (_menu_button_resp, _inner_resp) = MenuButton::new("File").ui(menu_bar_ui, |file_menu_ui|{
            if file_menu_ui.add(Button::new("Create New Project")).clicked() {
                main_bar_context.ui_affects.push_back(UIAffect::CreateNewProjectButtonPressed);
            }

            if file_menu_ui.add(Button::new("Quit")).clicked() {
                main_bar_context.ui_affects.push_back(UIAffect::QuitButtonPressed);
            };
        });
       
        
    }); 
}

