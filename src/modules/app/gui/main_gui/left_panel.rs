
use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::panel::SidePanel;
use eframe::egui::containers::scroll_area::ScrollArea;
use eframe::egui::widgets::Button;
use eframe::egui::PointerButton;

use crate::modules::app::gui::gui_event::{GUIEvent, GUIEventBuffer};

use crate::modules::app::project::project_view::ProjectView;
use crate::modules::app::project::semantic_node::SemanticNode;

pub struct LeftPanel {
    visibility: bool,
}

impl LeftPanel {
    pub(super) fn new() -> Self {
        Self { 
            visibility: true, 
        }
    }

    pub(super) fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
        selected_project: Option<&ProjectView>,
    ) {
        if !self.visibility {
            return;
        }

        SidePanel::left("Left-Panel").show(context, |left_panel_ui|{
            let ui_height = left_panel_ui.available_size().y; 

            let text_style = eframe::egui::TextStyle::Body; 
            let row_height = left_panel_ui.text_style_height(&text_style);

            let button_height = left_panel_ui.text_style_height(&eframe::egui::TextStyle::Button);
            let button_part = button_height / ui_height;
            
            left_panel_ui.vertical(|vertical_ui|{
                vertical_ui.set_max_height(ui_height * (1.0 - button_part - 0.005));
                ScrollArea::vertical().show_rows(
                    vertical_ui, 
                    row_height,
                    200,
                    |scrol_ui, row_range|{
                    if let Some(project_view) = selected_project {
                        let semantic_nodes: Vec<&SemanticNode> = project_view
                            .iter_semantic_nodes()
                            .collect(); 
                        for semantic_node in semantic_nodes {
                            scrol_ui.label(semantic_node.get_name());
                        }
                    }

                    if scrol_ui.response().clicked_by(PointerButton::Secondary) {
                        println!("right click");
                    };
                });
            });
            

            left_panel_ui.horizontal(|horizontal_ui|{
                if horizontal_ui.add(Button::new("+")).clicked() {
                    event_buffer.push(GUIEvent::CreateSemanticNodeRequest);
                } 
            })
        });
    } 

    pub fn set_visibility(&mut self, visibility: bool) {
        self.visibility = visibility;
    }

}
