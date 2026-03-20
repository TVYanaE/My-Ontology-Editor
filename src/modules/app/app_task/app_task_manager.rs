use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
use tokio::runtime::Runtime;
use eframe::egui::Context as EGUIContext;

use super::{AppBlockingTask, AppAsyncTask};
use super::super::app_event::{AppEvent, AppEvents};

pub struct AppTaskManager {
    runtime: Runtime,
    tx: UnboundedSender<Option<AppEvent>>,
    rx: UnboundedReceiver<Option<AppEvent>>,
    app_events: AppEvents
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
            rx: rx,
            app_events: AppEvents::new(),
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

    pub fn schedule_app_event(
        &mut self,
        event: AppEvent,
        egui_context: EGUIContext,
    ) {
        self.app_events.push(event);
        egui_context.request_repaint();
    }

    pub fn check_tasks(&mut self) {
        while let Ok(event) = self.rx.try_recv() {
            if let Some(event) = event {
                self.app_events.push(event);
            }
        } 
    }

    pub fn check_events(&mut self) -> impl Iterator<Item = AppEvent> {    
        self.app_events.drain()
    }
}
