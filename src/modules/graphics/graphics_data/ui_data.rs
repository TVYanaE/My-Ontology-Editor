pub mod modal_windows_data;
pub mod panels_data;

use self::{
    modal_windows_data::ModalWindowsData,  
    panels_data::PanelsData
};

#[derive(Default)]
pub struct UIData {
    pub panels_data: PanelsData,
    pub modal_windows_data: ModalWindowsData,
}



