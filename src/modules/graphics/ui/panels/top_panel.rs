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

pub struct TopPanelContext<'c> {
    pub egui_context: &'c EGUIContext,
    pub ui_affects: &'c mut UIAffects,
    pub top_panel_data: &'c mut TopPanelData,
}

pub fn top_panel(
    top_panel_context: TopPanelContext,
) {
    TopBottomPanel::new(TopBottomSide::Top, "Top-Panel")
        .show(top_panel_context.egui_context, |top_panel_ui|{
            top_panel_ui.vertical(|vertical_ui|{
                main_bar(
                    MainBarContext { 
                        ui: vertical_ui, 
                        ui_affects: top_panel_context.ui_affects, 
                    },
                );
            }) 

        });
}
