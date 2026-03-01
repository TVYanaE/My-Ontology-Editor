mod logic_core_error;
mod logic_core_error_handle;
mod logic_core_logic;
mod logic_core_state;
mod logic_core_state_handle;


use super::{
    db_core::DBCore,
    infrastructure::{
        logic_module_io::{
            event_sender::EventSender,
            event_manager::EventManager,
        },
        job_manager::{
            Job, JobManager,
        },
        confirmation_cache::{
            ConfirmationCache,
        },
    },
    project_manager::{
        ProjectManager,
    },
    project_cache::{
        ProjectCache,
    },
};
use self::{
    logic_core_error_handle::logic_core_error_handle,
    logic_core_state::LogicCoreState,
    logic_core_state_handle::{
        LogicCoreStateHandle,
        ReadyStateContext,
    },
};

pub struct LogicCore {
    logic_core_state: LogicCoreState,  
}

pub struct JobContext<'c, S: EventSender> {
    pub db_core: &'c mut DBCore,
    pub project_manager: &'c ProjectManager,
    pub event_manager: &'c EventManager<S>,
    pub job_manager: &'c mut JobManager,
    pub confirmation_cache: &'c mut ConfirmationCache,
    pub project_cache: &'c mut ProjectCache,
}

impl LogicCore {
    pub fn new() -> Self {
        Self {
            logic_core_state: LogicCoreState::default(),
        }
    }
    
    pub fn on_job<S: EventSender>(
        &mut self, 
        job: Job,
        context: JobContext<S> 
    ) {
        let current_state = std::mem::replace(
            &mut self.logic_core_state, 
            LogicCoreState::Processing
        ); 

        self.logic_core_state = match (current_state, job) {
            (LogicCoreState::Ready, job) => {
                match LogicCoreStateHandle::ready_handle(
                    job, 
                    ReadyStateContext { 
                        event_manager: context.event_manager, 
                        project_manager: context.project_manager, 
                        db_core: context.db_core, 
                        job_manager: context.job_manager, 
                        confirmation_cache: context.confirmation_cache, 
                        project_cache: context.project_cache,
                    } 
                ) {
                    Ok(Some(new_state)) => new_state,
                    Ok(None) => LogicCoreState::Ready,
                    Err(error) => {
                        if let Some(new_state) = logic_core_error_handle(
                            error, 
                            context.event_manager,
                        ) {
                            new_state
                        } 
                        else {
                            LogicCoreState::Ready
                        } 
                    },
                }              
            }, 
            (current_state,_) => current_state,
        }
    }

    pub fn event_loop_closed_handle(&mut self) {
        self.logic_core_state = LogicCoreState::Shutdown;
    }

    pub fn is_shutdown(&self) -> bool {
        match &self.logic_core_state {
            LogicCoreState::Shutdown => true,
            _ => false
        }
    }
}
