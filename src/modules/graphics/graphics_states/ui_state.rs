pub mod create_new_project_window_state;
pub mod ui_general_state;

use self::{
    create_new_project_window_state::CreateNewProjectWindowState,
    ui_general_state::UIGeneralState,
};

#[derive(Debug, Default)]
pub struct UIState {
    pub ui_general_state: UIGeneralState,
    pub create_new_project_window_state: CreateNewProjectWindowState,
}




