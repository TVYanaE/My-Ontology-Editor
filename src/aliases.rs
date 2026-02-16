use egui::{
    containers::{
        CentralPanel
    },
    Context, Ui,
};
use winit::{
    event_loop::EventLoopProxy,
};
use calloop::{
    channel::Sender,
};
use egui_wgpu::{
    Renderer, RendererOptions,
    ScreenDescriptor
};
use egui_winit::{
    State, EventResponse,
};
use crate::{
    modules::{
        logic_module::LogicEvent,
        graphics_module::CustomEvent
    },
};

// Project aliases
pub type LogicEvents = Sender<LogicEvent>;
pub type CustomEvents = EventLoopProxy<CustomEvent>;

// egui aliases
pub type EGUIContext = Context;
pub type EGUIUI = Ui;
pub type EGUIRenderer = Renderer;
pub type EGUIWinitState = State;
pub type EGUIRendererOptions = RendererOptions;
pub type EGUIEventRespone = EventResponse;
pub type EGUIScreenDescriptor = ScreenDescriptor;

// egui containers 
pub type EGUICentralPanel = CentralPanel;

