use calloop::{
    channel::{
        Channel, Event, 
    },
    EventLoop,  
};
use super::{
    EventLoopResource,
    events::{EventSender, LogicCommand},
};


pub fn init_event_loop<'e, S>(
    channel: Channel<LogicCommand> 
) -> EventLoop<'e, EventLoopResource<S>>
where 
    S:EventSender + Send + 'static 
{
    let event_loop: EventLoop<EventLoopResource<S>> = EventLoop::try_new().expect("Event Loop Error init calloop. Logic Module");
    let event_loop_handle = event_loop.handle();
    
    let _ = event_loop_handle.insert_source(channel, |
        event,
        _meta,
        event_loop_resource
    |{
        match event {
            Event::Msg(command) => {
                event_loop_resource.job_manager.on_command(
                    command, 
                    &mut event_loop_resource.confirmation_cache
                ); 
            },
            Event::Closed => {
                event_loop_resource.logic_core.event_loop_closed_handle();
            }
        } 
    });

    event_loop
}
