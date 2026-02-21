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
                events::{UIEvent, UIEvents},
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
                UIEvent::OpenCreateNewProjectWindow => {
                    transition = Transition::Next(UIState::ModalWindow(ModalWindowKind::CreateNewProject));
                },
                UIEvent::DirPicked(dir) => {
                    modal_window.create_new_project_window.set_project_dir(dir);   
                    transition = Transition::Next(UIState::ModalWindow(ModalWindowKind::CreateNewProject));
                },
                UIEvent::FileDialogClosed => {
                    transition = Transition::Rollback; 
                },
                UIEvent::OpenFileDialogReq => {
                    modal_window.file_dialog.open_for_pick_directory();
                    transition = Transition::Next(UIState::ModalWindow(ModalWindowKind::FileDialog));
                },
                UIEvent::Confirmation { task_id, confirm } => {
                    ui_affects.push(UIAffect::Confirmation { task_id, confirm });
                    transition = Transition::Rollback;
                },
                UIEvent::NotificationClosed => {
                    transition = Transition::Rollback;
                },
                UIEvent::ShowNotification(text) => {
                    modal_window.notification.set_notification_text(&text);
                    transition = Transition::Next(UIState::ModalWindow(ModalWindowKind::Notification));
                },
                UIEvent::CreateProjectReq { project_name, project_dir } => {
                    ui_affects.push(UIAffect::CreateProjectReq { project_name, project_dir }); 
                    transition = Transition::Next(UIState::Default);
                },
                UIEvent::CloseCreateNewProjectWindow => {
                    transition = Transition::Next(UIState::Default);
                },
            }
        }

        Ok(transition) 
    } 
}
