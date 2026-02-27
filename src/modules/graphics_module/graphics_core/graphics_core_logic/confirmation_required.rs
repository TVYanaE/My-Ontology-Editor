use super::{
    GraphicsCoreLogic,
};
use crate::{
    modules::{
        logic_module::{
            prelude::{
                ConfirmationID, ConfirmationKind,
            }, 
        },
        graphics_module::{
            graphics_core::{
                graphic_core_error::GraphicsCoreError,
                GraphicsCoreState,
            },
            ui::{UI, UIInputEvent, ChosedModalWindow},
        },  
    },
};

impl GraphicsCoreLogic {
    pub fn confirmation_required(
        ui: &mut UI,
        confirmation_id: ConfirmationID,
        confirmation_kind: ConfirmationKind,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        match confirmation_kind.clone() {
            ConfirmationKind::Owerrite { project_name, .. } => {
                let confirmation_text = format!("Project {} already extists. Replace?", project_name);

                ui.on_event(
                    UIInputEvent::ShowModalWindow(
                        ChosedModalWindow::ConfirmationWindow { 
                            confirmation_id, 
                            confirmation_kind, 
                            text: confirmation_text 
                        }
                    )
                )?; 
                Ok(None)
            },
        }
    } 
}
