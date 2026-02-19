
use calloop::{ 
    channel::{
        Channel, Event, 
    },
    EventLoop,
};
use super::{
    EventLoopResource,
    DBEvent,
};


pub fn init_event_loop<'e>(
    channel: Channel<DBEvent>,
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
            Event::Msg(db_event) => {
                event_loop_resource.db_core.on_event(
                    db_event,
                    &event_loop_resource.db_events
                );
            },
            Event::Closed => {
                event_loop_resource.loop_signal.stop();
            },
        } 
    });

    event_loop
} 
