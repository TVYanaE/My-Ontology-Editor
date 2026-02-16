use super::{
    modal_window::ModalWindowEvent,
    UIEvent, UIInputEvent, UIAffect,
    Panels, ModalWindow,
};

pub struct UILogic;

impl UILogic {
    pub fn ui_events_processing(
        mut events: Vec<UIEvent>, 
        _panels: &mut Panels,
        modal_window: &mut ModalWindow,
    ) -> Vec<UIAffect> {
        let mut ui_affects = Vec::with_capacity(16);

        while let Some(event) = events.pop() {
            match event {
                UIEvent::FileDialogClosed => {
                    modal_window.on_event(ModalWindowEvent::FileDialogClosed);
                },
                UIEvent::Error(error_text) => {
                    modal_window.on_event(ModalWindowEvent::ShowNotificationReq(error_text));
                },
                UIEvent::DirPicked(dir_path) => {
                    modal_window.create_new_project_window.set_project_dir(dir_path);
                    modal_window.on_event(ModalWindowEvent::FileDialogClosed);
                },
                UIEvent::QuitButtonPressed => {
                    ui_affects.push(UIAffect::ExitRequested);
                },
                UIEvent::OpenFileDialogReq => {
                    modal_window.on_event(ModalWindowEvent::FileDialogOpenReq);
                },
                UIEvent::NotificationClosed => {
                    modal_window.on_event(ModalWindowEvent::NotificationClosed);
                },
                UIEvent::CreateProjectReq(req) => {
                    modal_window.on_event(ModalWindowEvent::Reset);
                    ui_affects.push(UIAffect::CreateProjectReq(req));
                },
                UIEvent::CreateNewProjectButtonPressed => {
                    modal_window.on_event(ModalWindowEvent::CreateNewProjectReq);
                },
                UIEvent::CloseCreateNewProjectWindowButtonPressed => {
                    modal_window.on_event(ModalWindowEvent::Reset);
                },
            }
        }  
        ui_affects
    }
    pub fn ui_input_event_processing(
        event: UIInputEvent,
        modal_window: &mut ModalWindow,
    ) { 
        match event {
            UIInputEvent::Waiting => {
                modal_window.on_event(ModalWindowEvent::ShowWaitingWindowReq);
            },
            UIInputEvent::StopWaiting => {
                modal_window.on_event(ModalWindowEvent::Reset);
            }
        }
    }
}
