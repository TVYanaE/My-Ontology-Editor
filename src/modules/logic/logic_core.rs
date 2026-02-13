mod logic_event_error_handle;
mod logic_event_handle;

use std::{
    sync::{Arc},
    thread::{self, JoinHandle},
};
use flume::{
    Receiver, RecvError,
    Sender,
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        graphics::events::CustomEvents,
        logic::{
            events::{
                LogicEvent
            },
        },
    },
};
use self::{
    logic_event_handle::{
        logic_event_handle, LogicEventContext
    },
    logic_event_error_handle::{
        logic_event_error_handle,
    },
};

pub struct LogicCore {
    pub handle: JoinHandle<()>,
    pub logic_event_channel_sender: Sender<LogicEvent>,
}


impl LogicCore {
    pub fn start(
        custom_events: CustomEvents,
        logic_event_channel_sender: Sender<LogicEvent>,
        logic_event_channel_receiver: Receiver<LogicEvent>,
        app_dirs: Arc<ApplicationDirectories>,
    ) -> Self {
        let handle = thread::spawn(move ||{ 
            let mut logic_core_state = LogicCoreState::Wait;         

            loop {    
                match logic_core_state {
                    LogicCoreState::Wait => {},
                    LogicCoreState::Shutdown => {
                        break;
                    },
                };
                match logic_event_channel_receiver.recv() {
                    Ok(event) => {
                        match logic_event_handle(
                            event, 
                            LogicEventContext { 
                                app_dirs: &app_dirs, 
                                custom_events: &custom_events 
                            }
                        ) {
                            Ok(Some(new_state)) => {
                                logic_core_state = new_state;
                            },
                            Ok(None) => {},
                            Err(error) => {
                                logic_event_error_handle(error);
                            },
                        } 
                    },
                    Err(error) => {
                        match error {
                            RecvError::Disconnected => {
                                // logic for disconnected
                                break; 
                            }
                        }
                    }
                };  
            }
        });

        Self { 
            handle: handle,
            logic_event_channel_sender: logic_event_channel_sender,
        }
    } 
}

#[derive(Debug)]
pub enum LogicCoreState {
    Wait, 
    Shutdown,
}
