use crate::modules::app::confirmation_context::ConfirmationContextID;

use crate::modules::app::project::project_id::ProjectID;

#[derive(Debug, Clone)]
pub enum GUICommand {
    ShowNotification(String),
    ShowConfirmationWindow{
        confirmation_type: ConfirmationType,
        confirmation_text: String,
    },
    ShowLoading,
    StopShowLoading,
    ShowMainUI, 
    SelectProject {
        project_id: ProjectID,
    },
    ProjectOpened {
        project_id: ProjectID,
    },
    ProjectCreated {
        project_id: ProjectID,
    }
}

#[derive(Debug, Clone)]
pub enum ConfirmationType {
    OverwriteProjectFile(ConfirmationContextID),
}
