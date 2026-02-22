use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
    },
};
use super::{
    super::{
        super::{
            events::{
                LogicCommand, EventSender,
                TaskKind, ConfirmationID,
                DecisionKind,
            },
        },
        logic_core_logic::{
            LogicCoreLogic,
            CreateProjectContext,
            WorkAfterConfirmation,
        },
        logic_core_state::LogicCoreState,
        LogicCoreError
    },
    LogicCoreStateHandle
};

impl LogicCoreStateHandle {
    pub fn waiting_confirmation_handle<S: EventSender>(
        command: LogicCommand,
        app_dirs: &ApplicationDirectories,
        event_sender: &S,
        work: WorkAfterConfirmation,
        waiting_confirmation_id: ConfirmationID,
    ) -> Result<Option<LogicCoreState>, LogicCoreError<S>> {
        match command {
            LogicCommand::Task{
                task_id,
                task_kind, 
            } => {
                Ok(None) 
            },
            LogicCommand::Shutdown => {
                Ok(Some(LogicCoreState::Shutdown))
            },
            LogicCommand::ConfirmationDecision { 
                confirmation_id, 
                decision, 
                decision_kind 
            } => {
                if waiting_confirmation_id == confirmation_id {
                    match decision_kind {
                        DecisionKind::Owerrite => {
                            match work {
                                WorkAfterConfirmation::CreateProject { 
                                    task_id, 
                                    project_name, 
                                    project_path 
                                } => {
                                    let new_state = LogicCoreLogic::for_test(
                                        task_id, 
                                        project_name, 
                                        project_path, 
                                        decision, 
                                        event_sender
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
        } 
    }
}
