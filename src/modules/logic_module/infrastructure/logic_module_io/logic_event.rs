use super::{
    TaskID, TaskResult,
    ConfirmationKind, ConfirmationID,
};


#[derive(Debug, Clone)]
pub enum LogicEvent {
    TaskRespone {
        task_id: TaskID,
        task_result: TaskResult,
    }, 
    ConfirmationRequested {
        confirmation_id: ConfirmationID,
        confirmation_kind: ConfirmationKind,
    }, 
    Shutdown,
}
