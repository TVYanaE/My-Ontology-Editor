use super::{
    super::{
        super::{
            events::{
                LogicEvent, 
                EventSender, TaskKind,
                TaskResult, 
            }, 
            event_manager::{
                EventManager,
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
