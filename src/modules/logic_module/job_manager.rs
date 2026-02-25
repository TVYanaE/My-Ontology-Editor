mod job;
mod job_manager_error;

use std::{
    collections::{
        VecDeque,
    }, 
};
use super::{
    events::{
        LogicCommand, TaskKind,
        DecisionKind,
    },
    confirmation_cache::{
        ConfirmationCache, ConfirmationContext,
    }, 
};
pub use self::{
    job::{
        JobKind, JobID, Job
    },
    job_manager_error::JobManagerError,
};

pub struct JobManager {
    jobs: VecDeque<Job>, 
}


impl JobManager{
    pub fn new() -> Self {
        Self { 
            jobs: VecDeque::with_capacity(8), 
        }
    }

    pub fn on_command(
        &mut self,
        command: LogicCommand, 
        confirmation_cache: &mut ConfirmationCache,
    ) {
        match command {
            LogicCommand::Shutdown => {
                self.jobs.push_back(
                    Job { 
                        id: JobID::new(), 
                        kind: JobKind::Shutdown 
                    }
                );
            },
            LogicCommand::Task { 
                task_id, 
                task_kind 
            } => {
                match task_kind {
                    TaskKind::CreateProject { project_name, project_path } => {
                        self.jobs.push_back(
                            Job { 
                                id: JobID::new(), 
                                kind: JobKind::CheckCreatingProjectPath { 
                                    task_id, 
                                    project_name, 
                                    project_path 
                                } 
                            }
                        ); 
                    },
                }    
            },
            LogicCommand::ConfirmationDecision { 
                confirmation_id, 
                decision, 
                decision_kind 
            } => {
                if !decision {
                    if let Some(context) = confirmation_cache.remove(confirmation_id) {
                        self.jobs.push_back(
                            Job { 
                                id: JobID::new(), 
                                kind: JobKind::ConfirmationDecline { 
                                    confirmation_context: context 
                                } 
                            }
                        );
                    } 
                    
                    return;
                }    

                match decision_kind {
                    DecisionKind::Owerrite => {
                        if let Some(context) = confirmation_cache.remove(confirmation_id) {
                            match context {
                                ConfirmationContext::CreateProjectContext { 
                                    task_id, 
                                    project_name, 
                                    project_path 
                                } => {
                                    self.jobs.push_back(
                                        Job { 
                                            id: JobID::new(), 
                                            kind: JobKind::CreateProject { 
                                                task_id: task_id, 
                                                project_name: project_name, 
                                                project_path: project_path
                                            }, 
                                        }
                                    );
                                },
                                _ => {}
                            }
                        }
                    },
                }
            }, 
        }; 
    }

    pub fn push_job(&mut self, job: Job) {
        self.jobs.push_back(job);
    }

    pub fn pop_front(&mut self) -> Option<Job> {
        self.jobs.pop_front()
    } 
} 


