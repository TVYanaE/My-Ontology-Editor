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
                TaskKind,
            },
        },
        logic_core_logic::{
            LogicCoreLogic,
            CreateProjectContext,
        },
        logic_core_state::LogicCoreState,
        LogicCoreError
    },
    LogicCoreStateHandle
};


impl LogicCoreStateHandle {
    pub fn ready_handle<S: EventSender>(
        command: LogicCommand,
        app_dirs: &ApplicationDirectories,
        event_sender: &S,
    ) -> Result<Option<LogicCoreState>, LogicCoreError<S>> {
        match command {
            LogicCommand::Task{
                task_id,
                task_kind, 
            } => {
                match task_kind {
                    TaskKind::CreateProject { project_name, project_path } => {
                        std::thread::sleep(std::time::Duration::from_secs(1)); 
                        
                        let new_state = LogicCoreLogic::create_project_handle(
                            CreateProjectContext { 
                                app_dirs, 
                                project_name, 
                                project_path, 
                                task_id, 
                                event_sender: event_sender      
                            }
                        )?;

                        Ok(new_state)
                    }, 
                } 
            },
            LogicCommand::Shutdown => {
                Ok(Some(LogicCoreState::Shutdown))
            },
            _ => {Ok(None)}             
        } 
    }
}
