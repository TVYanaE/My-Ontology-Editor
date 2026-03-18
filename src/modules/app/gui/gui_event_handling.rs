
use super::gui_affect::{GUIAffectBuffer, GUIAffect};
use super::gui_event::GUIEvent;
use super::gui_state::{GUIStateTransform, GUIState};
use super::modal_window::{ModalWindow, ModalWindowType};

pub fn gui_event_handling<E: Iterator<Item = GUIEvent>>(
    gui_events: E,
    gui_affect_buffer: &mut GUIAffectBuffer,
    modal_window: &mut ModalWindow,
) -> GUIStateTransform {
    let mut transform = GUIStateTransform::Stay;
    
    for gui_event in gui_events {
        match gui_event {
            GUIEvent::ExitButtomPressed => {
                gui_affect_buffer.push(GUIAffect::ExitRequested);
            },
            GUIEvent::CreateProjectButtonPressed => {
                gui_affect_buffer.push(GUIAffect::CreateProjectRequested);
            },
            GUIEvent::CreateProjectCanceled => {
                transform = GUIStateTransform::Next(GUIState::Idle);
            }, 
            GUIEvent::OpenModalWindow(modal_window_type) => {
                transform = GUIStateTransform::Next(
                    GUIState::ShowModalWindow(modal_window_type)
                );
            },
            GUIEvent::PathSelected(path) => {
                if let Some(path_str) = path.to_str() {
                    modal_window.with_create_project_window(|create_project_window|{
                        create_project_window.set_project_path(path_str);
                    });
                    transform = GUIStateTransform::Prev;
                }
                else {
                    transform = GUIStateTransform::Next(
                        GUIState::ShowModalWindow(
                            ModalWindowType::Notification(
                                "Non UTF-8 Symbols isn't avaliable. Please Use only UTF-8 Symbols".into()
                            )
                        )
                    );
                } 
            },
            GUIEvent::FileDialogCanceled => {
                transform = GUIStateTransform::Prev;
            },
            GUIEvent::NotificationClosed => {
                transform = GUIStateTransform::Prev;
            },
            GUIEvent::CreateProjectRequest { 
                project_name, 
                project_path 
            } => {
                gui_affect_buffer.push(
                    GUIAffect::CreateProjectInfo { 
                        project_name: project_name, 
                        project_path: project_path, 
                    }
                );    
            },
            GUIEvent::ConfirmationObtain { 
                confirmation_type, 
                decision 
            } => {
                gui_affect_buffer.push(
                    GUIAffect::ConfirmationObtain { 
                        confirmation_type: confirmation_type, 
                        decision: decision 
                    }
                );
                transform = GUIStateTransform::Prev;
            },
        }
    }

    transform
}
