pub mod confirmation_context_manager; 

use super::id::{IDType, IDGenerator};

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct ConfirmationContextID(u64);

impl IDType for ConfirmationContextID {
    fn from_u64(value: u64) -> Self {
        Self(value)
    }
}

pub static CONFIRMATION_CONTEXT_ID_GENERATOR: IDGenerator<ConfirmationContextID> = IDGenerator::new();

pub enum ConfirmationContext {
    OverwriteProjectFileContext {
        project_name: String,
        project_path: String,
    }, 
}
