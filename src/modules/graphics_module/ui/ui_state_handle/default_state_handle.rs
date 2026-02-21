use super::{
    UIStateHandle,
};
use crate::{
    modules::{
        graphics_module::{
            ui::{
                ui_affect::UIAffects,
                events::UIInputEvent,
                ui_state::{UIState, ModalWindowKind, Transition},
                ui_error::UIError,
                main_ui::MainUI,
                modal_window::ModalWindow,
                ui_logic::UILogic,
            },
        },
    },
};

pub struct DefaultStateContext<'c> {
    pub main_ui: &'c mut MainUI,
    pub modal_window: &'c mut ModalWindow,
    pub ui_affects: &'c mut UIAffects,
}

impl UIStateHandle {
    pub fn default_state_handle( 
        event: UIInputEvent,
        context: DefaultStateContext,
    ) -> Result<Transition, UIError> {
        match event {
            UIInputEvent::Waiting => {
                Ok(Transition::Next(UIState::ModalWindow(ModalWindowKind::WaitingWindow)))
            },
            UIInputEvent::StopWaiting => {
                Ok(Transition::Next(UIState::Default))
            },
            UIInputEvent::PrepareUI(egui_context) => {
                let ui_events = UILogic::prepare_main_ui(
                    context.main_ui,
                    egui_context
                )?;

                let transition = UILogic::ui_events_handle(
                    ui_events, 
                    context.ui_affects, 
                    context.modal_window
                )?;

                Ok(transition)
            },
            UIInputEvent::ShowConfirmationWindow { task_id, text } => {
                context.modal_window.confirmation_window.set_task(task_id, &text);
                Ok(Transition::Next(UIState::ModalWindow(ModalWindowKind::ConfirmationWindow))) 
            },
        }
    } 
}
