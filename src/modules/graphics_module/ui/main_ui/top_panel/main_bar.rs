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
        graphics_module::{
            ui::{
                events::{UIEvents, UIEvent},
                ui_error::UIError,
            },
        },
    },
};


#[derive(Default)]
pub struct MainBar;

impl MainBar {
    pub(super) fn prepare(
        &mut self,
        egui_ui: &mut EGUIUI
    ) -> Result<UIEvents, UIError> {
        let mut ui_events = UIEvents::with_capacity(4);
        
        MenuBar::new().ui(egui_ui, |menu_bar_ui|{ 
            MenuButton::new("File").ui(menu_bar_ui, |file_menu_ui|{
                if file_menu_ui.add(Button::new("Create New Project")).clicked() {
                    ui_events.push(UIEvent::OpenCreateNewProjectWindow);
                }

                if file_menu_ui.add(Button::new("Quit")).clicked() {
                    ui_events.push(UIEvent::QuitApp);
                };
            }); 
        }); 

        Ok(ui_events)
    }
}
