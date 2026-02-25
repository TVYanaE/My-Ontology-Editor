use calloop::{ 
    channel::{
        Channel, Event, 
    },
    EventLoop,
};
use crate::{
    modules::{
        db_module::{
            commands::DBCommand,
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
        resource
    |{
        match event {
            Event::Msg(db_command) => {
                resource.db_core.on_command(
                    db_command,
                    &mut resource.db_connect_cache,
                );
            },
            Event::Closed => {
                resource.loop_signal.stop();
            },
        } 
    });

    event_loop
} 
