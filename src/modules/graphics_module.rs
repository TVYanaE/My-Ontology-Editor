mod graphics_backend;
mod graphics_core;
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
    aliases::CustomEvents,
    modules::{
        app_dirs::ApplicationDirectories,
        shared::LogicModuleDescriptor,
    },
};
use self::{
    graphics_backend::GraphicsBackend,
    graphics_core::GraphicsCore,
    ui::UI,
};
pub use self::{
    graphics_core::{
        graphics_event::{
            CustomEvent,
            InternalEvent, ExternalEvent,
        }, 
    },
};

pub struct GraphicsModule {
    graphics_backend: GraphicsBackend,
    graphics_core: GraphicsCore,
    ui: UI,
    app_dirs: Arc<ApplicationDirectories>,
    last_instance: Instant,
}

impl GraphicsModule {
    pub fn new(
        app_dirs: Arc<ApplicationDirectories>,
        logic_module_descriptor: LogicModuleDescriptor,
        custom_events: CustomEvents,
    ) -> Self {
        Self { 
            graphics_backend: GraphicsBackend::default(), 
            graphics_core: GraphicsCore::new(logic_module_descriptor, custom_events), 
            ui: UI::default(),
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
            &mut self.graphics_backend, &mut self.ui
        ); 
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: CustomEvent) {
        self.graphics_core.on_event(event.into(), &mut self.graphics_backend, &mut self.ui);
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

        self.graphics_core.on_event(event.into(), &mut self.graphics_backend, &mut self.ui); 

    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.graphics_core.is_shutdown() {
            event_loop.exit();
        }    
    } 
}
