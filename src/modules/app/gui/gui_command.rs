use super::super::confirmation_context::ConfirmationContextID;

#[derive(Debug, Clone)]
pub enum GUICommand {
    ShowCreateProjectWindow, 
    ShowNotification(String),
    ShowConfirmationWindow{
        confirmation_type: ConfirmationType,
        confirmation_text: String,
    },
}

#[derive(Debug, Clone)]
pub enum ConfirmationType {
    OverwriteProjectFile(ConfirmationContextID),
}
