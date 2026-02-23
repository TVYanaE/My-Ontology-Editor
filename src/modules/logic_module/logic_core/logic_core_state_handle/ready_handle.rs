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
                TaskKind,
            },
            project_manager::{
                ProjectManager,
            },
        },
        logic_core_logic::{
            LogicCoreLogic,
        },
        logic_core_state::LogicCoreState,
        logic_core_error::LogicCoreError,
    },
    LogicCoreStateHandle
};

pub struct ReadyStateContext<'c, S: EventSender> {
    pub event_sender: &'c S,
    pub project_manager: &'c ProjectManager,
    pub db_module_handler: &'c mut DBModuleHandler,
}

impl LogicCoreStateHandle {
    #[instrument(skip_all,err)]
    pub fn ready_handle<S: EventSender>(
        command: LogicCommand,
        context: ReadyStateContext<S>
    ) -> Result<Option<LogicCoreState>, LogicCoreError<S>> {
        match command {
            LogicCommand::Task{
                task_id,
                task_kind, 
            } => {
                match task_kind {
                    TaskKind::CreateProject { project_name, project_path } => {
                        std::thread::sleep(std::time::Duration::from_millis(500)); 
                        
                        match LogicCoreLogic::check_creating_project_path(
                            &task_id, 
                            &project_name, 
                            &project_path, 
                            context.event_sender,
                        )? {
                            Some(new_state) => {
                                Ok(Some(new_state))
                            },
                            None => {
                                let new_state = LogicCoreLogic::create_project(
                                    &task_id, 
                                    None,
                                    &project_name, 
                                    &project_path, 
                                    context.project_manager,
                                    context.event_sender,
                                    &context.db_module_handler.db_commands,
                                )?; 

                                Ok(new_state)
                            },
                        } 
                    }, 
                } 
            },
            LogicCommand::Shutdown => {
                let new_state = LogicCoreLogic::shutdown(
                    context.db_module_handler
                );

                Ok(new_state)
            },
            _ => {Ok(None)}             
        } 
    }
}
