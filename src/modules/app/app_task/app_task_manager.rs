use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
use tokio::runtime::Runtime;
use eframe::egui::Context as EGUIContext;

use super::{AppBlockingTask, AppAsyncTask};
use super::super::app_event::{AppEvent, ExternalAppEvents};

//type CallbackTask = Box<dyn FnOnce() + Send>;

pub struct AppTaskManager {
    runtime: Runtime,
    tx: UnboundedSender<Option<AppEvent>>,
    rx: UnboundedReceiver<Option<AppEvent>>,
}

impl AppTaskManager {
    pub fn new(runtime: Runtime) -> Self {
        let (
            tx, 
            rx
        ) = unbounded_channel();

        Self { 
            runtime: runtime,
            tx: tx,
            rx: rx
        }
    } 

    pub fn schedule_async<Fut, CB, R>(
        &mut self, 
        app_task: AppAsyncTask<Fut, CB, R>, 
        egui_context: EGUIContext
    ) 
    where
        R: Send + 'static,
        Fut: Future<Output = R> + Send + 'static,
        CB: FnOnce(R) -> Option<AppEvent> + Send + 'static,  
    {
        let func = app_task.task;
        let callback = app_task.callback;

        let tx = self.tx.clone();        

        self.runtime.spawn(async move {
            let result = func.await; 

            let event = callback(result);

            tx.send(event).unwrap();
            egui_context.request_repaint();
        });
    }

    pub fn schedule_blocking<F, CB, R>(
        &mut self, 
        app_task: AppBlockingTask<F, CB, R>, 
        egui_context: EGUIContext
    ) 
    where
        R: Send + 'static,
        F: FnOnce() -> R + Send + 'static,
        CB: FnOnce(R) -> Option<AppEvent> + Send + 'static,  
    {
        let func = app_task.task;
        let callback = app_task.callback;

        let tx = self.tx.clone();        

        self.runtime.spawn_blocking(move ||{
            let result = func(); 

            let event = callback(result);

            tx.send(event).unwrap();
            egui_context.request_repaint();
        });
    }

    pub fn run(&mut self, external_app_events: &mut ExternalAppEvents) {
        while let Ok(event) = self.rx.try_recv() {
            if let Some(event) = event {
                external_app_events.push(event);
            }
        } 
    }
}
