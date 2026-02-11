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
        EGUIUI, EGUIContext,
    },
    modules::{
        graphics::{
            events::UIAffects,
            graphics_data::{
                ui_data::{
                    panels_data::{
                        top_panel_data::{
                            main_bar_data::MainBarData,
                        },
                    },
                },
            },
            ui::{
                ui_affect::UIAffect,
            },
        },
    },
};

pub struct MainBarContext<'c> {
    pub ui: &'c mut EGUIUI, 
    pub egui_context: &'c EGUIContext, 
    pub ui_affects: &'c mut UIAffects,
    pub main_bar_data: &'c mut MainBarData,
} 

pub fn main_bar(    
    main_bar_context: MainBarContext,
) {
    MenuBar::new().ui(main_bar_context.ui, |menu_bar_ui|{
        let (menu_button_resp, _inner_resp) = MenuButton::new("File").ui(menu_bar_ui, |file_menu_ui|{
            if file_menu_ui.add(Button::new("Quit")).clicked() {
                main_bar_context.ui_affects.push_back(UIAffect::QuitButtonPushed);
            };
        });
       
        if menu_button_resp.clicked() {
            main_bar_context.egui_context.request_repaint();
        }
        let hovering_now = menu_button_resp.hovered();

        if hovering_now != main_bar_context.main_bar_data.menu_button_hovered {
            main_bar_context.egui_context.request_repaint();
        }
        
        main_bar_context.main_bar_data.menu_button_hovered = hovering_now;
    }); 
}
