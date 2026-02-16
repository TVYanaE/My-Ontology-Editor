mod event_loop;

use std::{
    sync::{Arc},
    thread::{self, JoinHandle},
};
use calloop::{
    LoopSignal,
    channel::{
        Channel, 
    },
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        graphics_module::CustomEvents,
        logic_module::{
            events::{
                LogicEvent
            },
        },
    },
};
use self::{
    event_loop::init_event_loop,
};

pub struct LogicCore {
    pub logic_core_state: LogicCoreState,
    pub custom_events: CustomEvents,
    pub app_dirs: Arc<ApplicationDirectories>,
    pub loop_signal: LoopSignal,
}

pub fn init_logic(
    custom_events: CustomEvents,
    app_dirs: Arc<ApplicationDirectories>,
    channel: Channel<LogicEvent>,
) -> JoinHandle<()> {
    let handle = thread::spawn(move||{
        let mut event_loop = init_event_loop(channel)
            .expect("Event Loop Error init calloop");
        let loop_signal = event_loop.get_signal();

        let mut logic_core = LogicCore {
            logic_core_state: LogicCoreState::Wait,
            custom_events: custom_events,
            app_dirs: app_dirs,
            loop_signal: loop_signal,
        };
        let _ = event_loop.run(None, &mut logic_core, |logic_core|{
            match logic_core.logic_core_state {
                LogicCoreState::Wait => {},
                LogicCoreState::Shutdown => {
                    logic_core.loop_signal.stop();
                }
            } 
        }); 
    });
    
    handle
}

#[derive(Debug)]
pub enum LogicCoreState {
    Wait, 
    Shutdown,
}
