mod graphics_backend;
mod graphics_core;
mod events;
pub mod logic_adapter;
mod task_cache;
mod ui;

use std::{
    sync::Arc,
    time::Instant,
};
use winit::{
    application::ApplicationHandler,
    event_loop::ActiveEventLoop,
    event::{WindowEvent},
    window::{WindowId, WindowAttributes},
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        logic_module::{
            prelude::LogicModuleHandler,
        },
    },
};
use self::{
    graphics_backend::GraphicsBackend,
    graphics_core::GraphicsCore,
    task_cache::TaskCache,
    ui::UI,
    events::InternalEvent,
};
pub use self::{
    events::{
        CustomEvent, CustomEvents,
        ExternalEvent,
    }, 
};

pub struct GraphicsModule {
    graphics_backend: GraphicsBackend,
    graphics_core: GraphicsCore,
    ui: UI,
    task_cache: TaskCache,
    app_dirs: Arc<ApplicationDirectories>,
    last_instance: Instant,
}

impl GraphicsModule {
    pub fn new(
        app_dirs: Arc<ApplicationDirectories>,
        logic_module_handler: LogicModuleHandler,
        custom_events: CustomEvents,
    ) -> Self {
        Self { 
            graphics_backend: GraphicsBackend::default(), 
            graphics_core: GraphicsCore::new(logic_module_handler, custom_events), 
            ui: UI::new(),
            task_cache: TaskCache::new(),
            app_dirs: app_dirs,
            last_instance: Instant::now(),
        }
    } 
}

impl ApplicationHandler<CustomEvent> for GraphicsModule {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .expect("Create Window Error");
        self.graphics_core.on_event(CustomEvent::InternalEvent(
            InternalEvent::ResumedEvent(window)).into(), 
            &mut self.graphics_backend, 
            &mut self.ui,
            &mut self.task_cache,
        ); 
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: CustomEvent) {
        self.graphics_core.on_event(
            event.into(), 
            &mut self.graphics_backend, 
            &mut self.ui,
            &mut self.task_cache,
        );
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let now = Instant::now();
        let delt = now - self.last_instance;
        println!("Time: {:?}", delt);
        self.last_instance = now;

        self.graphics_core.on_event(
            event.into(), 
            &mut self.graphics_backend, 
            &mut self.ui,
            &mut self.task_cache,
        ); 

    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.graphics_core.is_shutdown() {
            event_loop.exit();
        }    
    } 
}
