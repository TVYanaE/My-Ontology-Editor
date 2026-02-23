use egui::{
    containers::{
        CentralPanel
    },
    Context, Ui,
};
use egui_wgpu::{
    Renderer, RendererOptions,
    ScreenDescriptor
};
use egui_winit::{
    State, EventResponse,
};
use oneshot::{
    Sender, Receiver,
};

// EGUI aliases
pub type EGUIContext = Context;
pub type EGUIUI = Ui;
pub type EGUIRenderer = Renderer;
pub type EGUIWinitState = State;
pub type EGUIRendererOptions = RendererOptions;
pub type EGUIEventRespone = EventResponse;
pub type EGUIScreenDescriptor = ScreenDescriptor;

// EGUI containers aliases 
pub type EGUICentralPanel = CentralPanel;

// Oneshot aliases
pub type OneShotSender<T> = Sender<T>;
pub type OneShotReceiver<T> = Receiver<T>;
