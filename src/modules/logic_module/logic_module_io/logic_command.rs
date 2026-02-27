use calloop::{
    channel::Sender,
};
use super::{
    TaskID, TaskKind,
    ConfirmationID, DecisionKind,
};

pub type LogicCommands = Sender<LogicCommand>;

#[derive(Debug, Clone)]
pub enum LogicCommand {
    Task {
        task_id: TaskID,
        task_kind: TaskKind,
    },
    ConfirmationDecision {
        confirmation_id: ConfirmationID,
        decision: bool,
        decision_kind: DecisionKind, 
    },
    Shutdown, 
}
