
use eframe::egui::{
    Context as EGUIContext, 
    Ui as EGUIUI
};
use eframe::egui::containers::modal::Modal;
use eframe::egui::containers::Sides;
use eframe::egui::widgets::{Button, TextEdit, Label};

use crate::modules::app::gui::gui_event::{GUIEventBuffer, GUIEvent};

use crate::modules::app::gui::gui_state::{ModalWindowType, ChoosingItemType};
use crate::modules::app::gui::gui_state::FileDialogResponseReceiver;

pub struct CreateProjectWindow {
    project_name: String,
    project_path: String,
}

impl CreateProjectWindow {
    pub(super) fn new() -> Self {
        Self {
            project_name: String::with_capacity(32),
            project_path: String::with_capacity(32),
        }
    }
    pub(super) fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
    ) {
        Modal::new("Create-Project-Window".into())
            .show(context, |create_project_window_ui|{
                create_project_window_ui.vertical(|vertical_ui|{
                    main_panel(
                        vertical_ui, 
                        event_buffer, 
                        &mut self.project_name,
                        &mut self.project_path,
                    ); 
                    bottom_panel(
                        vertical_ui, 
                        event_buffer,
                        &self.project_name,
                        &self.project_path,
                    );
                }); 
            }
        ); 
    }
    /* pub fn set_project_name(&mut self, project_name: &str) {
        self.project_name.clear();
        self.project_name.push_str(project_name);
    } */
    pub fn set_project_path(&mut self, project_path: &str) {
        self.project_path.clear();
        self.project_path.push_str(project_path);
    }

    pub fn clear(&mut self) {
        self.project_path.clear();
        self.project_name.clear();
    }
}

fn main_panel(
    vertical_ui: &mut EGUIUI,
    event_buffer: &mut GUIEventBuffer,
    project_name: &mut String,
    project_path: &mut String,
) {
    
    vertical_ui.add(Label::new("Project Name"));

    vertical_ui.horizontal(|horizontal_ui|{
        horizontal_ui.add(TextEdit::singleline(project_name))
    }); 

    vertical_ui.add(Label::new("Project Path"));

    vertical_ui.horizontal(|horizontal_ui|{ 
        horizontal_ui.add(TextEdit::singleline(project_path));
        
        if horizontal_ui.add(Button::new("Path")).clicked() {
            event_buffer.push(
                GUIEvent::OpenModalWindow(
                    ModalWindowType::FileDialog{
                        item_type: ChoosingItemType::Dir,
                        receiver: FileDialogResponseReceiver::CreateProjectWindow,
                    }
                )
            );
        }
    });
}

fn bottom_panel(
    ui: &mut EGUIUI,
    event_buffer: &mut GUIEventBuffer,
    project_name: &str,
    project_path: &str,
) {
    let (left_resp, right_resp) = Sides::new().show(ui, 
        |left_ui|{
            left_ui.add(Button::new("Create")) 
        }, 
        |right_ui|{
            right_ui.add(Button::new("Cancel"))
        },
    ); 

    if left_resp.clicked() {
        event_buffer.push(
            GUIEvent::CreateProjectRequest { 
                project_name: project_name.to_string(), 
                project_path: project_path.to_string(), 
            }
        );
    };

    if right_resp.clicked() {
        event_buffer.push(GUIEvent::CreateProjectCanceled);
    };
}
