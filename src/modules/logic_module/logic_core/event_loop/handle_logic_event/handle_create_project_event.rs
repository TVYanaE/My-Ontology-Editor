use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        logic_module::{
            events::{
                ProjectDescriptor,    
            }, 
        },
    },
}; 
use super::{
    LogicEventError,
};

pub struct CreateProjectEventContext<'c> {
    pub app_dirs: &'c ApplicationDirectories,
}

pub fn handle_create_project_event(
    project_descriptor: ProjectDescriptor,
    create_project_event_context: CreateProjectEventContext,
) -> Result<(), LogicEventError> {
     
     

    Ok(())
}
