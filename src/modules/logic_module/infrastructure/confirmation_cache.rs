mod confirmation_context;

use std::{
    collections::{
        HashMap,
    },
};
use super::{
    super::{
        prelude::{
            ConfirmationID, ConfirmationKind,
        },
    },
};
pub use self::{
    confirmation_context::ConfirmationContext,
};


pub struct ConfirmationCache {
    confirmations: HashMap<ConfirmationID, ConfirmationContext> 
}

impl ConfirmationCache {
    pub fn new() -> Self {
        Self { 
            confirmations: HashMap::with_capacity(8), 
        }
    }
    
    pub fn push(
        &mut self,
        confirmation_id: ConfirmationID,
        confirmation_kind: ConfirmationKind,
    ) {
        self.confirmations.insert(
            confirmation_id, 
            confirmation_kind.into()  
        );
    }
    
    pub fn remove(
        &mut self,
        confirmation_id: ConfirmationID,
    ) -> Option<ConfirmationContext> {
        self.confirmations.remove(&confirmation_id)
    }
}
