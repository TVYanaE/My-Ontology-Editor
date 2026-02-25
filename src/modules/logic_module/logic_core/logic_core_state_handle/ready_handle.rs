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
                EventSender,
            },
            project_manager::{
                ProjectManager,
            },
            event_manager::EventManager,
            job_manager::{
                JobManager, Job, JobKind,
            },
            confirmation_cache::ConfirmationCache,
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
    pub event_manager: &'c EventManager<S>,
    pub project_manager: &'c ProjectManager,
    pub db_module_handler: &'c mut DBModuleHandler,
    pub job_manager: &'c mut JobManager, 
    pub confirmation_cache: &'c mut ConfirmationCache,
}

impl LogicCoreStateHandle {
    #[instrument(skip_all,err)]
    pub fn ready_handle<S: EventSender>(
        job: Job,
        context: ReadyStateContext<S>
    ) -> Result<Option<LogicCoreState>, LogicCoreError<S>> {
        match job.kind {
            JobKind::CheckCreatingProjectPath { 
                task_id, 
                project_name, 
                project_path 
            } => {
                let jobs = LogicCoreLogic::check_creating_project_path(
                    task_id, 
                    &project_name, 
                    &project_path, 
                    context.event_manager,
                    context.confirmation_cache,
                )?; 

                if !jobs.is_empty() {
                    for job in jobs {
                        context.job_manager.push_job(job);
                    }
                } 

                Ok(Some(LogicCoreState::Ready))
            },
            JobKind::CreateProject { 
                task_id, 
                project_name, 
                project_path 
            } => {
                let jobs = LogicCoreLogic::create_project(
                    task_id, 
                    &project_name, 
                    &project_path, 
                    context.project_manager, 
                    context.event_manager, 
                    &context.db_module_handler.db_commands,
                )?;

                if !jobs.is_empty() {
                    for job in jobs {
                        context.job_manager.push_job(job);
                    }
                } 

                Ok(Some(LogicCoreState::Ready))
            },
            JobKind::ConfirmationDecline { 
                confirmation_context 
            } => {
                let jobs = LogicCoreLogic::confirmation_decline(
                    context.event_manager, 
                    confirmation_context,
                )?;

                if !jobs.is_empty() {
                    for job in jobs {
                        context.job_manager.push_job(job);
                    }
                } 

                Ok(Some(LogicCoreState::Ready))
            },
            JobKind::Shutdown => {
                let jobs = LogicCoreLogic::shutdown(
                    context.db_module_handler
                );

                if !jobs.is_empty() {
                    for job in jobs {
                        context.job_manager.push_job(job);
                    }
                }

                Ok(Some(LogicCoreState::Shutdown))
            },
            _ => {Ok(None)}             
        } 
    }
}
