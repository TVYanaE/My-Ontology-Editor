use anyhow::Context;
use calloop::{
    channel::{
        Channel, Event, 
    },
    EventLoop,  
};
use crate::{
    modules::{
        logic_module::{
            events::LogicEvent,
        }, 
    },
};
use super::{
    EventLoopResource
};


pub fn init_event_loop<'e>(
    channel: Channel<LogicEvent> 
) -> Result<EventLoop<'e, EventLoopResource>, anyhow::Error>{
    let event_loop: EventLoop<EventLoopResource> = EventLoop::try_new().context("Event Loop Error init calloop")?;
    let event_loop_handle = event_loop.handle();
    
    let _ = event_loop_handle.insert_source(channel, |
        event,
        _meta,
        event_loop_resource
    |{
        match event {
            Event::Msg(logic_event) => {
                event_loop_resource.logic_core.on_event(
                    logic_event, 
                    &event_loop_resource.custom_events, 
                    &event_loop_resource.logic_events,
                    &event_loop_resource.app_dirs,
                    event_loop_resource.project_manager.clone()
                ); 
            },
            Event::Closed => {
                event_loop_resource.logic_core.event_loop_closed_handle();
            }
        } 
    });

    Ok(event_loop)
}
