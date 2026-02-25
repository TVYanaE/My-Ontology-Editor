use super::{
    GraphicsCoreLogic,
};
use crate::{
    modules::{
        logic_module::{
            events::{
                LogicCommand, 
                DecisionKind, 
                ConfirmationID,
            }, 
            logic_module_handler::LogicModuleHandler,
        },
        graphics_module::{
            graphics_core::{
                graphic_core_error::GraphicsCoreError,
                GraphicsCoreState,
            },
        },  
    },
};

impl GraphicsCoreLogic {
    pub fn confirmation_obtain(
        confirmation_id: ConfirmationID,
        decision: bool,
        decision_kind: DecisionKind,
        logic_module_handler: &mut LogicModuleHandler,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        logic_module_handler.logic_commands.send(
            LogicCommand::ConfirmationDecision { 
                confirmation_id: confirmation_id, 
                decision: decision, 
                decision_kind: decision_kind, 
            }
        )?;

        Ok(None)
    } 
}
