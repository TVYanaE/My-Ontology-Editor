pub mod app_dirs;
mod app_error;
mod app_error_hanlding;
mod app_event;
mod app_kernel; 
mod app_state;
mod app_task;
mod confirmation_context;
mod gui;
mod id;
mod project;
mod time_detector;

use std::sync::Arc;

use eframe::CreationContext;
use eframe::App as EFrameApp;
use eframe::egui::{Context, ViewportCommand};
use eframe::Frame;
use tokio::runtime::Runtime;

use self::app_dirs::AppDirs;
use self::app_error_hanlding::app_error_handling;
use self::app_event::{AppEvent, ExternalAppEvents};
use self::app_kernel::AppKernel;
use self::app_state::AppState;
use self::app_task::app_task_manager::AppTaskManager;
use self::confirmation_context::confirmation_context_manager::ConfirmationContextManager;
use self::gui::GUI;
use self::time_detector::TimeDetector;


pub struct App {
    state: AppState, 
    kernel: AppKernel,
    gui: GUI,
    time_detector: TimeDetector,
    app_dirs: Arc<AppDirs>,
    app_task_manager: AppTaskManager,
    external_app_events: ExternalAppEvents,
    confirmation_context_manager: ConfirmationContextManager, 
}

impl App {
    pub fn new(
        _creation_context: &CreationContext,
        app_dirs: AppDirs,
        runtime: Runtime,
    ) -> Self {
        
        App {
            state: AppState::default(),
            kernel: AppKernel::new(),
            gui: GUI::new(),
            time_detector: TimeDetector::new(),
            app_dirs: Arc::new(app_dirs),
            app_task_manager: AppTaskManager::new(runtime),
            external_app_events: ExternalAppEvents::new(),
            confirmation_context_manager: ConfirmationContextManager::new(),
        }
    }
} 

impl EFrameApp for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.time_detector.start_measurement();

        match self.gui.prepare_gui(ctx) {
            Ok(gui_affect_buffer) => {
                for gui_affect in gui_affect_buffer {
                    match self.kernel.gui_affects_handling(
                        gui_affect, 
                        &mut self.gui, 
                        &self.state,
                        &mut self.confirmation_context_manager,
                    ) {
                        Ok(event_opt) => {
                            if let Some(event) = event_opt {
                                match self.kernel.app_event_handling(
                                    &self.state, 
                                    event, 
                                    &mut self.app_task_manager,
                                    ctx.clone(),
                                    &mut self.gui,
                                    &mut self.confirmation_context_manager,
                                    self.app_dirs.clone()
                                ) {
                                    Ok(new_state_opt) => {
                                        if let Some(new_state) = new_state_opt {
                                            self.state = new_state;
                                        };
                                    },
                                    Err(error) => {
                                        match app_error_handling(error.into()) {
                                            Some(new_state) => {
                                                self.state = new_state;
                                            },
                                            None => {},
                                        }
                                    },
                                }
                            };
                        },
                        Err(error) => {
                            match app_error_handling(error.into()) {
                                Some(new_state) => {
                                    self.state = new_state;
                                },
                                None => {},
                            }
                        },
                    } 
                } 
            },
            Err(error) => {
                match app_error_handling(error.into()) {
                    Some(new_state) => {
                        self.state = new_state;
                    },
                    None => {},
                } 
            }, 
        }; 

        self.app_task_manager.run(&mut self.external_app_events); 


        if !self.external_app_events.is_empty() {
            let events: Vec<AppEvent> = self.external_app_events.drain().collect();
        
            for event in events {
                match self.kernel.app_event_handling(
                    &self.state, 
                    event, 
                    &mut self.app_task_manager, 
                    ctx.clone(),
                    &mut self.gui,
                    &mut self.confirmation_context_manager,
                    self.app_dirs.clone(),
                ) {
                    Ok(Some(new_state)) => {
                        self.state = new_state;
                    },
                    Ok(None) => {},
                    Err(error) => {
                        match app_error_handling(error.into()) {
                            Some(new_state) => {
                                self.state = new_state;
                            },
                            None => {},
                        }
                    },
                }
            } 
        }; 

        match &self.state { 
            AppState::Ready => {},
            AppState::Shutdown => {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }

        self.time_detector.stop_and_display();
    } 
}
