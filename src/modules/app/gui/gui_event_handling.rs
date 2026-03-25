
use crate::modules::app::gui::gui_affect::{GUIAffectBuffer, GUIAffect};

use crate::modules::app::gui::gui_event::GUIEvent;

use crate::modules::app::gui::gui_state::{GUIStateTransform, GUIState};
use crate::modules::app::gui::gui_state::ModalWindowType;
use crate::modules::app::gui::gui_state::FileDialogResponseReceiver;

use crate::modules::app::gui::main_gui::MainGUI;

use crate::modules::app::gui::modal_window::ModalWindow;

use crate::modules::app::project::project_view::ProjectView;

pub fn gui_event_handling<E: Iterator<Item = GUIEvent>>(
    gui_events: E,
    gui_affect_buffer: &mut GUIAffectBuffer,
    modal_window: &mut ModalWindow,
    main_gui: &mut MainGUI,
    selected_project: Option<&ProjectView>, 
) -> GUIStateTransform {
    let mut transform = GUIStateTransform::Stay;
    
    for gui_event in gui_events {
        match gui_event {
            GUIEvent::ExitButtomPressed => {
                gui_affect_buffer.push(GUIAffect::ExitRequested);
            },
            GUIEvent::CreateProjectButtonPressed => {
                transform = GUIStateTransform::Next(
                    GUIState::ShowModalWindow(
                        ModalWindowType::CreateProjectWindow
                    )
                );
            },
            GUIEvent::CreateProjectCanceled => {
                transform = GUIStateTransform::Next(GUIState::Idle);
            }, 
            GUIEvent::OpenModalWindow(modal_window_type) => {
                transform = GUIStateTransform::Next(
                    GUIState::ShowModalWindow(modal_window_type)
                );
            },
            GUIEvent::PathSelected { 
                path,
                receiver,
            } => {
                if let Some(path_str) = path.to_str() {
                    match receiver {
                        FileDialogResponseReceiver::CreateProjectWindow => {
                            modal_window.with_create_project_window(|create_project_window|{
                                create_project_window.set_project_path(path_str);
                            });
                        },
                        FileDialogResponseReceiver::OpenProjectWindow => {
                            modal_window.with_open_project_window(|open_project_window|{
                                open_project_window.set_project_file_path(path_str);
                            });
                        },
                    };
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
                        project_name, 
                        project_path, 
                    }
                );    
            },
            GUIEvent::ConfirmationObtain { 
                confirmation_type, 
                decision 
            } => {
                gui_affect_buffer.push(
                    GUIAffect::ConfirmationObtain { 
                        confirmation_type, 
                        decision, 
                    }
                );
                transform = GUIStateTransform::Prev;
            },
            GUIEvent::OpenProjectButtonPressed => {
                transform = GUIStateTransform::Next(
                    GUIState::ShowModalWindow(
                        ModalWindowType::OpenProjectWindow
                    )
                );
            },
            GUIEvent::OpenProjectCanceled => {
                transform = GUIStateTransform::Next(
                    GUIState::Idle
                );
            },
            GUIEvent::OpenProjectRequest { 
                project_file_path 
            } => {
                gui_affect_buffer.push(
                    GUIAffect::OpenProjectInfo { 
                        project_file_path, 
                    }
                );   
            },
            GUIEvent::SwitchProjectRequest { 
                project_id, 
            } => {
                gui_affect_buffer.push(
                    GUIAffect::SwitchProjectRequest { 
                        project_id, 
                    }
                );
            }, 
            GUIEvent::SetLeftPanelVisibility { 
                visibility 
            } => {
                main_gui.with_left_panel(|left_panel|{
                    left_panel.set_visibility(visibility);
                });
            },
            GUIEvent::CreateSemanticNodeRequest => {
                if let Some(_project_view) = selected_project {
                    transform = GUIStateTransform::Next(
                        GUIState::ShowModalWindow(
                            ModalWindowType::CreateSemanticNodeWindow
                        )
                    );
                } 
            },
            GUIEvent::CreateSemanticNodeWindowClosed => {
                transform = GUIStateTransform::Next(
                    GUIState::Idle
                );
            },
        }
    }

    transform
}
