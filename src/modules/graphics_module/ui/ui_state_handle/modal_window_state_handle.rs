use crate::{
    modules::{
        graphics_module::{
            ui::{
                main_ui::MainUI,
                modal_window::ModalWindow,
                ui_error::UIError,
                ui_state::{UIState, ModalWindowKind, Transition},
                events::UIInputEvent,
                ui_logic::UILogic,
                ui_affect::UIAffects,
            },
        },
    },
};
use super::{
    UIStateHandle,
};

pub struct ModalWindowStateContext<'c> {
    pub main_ui: &'c mut MainUI,
    pub modal_window: &'c mut ModalWindow,
    pub ui_affects: &'c mut UIAffects,
}

impl UIStateHandle {
    pub fn modal_window_state_handle(
        event: UIInputEvent,
        modal_window_kind: ModalWindowKind,
        context: ModalWindowStateContext,
    ) -> Result<Transition, UIError> {
        match event {
            UIInputEvent::Waiting => {
                Ok(Transition::Stay) 
            },
            UIInputEvent::StopWaiting => {
                Ok(Transition::Next(UIState::Default))
            },
            UIInputEvent::PrepareUI(egui_context) => {
                let mut main_ui_events = UILogic::prepare_main_ui(
                    context.main_ui, 
                    egui_context
                )?;
                
                let modal_window_events = UILogic::prepare_modal_window(
                    modal_window_kind, 
                    context.modal_window, 
                    egui_context
                )?;

                main_ui_events.extend(modal_window_events.into_iter());
                 
                let transition = UILogic::ui_events_handle(
                    main_ui_events, 
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
