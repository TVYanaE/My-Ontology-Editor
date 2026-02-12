pub mod create_new_project_window_data;

use self::{
    create_new_project_window_data::CreateNewProjectWindowData,
};

#[derive(Default)]
pub struct ModalWindowsData {
    pub create_new_project_window_data: CreateNewProjectWindowData,
}
