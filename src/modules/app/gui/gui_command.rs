use crate::modules::app::confirmation_context::ConfirmationContextID;

#[derive(Debug, Clone)]
pub enum GUICommand {
    ShowNotification(String),
    ShowConfirmationWindow{
        confirmation_type: ConfirmationType,
        confirmation_text: String,
    },
    ShowLoading,
    StopShowLoading,
}

#[derive(Debug, Clone)]
pub enum ConfirmationType {
    OverwriteProjectFile(ConfirmationContextID),
}
