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

