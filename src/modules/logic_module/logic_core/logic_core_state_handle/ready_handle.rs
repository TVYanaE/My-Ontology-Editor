use tracing::{
    instrument,
};
use super::{
    super::{
        super::{
            db_core::DBCore,
            logic_module_io::{
                event_sender::EventSender,
                event_manager::EventManager,
            },
            project_manager::{
                ProjectManager,
            },
            job_manager::{
                JobManager, Job, JobKind,
            },
            project_cache::{
                ProjectCache,
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
    pub db_core: &'c mut DBCore,
    pub job_manager: &'c mut JobManager, 
    pub confirmation_cache: &'c mut ConfirmationCache,
    pub project_cache: &'c mut ProjectCache, 
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
                    context.project_cache,
                    context.event_manager, 
                    context.db_core,
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
                let jobs = LogicCoreLogic::shutdown();

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
