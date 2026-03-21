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
use eframe::egui::{Context as EGUIContext, ViewportCommand};
use eframe::Frame;
use tokio::runtime::Runtime;

use self::app_dirs::AppDirs;
use self::app_error_hanlding::app_error_handling;
use self::app_event::AppEvent;
use self::app_kernel::{AppKernel, AppEventHandlingConxtex, GUIAffectsHandlingContext};
use self::app_state::AppState;
use self::app_task::app_task_manager::AppTaskManager;
use self::confirmation_context::confirmation_context_manager::ConfirmationContextManager;
use self::gui::GUI;
use self::time_detector::TimeDetector;
use self::project::project_cache::ProjectCache;


pub struct App {
    state: AppState, 
    gui: GUI,
    time_detector: TimeDetector,
    app_dirs: Arc<AppDirs>,
    app_task_manager: AppTaskManager,
    confirmation_context_manager: ConfirmationContextManager, 
    project_cache: ProjectCache,
}

impl App {
    pub fn new(
        _creation_context: &CreationContext,
        app_dirs: AppDirs,
        runtime: Runtime,
    ) -> Self {
        
        App {
            state: AppState::default(),
            gui: GUI::new(),
            time_detector: TimeDetector::new(),
            app_dirs: Arc::new(app_dirs),
            app_task_manager: AppTaskManager::new(runtime),
            confirmation_context_manager: ConfirmationContextManager::new(),
            project_cache: ProjectCache::new(),
        }
    }

    fn app_event_ctx<'a>(&'a mut self, egui_ctx: EGUIContext) -> AppEventHandlingConxtex<'a> {
        AppEventHandlingConxtex { 
            current_state: &self.state, 
            app_task_manager: &mut self.app_task_manager, 
            egui_context: egui_ctx, 
            gui: &mut self.gui, 
            confirmation_context_manager: &mut self.confirmation_context_manager, 
            app_dirs: self.app_dirs.clone(), 
            project_cache: &mut self.project_cache 
        }
    }

    fn gui_affects_ctx<'a>(&'a mut self) -> GUIAffectsHandlingContext<'a> {
        GUIAffectsHandlingContext { 
            gui: &mut self.gui, 
            current_app_state: &self.state, 
            confirmation_context_manager: &mut self.confirmation_context_manager 
        } 
    }
} 

impl EFrameApp for App {
    fn update(&mut self, ctx: &EGUIContext, _frame: &mut Frame) {
        self.time_detector.start_measurement();

        match self.gui.prepare_gui(ctx) {
            Ok(gui_affect_buffer) => {
                for gui_affect in gui_affect_buffer {
                    match AppKernel::gui_affects_handling(gui_affect, self.gui_affects_ctx()) {
                        Ok(event_opt) => {
                            if let Some(event) = event_opt {
                                match AppKernel::app_event_handling(event, self.app_event_ctx(ctx.clone())) {
                                    Ok(new_state_opt) => {
                                        if let Some(new_state) = new_state_opt {
                                            self.state = new_state;
                                        };
                                    },
                                    Err(error) => {
                                        if let Some(new_state) = app_error_handling(error.into()) {
                                            self.state = new_state;
                                        }
                                    },
                                }
                            };
                        },
                        Err(error) => {
                            if let Some(new_state) = app_error_handling(error.into()) {
                                self.state = new_state;
                            }
                        },
                    } 
                } 
            },
            Err(error) => {
                if let Some(new_state) = app_error_handling(error.into()) {
                    self.state = new_state;
                } 
            }, 
        }; 

        self.app_task_manager.check_tasks(); 

        let app_events = self
            .app_task_manager
            .check_events()
            .collect::<Vec<AppEvent>>(); 


        if !app_events.is_empty() { 
            for event in app_events {
                match AppKernel::app_event_handling(event, self.app_event_ctx(ctx.clone())) {
                    Ok(Some(new_state)) => {
                        self.state = new_state;
                    },
                    Ok(None) => {},
                    Err(error) => {
                        if let Some(new_state) = app_error_handling(error.into()) {
                            self.state = new_state;
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
