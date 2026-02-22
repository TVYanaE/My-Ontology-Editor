use crate::{
    aliases::{
        EGUIContext,
    },
    modules::{
        graphics_module::{
            ui::{
                ui_affect::{UIAffects, UIAffect},
                main_ui::MainUI,
                modal_window::ModalWindow,
                ui_error::UIError,
                events::{UIEvent, UIEvents, ChosedModalWindow},
                ui_state::{UIState, ModalWindowKind, Transition},
            },
        },
    },
}; 

pub struct UILogic;

impl UILogic {
    pub fn prepare_main_ui(
        main_ui: &mut MainUI,
        egui_context: &EGUIContext
    ) -> Result<UIEvents, UIError> {
         
        let main_ui_events = main_ui.prepare(egui_context)?;

        Ok(main_ui_events)
    } 

    pub fn prepare_modal_window(
        modal_window_kind: ModalWindowKind,
        modal_window: &mut ModalWindow,
        egui_context: &EGUIContext,
    ) -> Result<UIEvents, UIError> {
        let modal_window_events = modal_window.prepare(egui_context, modal_window_kind)?;

        Ok(modal_window_events)
    }

    pub fn ui_events_handle(
        mut ui_events: UIEvents,
        ui_affects: &mut UIAffects,
        modal_window: &mut ModalWindow,
    ) -> Result<Transition, UIError> {
        let mut transition  = Transition::Stay;
        for event in ui_events.drain(..) {
            match event {
                UIEvent::QuitApp => {
                    ui_affects.push(UIAffect::ExitRequested);
                },
                UIEvent::ShowModalWindow(choosed_window) => {
                    match choosed_window {
                        ChosedModalWindow::FileDialog => {
                            modal_window.file_dialog.open_for_pick_directory();
                            transition = Transition::Next(UIState::ModalWindow(ModalWindowKind::FileDialog));
                        },
                        ChosedModalWindow::CreateNewProject { 
                            project_name, 
                            project_path 
                        } => {
                            if let Some(project_name) = project_name {
                                modal_window.create_new_project_window.set_project_name(&project_name);
                            }
                            if let Some(project_path) = project_path {
                                modal_window.create_new_project_window.set_project_path(&project_path);
                            }
                            transition = Transition::Next(UIState::ModalWindow(ModalWindowKind::CreateNewProject));
                        },
                        ChosedModalWindow::Notification { text } => {
                            modal_window.notification.set_notification_text(&text);
                            transition = Transition::Next(UIState::ModalWindow(ModalWindowKind::Notification));
                        },
                        ChosedModalWindow::WaitingWindow { text } => {
                            modal_window.waiting_window.set_text(&text);
                            transition = Transition::Next(UIState::ModalWindow(ModalWindowKind::WaitingWindow));
                        },
                        ChosedModalWindow::ConfirmationWindow { 
                            confirmation_id, 
                            confirmation_kind,
                            text,
                        } => {
                            modal_window.confirmation_window.set_confirmation(confirmation_id.clone(), &text, confirmation_kind.clone());
                            transition = Transition::Next(UIState::ModalWindow(ModalWindowKind::ConfirmationWindow));
                        },
                    }
                }, 
                UIEvent::ModalWindowClose => {
                    transition = Transition::Rollback;
                },
                UIEvent::ShowMainUI => {
                    transition = Transition::Next(UIState::Default);
                },
                UIEvent::PathPicked(path) => {
                    modal_window.create_new_project_window.set_project_path(&path);   
                    transition = Transition::Next(
                        UIState::ModalWindow(ModalWindowKind::CreateNewProject)
                    );
                },
                 
                UIEvent::ConfirmationDecision { 
                    confirmation_id, 
                    decision, 
                    decision_kind 
                } => {
                    ui_affects.push(UIAffect::ConfirmationDecision { 
                        confirmation_id, 
                        decision,
                        decision_kind,
                    });
                    transition = Transition::Rollback;
                }, 
                UIEvent::CreateProjectReq { project_name, project_path } => {
                    ui_affects.push(UIAffect::CreateProjectReq { project_name, project_path }); 
                    transition = Transition::Stay;
                }, 
            }
        }

        Ok(transition) 
    } 
}
