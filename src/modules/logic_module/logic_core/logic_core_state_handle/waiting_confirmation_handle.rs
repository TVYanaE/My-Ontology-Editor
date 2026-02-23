use tracing::{
    instrument,
};
use crate::{
    modules::{
        db_module::DBModuleHandler,
    },
};
use super::{
    super::{
        super::{
            events::{
                LogicCommand, EventSender,
                ConfirmationID,
                DecisionKind,
            },
            project_manager::ProjectManager,
        },
        logic_core_logic::{
            LogicCoreLogic,
            WorkAfterConfirmation,
        },
        logic_core_state::LogicCoreState,
        logic_core_error::LogicCoreError,
    },
    LogicCoreStateHandle
};

pub struct WaitingConfirmationStateContext<'c, S: EventSender> {
    pub event_sender: &'c S,
    pub work: WorkAfterConfirmation,
    pub waiting_confirmation_id: ConfirmationID,
    pub project_manager: &'c ProjectManager,
    pub db_module_handler: &'c mut DBModuleHandler,
}

impl LogicCoreStateHandle {
    #[instrument(skip_all,err)]
    pub fn waiting_confirmation_handle<S: EventSender>(
        command: LogicCommand,
        context: WaitingConfirmationStateContext<S> 
    ) -> Result<Option<LogicCoreState>, LogicCoreError<S>> {
        match command { 
            LogicCommand::Shutdown => {
                let new_state = LogicCoreLogic::shutdown(
                    context.db_module_handler
                );

                Ok(new_state)
            },
            LogicCommand::ConfirmationDecision { 
                confirmation_id, 
                decision, 
                decision_kind 
            } => {
                if context.waiting_confirmation_id == confirmation_id {
                    match decision_kind {
                        DecisionKind::Owerrite => {
                            match context.work {
                                WorkAfterConfirmation::CreateProject { 
                                    task_id, 
                                    project_name, 
                                    project_path 
                                } => {
                                    let new_state = LogicCoreLogic::create_project(
                                        &task_id, 
                                        Some(decision),
                                        &project_name, 
                                        &project_path, 
                                        context.project_manager,
                                        context.event_sender,
                                        &context.db_module_handler.db_commands,
                                    )?; 

                                    Ok(new_state)
                                }
                            }
                        },
                    } 
                }
                else {
                    Ok(None)
                }
            },
            _ => Ok(None)
        } 
    }
}
