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
            graphics_data::{
                ui_data::{
                    panels_data::{
                        top_panel_data::TopPanelData,
                    },
                },
            },
        },
    },
};
use self::{
    main_bar::{main_bar, MainBarContext}, 
};

pub fn top_panel(
    egui_context: &EGUIContext, 
    ui_affects: &mut UIAffects,
    top_panel_data: &mut TopPanelData,
) {
    TopBottomPanel::new(TopBottomSide::Top, "Top-Panel")
        .show(egui_context, |top_panel_ui|{
            top_panel_ui.vertical(|vertical_ui|{
                main_bar(
                    MainBarContext { 
                        ui: vertical_ui, 
                        ui_affects: ui_affects, 
                    },
                );
            }) 

        });
}
