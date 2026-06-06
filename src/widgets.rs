use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    widget::{checkbox, row, text},
};

use crate::material_list::material::Material;

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
        Item {
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
