mod handle_graphics_event;
mod register_graphics_event;

use winit::{
    application::ApplicationHandler,
    event_loop::ActiveEventLoop,
    event::WindowEvent,
    window::{WindowId, WindowAttributes},
};
use crate::{
    modules::{
        graphics::{
            events::{
                graphics_event::{ITCEvent, CustomEvent},
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
    register_graphics_event::register_graphics_event,
};

#[derive(Default)]
pub struct GraphicsCore {
    graphics_core_state: GraphicsCoreState,
    graphics_data: GraphicsData,
    graphics_states: GraphicsStates,
    event_buffers: EventBuffers,
}

impl ApplicationHandler<ITCEvent> for GraphicsCore {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .expect("Create Window Error");

        register_graphics_event(
            &mut self.event_buffers.graphics_event_buffer, 
            CustomEvent::ResumedEvent(window)
        ); 
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: ITCEvent) {
        register_graphics_event(&mut self.event_buffers.graphics_event_buffer, event); 
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        register_graphics_event(&mut self.event_buffers.graphics_event_buffer, event); 
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        match &self.graphics_core_state {
            GraphicsCoreState::Shutdown => {
                event_loop.exit();
            },
            _ => {
                handle_graphics_event(GraphicsApplicationContext::from(self));
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
