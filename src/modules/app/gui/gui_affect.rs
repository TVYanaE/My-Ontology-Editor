use crate::modules::app::gui::gui_command::ConfirmationType;

use crate::modules::app::project::project_id::ProjectID;

#[derive(Debug)]
pub enum GUIAffect {
    ExitRequested,
    CreateProjectInfo {
        project_name: String,
        project_path: String,
    },
    ConfirmationObtain {
        confirmation_type: ConfirmationType,
        decision: bool,
    },
    OpenProjectInfo {
        project_file_path: String, 
    },
    SwitchProjectRequest {
        project_id: ProjectID,
    }
}

pub struct GUIAffectBuffer(Vec<GUIAffect>);

impl GUIAffectBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    } 
    pub fn push(&mut self, gui_affect: GUIAffect) {
        self.0.push(gui_affect);
    }
}

impl IntoIterator for GUIAffectBuffer {
    type Item = GUIAffect;
    type IntoIter = std::vec::IntoIter<GUIAffect>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
