use super::{
    super::{
        super::{
            logic_module_io::{
                logic_event::LogicEvent,
                event_sender::EventSender,
                event_manager::EventManager,
                TaskResult,
            },
            job_manager::{
                Job, 
            },
            confirmation_cache::{
                ConfirmationContext,
            },
        }, 
        logic_core_error::LogicCoreError,
    },
    LogicCoreLogic, 
};

impl LogicCoreLogic {
    pub fn confirmation_decline<S: EventSender>(
        event_manager: &EventManager<S>,
        context: ConfirmationContext,
    ) -> Result<Vec<Job>, LogicCoreError<S>> {
        let jobs = Vec::with_capacity(2);
   
        match context {
            ConfirmationContext::CreateProjectContext { 
                task_id, 
                .. 
            } => {
                event_manager.send_event(
                    LogicEvent::TaskRespone { 
                        task_id: task_id,  
                        task_result: TaskResult::CanceledByUser,
                    }
                )?;
            },
        }

        Ok(jobs)
    } 
}
