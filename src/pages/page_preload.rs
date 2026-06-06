// PagePreload

use iced::widget::{button, column, text};

use crate::{
    Message,
    pages::{Page, page_list_loaded::PageListLoaded},
};

#[derive(Debug, Clone)]
pub enum PagePreloadMessage {
    ButtonPressed,
}

pub struct PagePreload;

impl PagePreload {
    pub fn new() -> Self {
        Self
    }
}

impl Page for PagePreload {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        if let Message::PagePreload(msg) = message {
            match msg {
                PagePreloadMessage::ButtonPressed => return Some(Box::new(PageListLoaded::new())),
            }
        }
        None
    }

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            text("Hello!"),
            button("Load List").on_press(Message::PagePreload(PagePreloadMessage::ButtonPressed)),
        ]
        .into()
    }
}
