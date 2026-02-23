use calloop::{ 
    channel::{
        Channel, Event, 
    },
    EventLoop,
};
use crate::{
    modules::{
        db_module::{
            events::DBCommand,
            EventLoopResource,
        },
    },
};


pub fn init_event_loop<'e>(
    channel: Channel<DBCommand>,
) -> EventLoop<'e, EventLoopResource> {
    let event_loop: EventLoop<EventLoopResource> = EventLoop::try_new()
        .expect("Event Loop Error init calloop. Logic Module");
    let event_loop_handle = event_loop.handle(); 

    let _ = event_loop_handle.insert_source(channel, |
        event,
        _meta,
        event_loop_resource
    |{
        match event {
            Event::Msg(db_command) => {
                event_loop_resource.db_core.on_command(
                    db_command
                );
            },
            Event::Closed => {
                event_loop_resource.loop_signal.stop();
            },
        } 
    });

    event_loop
} 
