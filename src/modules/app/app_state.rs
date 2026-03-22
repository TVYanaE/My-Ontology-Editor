#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub enum AppState { 
    #[default]
    NotInit,
    Initialisation,
    Ready,
    Shutdown,
}
