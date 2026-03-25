mod file_menu; 
mod view_menu;

use eframe::egui::Ui as EGUIUI;
use eframe::egui::containers::menu::{MenuBar, MenuButton};

use crate::modules::app::gui::gui_event::GUIEventBuffer;

use self::file_menu::FileMenu;
use self::view_menu::ViewMenu;

pub struct MainBar {
    file_menu: FileMenu,
    view_menu: ViewMenu,
}

impl MainBar {
    pub fn new() -> Self {
        Self {  
            file_menu: FileMenu::new(),
            view_menu: ViewMenu::new(),
        }
    }
    pub fn prepare(
        &mut self,
        ui: &mut EGUIUI,
        event_buffer: &mut GUIEventBuffer,
    ) {
        MenuBar::new()
            .ui(ui, |menu_bar_ui|{
                MenuButton::new("File")
                    .ui(menu_bar_ui, |file_menu_ui|{
                        self.file_menu.prepare(file_menu_ui, event_buffer);
                    }
                );

                MenuButton::new("View")
                    .ui(menu_bar_ui, |view_menu_ui|{
                        self.view_menu.prepare(view_menu_ui, event_buffer); 
                    }
                );
            }
        ); 
    }
}
