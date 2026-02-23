pub mod project_layouts;

use self::{
    project_layouts::ProjectLayouts,
};

#[derive(Debug, Default)]
pub struct ProjectTemplate {
    pub project_layouts: ProjectLayouts, 
}
