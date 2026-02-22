mod events;
mod main_ui;
mod modal_window;
mod ui_affect;
mod ui_error;
mod ui_logic;
mod ui_state;
mod ui_state_handle;


use self::{
    main_ui::MainUI,
    modal_window::ModalWindow,
    ui_affect::{
        UIAffects,
    },
    ui_state::{
        UIState, Transition,
    },
    ui_state_handle::{
        UIStateHandle,
        DefaultStateContext, 
        ModalWindowStateContext, 
    }
};
pub use self::{
    events::{UIInputEvent, ChosedModalWindow},
    ui_error::UIError,
    ui_affect::UIAffect,
};



pub struct UI {
    state: UIState,
    prev_state: Option<UIState>,
    main_ui: MainUI,
    modal_window: ModalWindow, 
    ui_affects: UIAffects,
}

impl UI {
    pub fn new() -> Self {
        Self { 
            state: UIState::default(),
            prev_state: None,
            main_ui: MainUI::default(), 
            modal_window: ModalWindow::default(),
            ui_affects: UIAffects::with_capacity(8),
        }
    }
 

    pub fn on_event(&mut self, event: UIInputEvent) -> Result<UIAffects, UIError> {
        let current_state = std::mem::replace(
            &mut self.state, 
            UIState::Processing,
        );

        self.state = match (current_state, event) {
            (UIState::Default, event) => {
                match UIStateHandle::default_state_handle(
                    event, 
                    DefaultStateContext { 
                        main_ui: &mut self.main_ui,
                        modal_window: &mut self.modal_window,
                        ui_affects: &mut self.ui_affects,
                    }
                )? {
                    Transition::Stay => {
                        UIState::Default
                    }, 
                    Transition::Next(new_state) => {
                        self.prev_state = Some(UIState::Default);
                        new_state
                    },
                    Transition::Rollback => {
                        if let Some(prev_state) = self.prev_state.take() {
                            self.prev_state = Some(UIState::Default);
                            prev_state
                        }
                        else {
                            UIState::Default
                        }
                    },
                }
            },
            (UIState::ModalWindow(kind), event) => {
                match UIStateHandle::modal_window_state_handle(
                    event, 
                    kind.clone(),
                    ModalWindowStateContext { 
                        main_ui: &mut self.main_ui, 
                        modal_window: &mut self.modal_window,
                        ui_affects: &mut self.ui_affects,
                    }
                )? {
                    Transition::Stay => {
                        UIState::ModalWindow(kind)
                    }, 
                    Transition::Next(new_state) => {
                        self.prev_state = Some(UIState::ModalWindow(kind));
                        new_state
                    },
                    Transition::Rollback => {
                        if let Some(prev_state) = self.prev_state.take() {
                            self.prev_state = Some(UIState::ModalWindow(kind));
                            prev_state
                        }
                        else {
                            UIState::Default
                        }
                    },
                } 
            },
            (current_state, _) => current_state,
        };
        
        let ui_affects: UIAffects = self.ui_affects.drain(..).collect();

        Ok(ui_affects)
    }
}
 

