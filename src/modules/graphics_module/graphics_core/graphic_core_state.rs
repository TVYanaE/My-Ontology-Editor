use crate::{
    modules::{
        logic_module::{
            prelude::{
                TaskID,
            },
        },
    },
};

#[derive(Debug, Clone)]
pub enum GraphicsCoreState {
    Processing,
    Runnig,
    WaitingTask {
        task_id: TaskID, 
    },
    Shutdown,
}

impl Default for GraphicsCoreState {
    fn default() -> Self {
        Self::Runnig
    }
}
