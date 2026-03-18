pub mod app_task_manager;

use super::app_event::AppEvent;

#[derive(Debug, Clone)]
pub struct AppBlockingTask<F, CB, R>
where 
    R: Send + 'static,
    F: FnOnce() -> R + Send + 'static,
    CB: FnOnce(R) -> Option<AppEvent> + Send + 'static, 
{
    pub task: F,
    pub callback: CB,
}

#[derive(Debug, Clone)]
pub struct AppAsyncTask<Fut, CB, R>
where 
    R: Send + 'static,
    Fut: Future<Output = R> + Send + 'static,
    CB: FnOnce(R) -> Option<AppEvent> + Send + 'static, 
{
    pub task: Fut,
    pub callback: CB,
}
