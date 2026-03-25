mod confirmation_window;
mod create_project_window;
mod create_semantic_node_window;
mod file_dialog;
mod loading_window;
mod notification;
mod open_project_window;

use eframe::egui::Context as EGUIContext;

use crate::modules::app::gui::gui_event::GUIEventBuffer;
use crate::modules::app::gui::gui_state::ModalWindowType;

use self::confirmation_window::ConfirmationWindow;
use self::create_project_window::CreateProjectWindow;
use self::create_semantic_node_window::CreateSemanticNodeWindow;
use self::file_dialog::FileDialog;
use self::loading_window::LoadingWindow;
use self::notification::Notification;
use self::open_project_window::OpenProjectWindow;


pub struct ModalWindow {
    confirmation_window: ConfirmationWindow,
    create_project_window: CreateProjectWindow,
    create_semantic_node_window: CreateSemanticNodeWindow,
    file_dialog: FileDialog,
    loading_window: LoadingWindow,
    notification: Notification,
    open_project_window: OpenProjectWindow,
}

impl ModalWindow {
    pub fn new() -> Self {
        Self {
            confirmation_window: ConfirmationWindow::new(),
            create_project_window: CreateProjectWindow::new(),
            create_semantic_node_window: CreateSemanticNodeWindow::new(),
            file_dialog: FileDialog::new(),
            loading_window: LoadingWindow::new(),
            notification: Notification::new(),
            open_project_window: OpenProjectWindow::new(),
        }
    } 
    pub fn prepare(
        &mut self,
        ctx: &EGUIContext,
        modal_window_type: &ModalWindowType,
        event_buffer: &mut GUIEventBuffer,
    ) {
        match modal_window_type {
            ModalWindowType::CreateProjectWindow => {
                self.create_project_window.prepare(ctx, event_buffer); 
            },
            ModalWindowType::OpenProjectWindow => {
                self.open_project_window.prepare(ctx, event_buffer);
            },
            ModalWindowType::FileDialog {
                item_type,
                receiver,
            } => {
                self.file_dialog.prepare(
                    ctx, 
                    item_type, 
                    event_buffer,
                    receiver,
                ); 
            },
            ModalWindowType::Notification(text) => {
                self.notification.prepare(ctx, event_buffer, text);
            },
            ModalWindowType::ConfirmationWindow { 
                confirmation_text, 
                confirmation_type 
            } => {
                self.confirmation_window.prepare(
                    ctx, 
                    event_buffer, 
                    confirmation_text, 
                    confirmation_type
                ); 
            },
            ModalWindowType::LoadingWindow => {
                self.loading_window.prepare(ctx);
            },
            ModalWindowType::CreateSemanticNodeWindow => {
                self.create_semantic_node_window.prepare(ctx, event_buffer);
            },
        }
    }

    pub fn with_create_project_window<F>(&mut self, f: F) 
    where 
        F: FnOnce(&mut CreateProjectWindow)
    {
        f(&mut self.create_project_window);
    }

    pub fn with_open_project_window<F>(&mut self, f: F) 
    where 
        F: FnOnce(&mut OpenProjectWindow)
    {
        f(&mut self.open_project_window);
    }
}
