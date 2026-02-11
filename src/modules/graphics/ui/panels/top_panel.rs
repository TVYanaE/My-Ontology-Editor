mod main_bar;

use egui::{
    containers::{
        panel::{
            TopBottomPanel, TopBottomSide
        },
    },
};
use crate::{
    aliases::{
        EGUIContext 
    },
    modules::{
        graphics::{
            events::UIAffects,
        },
    },
};
use self::{
    main_bar::main_bar, 
};

pub fn top_panel(egui_context: &EGUIContext, ui_affects: &mut UIAffects) {
    TopBottomPanel::new(TopBottomSide::Top, "Top-Panel")
        .show(egui_context, |top_panel_ui|{
            top_panel_ui.vertical(|vertical_ui|{
                main_bar(vertical_ui, egui_context, ui_affects);
            }) 

        });
}
