mod aliases;
mod application;

use eframe::{
    run_native,
};
use aliases::{
    EFrameResult, EFrameNativeOptions
};
use application::{
    Application,
};

fn main() -> EFrameResult {
    let native_options = EFrameNativeOptions::default();

    run_native("My-Ontology-Editor", native_options, Box::new(|creation_context|{
        Ok(Box::new(Application::new(creation_context)))
    }))
}
