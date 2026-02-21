use uuid::{
    Uuid,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskID(pub Uuid);
