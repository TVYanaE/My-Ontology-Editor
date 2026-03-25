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

pub struct OpenProjectWindow {
    project_file_path: String,
}

impl OpenProjectWindow {
    pub(super) fn new() -> Self {
        Self { 
            project_file_path: String::with_capacity(64), 
        }
    }

    pub(super) fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
    ) {
        Modal::new("Open-Project-Window".into())
            .show(context, |create_project_window_ui|{
                create_project_window_ui.vertical(|vertical_ui|{
                    main_panel(
                        vertical_ui, 
                        event_buffer, 
                        &mut self.project_file_path
                    );

                    bottom_panel(
                        vertical_ui, 
                        event_buffer,
                        &self.project_file_path,
                    );
                }); 
            }
        );
    }

    pub fn set_project_file_path(&mut self, project_file_path: &str) {
        self.project_file_path.clear();
        self.project_file_path.push_str(project_file_path);
    }

    pub fn clear(&mut self) {
        self.project_file_path.clear();
    }  
}

fn main_panel(
    vertical_ui: &mut EGUIUI,
    event_buffer: &mut GUIEventBuffer,
    project_file_path: &mut String,
) {
    vertical_ui.add(Label::new("Project File Path"));

    vertical_ui.horizontal(|horizontal_ui|{ 
        horizontal_ui.add(TextEdit::singleline(project_file_path));
        
        if horizontal_ui.add(Button::new("Path")).clicked() {
            event_buffer.push(
                GUIEvent::OpenModalWindow(
                    ModalWindowType::FileDialog{
                        item_type: ChoosingItemType::File,
                        receiver: FileDialogResponseReceiver::OpenProjectWindow,
                    }
                )
            );
        }
    });
}

fn bottom_panel(
    ui: &mut EGUIUI,
    event_buffer: &mut GUIEventBuffer,
    project_file_path: &str,
) {
    let (left_resp, right_resp) = Sides::new().show(ui, 
        |left_ui|{
            left_ui.add(Button::new("Open")) 
        }, 
        |right_ui|{
            right_ui.add(Button::new("Cancel"))
        },
    ); 

    if left_resp.clicked() {
        event_buffer.push(
            GUIEvent::OpenProjectRequest { 
                project_file_path: project_file_path.to_string(),
            }
        ); 
    };

    if right_resp.clicked() {
        event_buffer.push(GUIEvent::OpenProjectCanceled);
    };
}
