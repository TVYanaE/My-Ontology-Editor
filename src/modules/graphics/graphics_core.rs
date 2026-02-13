mod handle_graphics_event;
mod handle_redraw;
mod redraw_error_handle;

use std::{
    sync::{
        Arc,
    },
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
        graphics::{
            events::{
                CustomEvents,
                graphics_event::{CustomEvent},
                EventBuffers,
            },
            graphics_data::GraphicsData, 
            graphics_states::GraphicsStates, 
        },
    },
};
use self::{
    handle_graphics_event::{
        GraphicsApplicationContext, handle_graphics_event
    },
    handle_redraw::{
        handle_redraw, HandleRedrawContext
    },
    redraw_error_handle::redraw_error_handle,
};


pub struct GraphicsCore {
    graphics_core_state: GraphicsCoreState,
    graphics_data: GraphicsData,
    graphics_states: GraphicsStates,
    event_buffers: EventBuffers,
    last_instance: Instant, 
}

impl GraphicsCore {
    pub fn new(
        custom_events: CustomEvents,
        app_dirs: Arc<ApplicationDirectories>,
    ) -> Self {
        Self { 
            graphics_core_state: GraphicsCoreState::default(), 
            graphics_data: GraphicsData::new(app_dirs), 
            graphics_states: GraphicsStates::default(), 
            event_buffers: EventBuffers::new(custom_events), 
            last_instance: Instant::now(),
        }
    }
}

impl ApplicationHandler<CustomEvent> for GraphicsCore {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .expect("Create Window Error");
        handle_graphics_event(GraphicsApplicationContext::from(self), CustomEvent::ResumedEvent(window).into()); 
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: CustomEvent) {
        handle_graphics_event(GraphicsApplicationContext::from(self), event.into());
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

        match event {
            WindowEvent::RedrawRequested => {
                if let Err(error) = handle_redraw(
                    HandleRedrawContext { 
                        event_buffers: &mut self.event_buffers, 
                        graphics_data: &mut self.graphics_data, 
                        graphics_states: &mut self.graphics_states 
                    }
                ) {
                    redraw_error_handle(error, &self.event_buffers.custom_events);
                }
            },
            _ => {
                handle_graphics_event(GraphicsApplicationContext::from(self), event.into());
            },
        }

    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        match &self.graphics_core_state {
            GraphicsCoreState::Shutdown => {
                event_loop.exit();
            },
            _ => { 
            }
        }
    } 
}

#[derive(Debug, Clone)]
pub enum GraphicsCoreState {
    Processing,
    Runnig,
    Shutdown,
}

impl Default for GraphicsCoreState {
    fn default() -> Self {
        Self::Runnig
    }
}

impl<'c> From<&'c mut GraphicsCore> for GraphicsApplicationContext<'c> {
    fn from(value: &'c mut GraphicsCore) -> Self {
        Self { 
            event_buffers: &mut value.event_buffers, 
            graphics_core_state: &mut value.graphics_core_state, 
            graphics_states: &mut value.graphics_states, 
            graphics_data: &mut value.graphics_data 
        }
    }
}
