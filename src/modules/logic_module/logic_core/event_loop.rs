mod handle_logic_event;
mod handle_logic_event_error;

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
            logic_core::{
                LogicCore, LogicCoreState,
            },     
        },
    },
};
use self::{
    handle_logic_event::{
        handle_logic_event, LogicEventContext 
    }, 
    handle_logic_event_error::{
        handle_logic_event_error,
    }, 
};


pub fn init_event_loop<'e>(
    channel: Channel<LogicEvent> 
) -> Result<EventLoop<'e, LogicCore>, anyhow::Error>{
    let event_loop: EventLoop<LogicCore> = EventLoop::try_new().context("Event Loop Error init calloop")?;
    let event_loop_handle = event_loop.handle();
    
    let _ = event_loop_handle.insert_source(channel, |
        event,
        _meta,
        logic_core
    |{
        match event {
            Event::Msg(logic_event) => {
                match handle_logic_event(
                    logic_event, 
                    LogicEventContext { 
                        app_dirs: &mut logic_core.app_dirs, 
                        custom_events: &logic_core.custom_events, 
                    }
                ) {
                    Ok(Some(new_state)) => {
                        logic_core.logic_core_state = new_state;  
                    },
                    Ok(None) => {},
                    Err(error) => {
                        if let Some(new_state) = handle_logic_event_error(error) {
                            logic_core.logic_core_state = new_state;
                        }
                    }
                }
            },
            Event::Closed => {
                logic_core.logic_core_state = LogicCoreState::Shutdown; 
            }
        } 
    });

    Ok(event_loop)
}
