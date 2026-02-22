use crate::{
    modules::{
        graphics_module::{
            ui::{
                main_ui::MainUI,
                modal_window::ModalWindow,
                ui_error::UIError,
                ui_state::{UIState, ModalWindowKind, Transition},
                events::{UIInputEvent, ChosedModalWindow},
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
            UIInputEvent::ShowModalWindow(chosed_window) => {
                match chosed_window {
                    ChosedModalWindow::FileDialog => {
                        context.modal_window.file_dialog.open_for_pick_directory();
                        Ok(Transition::Next(UIState::ModalWindow(ModalWindowKind::FileDialog)))
                    },
                    ChosedModalWindow::ConfirmationWindow { 
                        confirmation_id, 
                        confirmation_kind, 
                        text 
                    } => {
                        context.modal_window.confirmation_window.set_confirmation(confirmation_id, &text, confirmation_kind);
                        Ok(Transition::Next(UIState::ModalWindow(ModalWindowKind::ConfirmationWindow)))
                    },
                    ChosedModalWindow::Notification { text } => {
                        context.modal_window.notification.set_notification_text(&text);
                        Ok(Transition::Next(UIState::ModalWindow(ModalWindowKind::Notification)))
                    },
                    ChosedModalWindow::WaitingWindow { text } => {
                        context.modal_window.waiting_window.set_text(&text);
                        Ok(Transition::Next(UIState::ModalWindow(ModalWindowKind::WaitingWindow)))
                    },
                    ChosedModalWindow::CreateNewProject { project_name, project_path } => {
                        if let Some(project_name) = project_name {
                                context.modal_window.create_new_project_window.set_project_name(&project_name);
                        };
                        if let Some(project_path) = project_path {
                            context.modal_window.create_new_project_window.set_project_path(&project_path);
                        };
                        Ok(Transition::Next(UIState::ModalWindow(ModalWindowKind::CreateNewProject)))
                    },
                }
            },
            UIInputEvent::ShowMainUI => {
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
        }
    }
}
