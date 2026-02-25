mod task_context;

use std::{
    collections::{
        HashMap,
    },
};
use crate::{
    modules::{
        logic_module::{
            events::TaskID,
        }, 
    },
};
pub use self::{
    task_context::TaskContext,
};

pub struct TaskCache {
    tasks: HashMap<TaskID, TaskContext> 
}

impl TaskCache {
    pub fn new() -> Self {
        Self { 
            tasks: HashMap::with_capacity(4), 
        }
    }

    pub fn push(
        &mut self,
        task_id: TaskID,
        task_context: TaskContext,
    ) {
        self.tasks.insert(task_id, task_context);
    }

    pub fn remove(
        &mut self,
        task_id: &TaskID
    ) -> Option<TaskContext> {
        self.tasks.remove(task_id)
    }
}
