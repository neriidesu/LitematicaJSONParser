use crate::Message;

pub mod page_list_loaded;
pub mod page_preload;

pub trait Page {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>>;
    fn view(&self) -> iced::Element<'_, Message>;
}
