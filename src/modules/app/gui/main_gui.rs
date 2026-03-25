mod bottom_panel;
mod central_panel;
mod left_panel;
mod top_panel;

use eframe::egui::Context as EGUIContext;

use crate::modules::app::gui::gui_event::GUIEventBuffer;

use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::project_view::ProjectView;

use self::bottom_panel::BottomPanel;
use self::central_panel::CentralPanel;
use self::left_panel::LeftPanel;
use self::top_panel::TopPanel;

pub struct MainGUI {
    bottom_panel: BottomPanel,
    central_panel: CentralPanel,
    left_panel: LeftPanel,
    top_panel: TopPanel,
}

impl MainGUI {
    pub fn new() -> Self {
        Self {  
            bottom_panel: BottomPanel::new(),
            central_panel: CentralPanel::new(),
            left_panel: LeftPanel::new(),
            top_panel: TopPanel::new(),
        }
    }

    pub fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
        project_views: &[(&ProjectID, &ProjectView)],
        selected_project: Option<&ProjectView>,
    ) {
        self.top_panel.prepare(context, event_buffer, project_views, selected_project); 
        self.bottom_panel.prepare(context, event_buffer, selected_project);
        self.left_panel.prepare(context, event_buffer, selected_project);
        self.central_panel.prepare(context, event_buffer);
    }

    pub fn with_left_panel<F>(&mut self, f: F) 
    where 
        F: FnOnce(&mut LeftPanel)
    {
        f(&mut self.left_panel)
    }
}
