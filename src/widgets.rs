use iced::{
    Alignment::Center,
    Element,
    Length::{self, Fill},
    widget::{button, checkbox, column, container, row, text},
};

use crate::{
    Message,
    material_list::{MaterialList, material::Material},
    pages::page_preload::PagePreloadMessage,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    material: Material,
    completed: bool,
}

#[derive(Debug, Clone)]
pub enum ItemMessage {
    Completed(bool),
}

impl Item {
    pub fn new(material: Material) -> Self {
        Self {
            material: material,
            completed: false,
        }
    }

    pub fn update(&mut self, message: ItemMessage) {
        match message {
            ItemMessage::Completed(completed) => {
                self.completed = completed;
            }
        }
    }

    pub fn view(&self) -> Element<'_, ItemMessage> {
        let checkbox = checkbox(self.completed)
            .label(&self.material.Item)
            .on_toggle(ItemMessage::Completed)
            .width(Fill)
            .size(17)
            .text_shaping(text::Shaping::Advanced);

        let label = text(self.material.format_item_count());

        row![checkbox, label].spacing(20).align_y(Center).into()
    }
}

#[derive(Clone)]
pub struct ListPreview {
    pub material_list: MaterialList,
}

pub enum ListPreviewMessage {}

impl ListPreview {
    pub fn new(material_list: MaterialList) -> Self {
        Self {
            material_list: material_list,
        }
    }

    pub fn update(&mut self, message: ListPreviewMessage) {
        match message {}
    }

    pub fn view(&self) -> Element<'_, Message> {
        let label = text(&self.material_list.Name);

        let button = button("Load List").on_press(Message::PagePreload(
            PagePreloadMessage::LoadList(self.material_list.clone()),
        ));

        let content = row![label.width(Length::Fill), button]
            .spacing(10)
            .align_y(iced::Alignment::Center)
            .width(Length::Fill)
            .padding(10);

        container(content)
            .center(Length::Fill)
            .max_height(50)
            .height(50)
            .into()
    }
}
