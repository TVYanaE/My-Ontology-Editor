use std::collections::hash_map::HashMap;

use super::{ConfirmationContextID, ConfirmationContext};

pub struct ConfirmationContextManager {
    confirmation_contexts: HashMap<ConfirmationContextID, ConfirmationContext>,
}

impl ConfirmationContextManager {
    pub fn new() -> Self {
        Self { 
            confirmation_contexts: HashMap::with_capacity(8), 
        }
    }
    pub fn push(
        &mut self,
        confirmation_context_id: ConfirmationContextID,
        confirmation_context: ConfirmationContext,
    ) {
        self.confirmation_contexts.insert(
            confirmation_context_id, 
            confirmation_context
        );
    }
    pub fn remove(
        &mut self,
        confirmation_context_id: &ConfirmationContextID 
    ) -> Option<ConfirmationContext> {
        self.confirmation_contexts.remove(confirmation_context_id)
    } 
}
