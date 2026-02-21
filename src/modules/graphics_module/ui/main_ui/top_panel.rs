pub mod main_bar;

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
        graphics_module::{
            ui::{
                events::UIEvents,
                ui_error::UIError,
            },
        },
    },
};
use self::{
    main_bar::MainBar, 
};

#[derive(Default)]
pub struct TopPanel{
    main_bar: MainBar
}

impl TopPanel {
    pub(super) fn prepare(
        &mut self,
        egui_context: &EGUIContext,
    ) -> Result<UIEvents, UIError> {
        let mut ui_events = UIEvents::with_capacity(8);

        let inner_resp = TopBottomPanel::new(TopBottomSide::Top, "Top-Panel")
        .show(egui_context, |top_panel_ui|{
            self.main_bar.prepare(top_panel_ui)
        });

        let main_bar_events = inner_resp.inner?;

        ui_events.extend(main_bar_events.into_iter());

        Ok(ui_events)
    }
}
