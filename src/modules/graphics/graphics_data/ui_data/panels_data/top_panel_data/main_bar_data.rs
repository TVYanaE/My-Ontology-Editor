
pub struct MainBarData {
    pub menu_button_hovered: bool,
}

impl Default for MainBarData {
    fn default() -> Self {
        Self { 
            menu_button_hovered: false 
        }
    }
}
