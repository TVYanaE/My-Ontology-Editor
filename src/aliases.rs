use eframe::{
    Frame, Result, NativeOptions,
    CreationContext,
    egui::{
        Context
    },
};

pub type EFrame = Frame; 
pub type EFrameResult = Result;
pub type EFrameNativeOptions = NativeOptions;
pub type EFrameCreationContext<'s> = CreationContext<'s>; 
pub type EGUIContext = Context;

