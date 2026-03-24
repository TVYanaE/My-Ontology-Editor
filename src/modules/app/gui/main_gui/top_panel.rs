mod main_bar;
mod projects_bar;

use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::panel::{
    TopBottomPanel, TopBottomSide,
};

use crate::modules::app::gui::gui_event::GUIEventBuffer;

use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::project_view::ProjectView;

use self::main_bar::MainBar;
use self::projects_bar::ProjectsBar;

pub struct TopPanel {
    main_bar: MainBar,
    projects_bar: ProjectsBar,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            main_bar: MainBar::new(),
            projects_bar: ProjectsBar::new(),
        }
    }
    pub fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
        project_views: &[(&ProjectID, &mut ProjectView)],
    ) {
        TopBottomPanel::new(TopBottomSide::Top, "Top-Panel")
            .show(context, |top_panel_ui|{
                self.main_bar.prepare(top_panel_ui, event_buffer);

                top_panel_ui.separator();

                self.projects_bar.prepare(top_panel_ui, event_buffer, project_views);
            }
        ); 
    }
}
