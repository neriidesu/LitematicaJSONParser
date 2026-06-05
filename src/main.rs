use iced::{
    Alignment::Center,
    Element, Function,
    Length::Fill,
    widget::{Column, Scrollable, checkbox, column, container, row, scrollable, text},
};

use crate::material_list::{MaterialList, material::Material};
use std::{fs, vec};

pub mod material_list;

fn main() -> iced::Result {
    // let file_path = "./testdata/materials.json";
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // let material_list: MaterialList = MaterialList::from_str(&contents);

    // println!("{}", material_list.Name);

    // let third_mat: &Material = &material_list.Materials[2];

    // println!("{}", third_mat.format_item_count());

    // print_materials(&material_list);

    // text(15);
    // let interface = column![name];

    iced::application(App::new, App::update, App::view)
        .title("Litematica JSON Parser")
        .run()
}

/*
fn print_materials(list: &MaterialList) {
    let name = &list.Name;
    println!("Printing Materials for {name}");
    for material in &list.Materials {
        println!("{0}: {1}", material.Item, material.format_item_count())
    }
}
    */

struct App {
    list: MaterialList,
    items: Vec<Item>,
}

#[derive(Debug, Clone)]
enum Message {
    ItemMessage(usize, ItemMessage),
}

impl App {
    fn new() -> Self {
        let file_path = "./testdata/materials.json";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let material_list: MaterialList = MaterialList::from_str(&contents);
        let mut material_list_items = vec![];

        for material in material_list.Materials {
            material_list_items.push(Item::new(material));
        }

        let material_list: MaterialList = MaterialList::from_str(&contents);

        Self {
            list: material_list,
            items: material_list_items,
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::ItemMessage(i, item_message) => {
                if let Some(item) = self.items.get_mut(i) {
                    item.update(item_message);
                }
            }
        };

        iced::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let c = Column::new();
        let it: Element<_> = self
            .items
            .iter()
            .fold(Column::new().spacing(10), |col, i| {
                col.push(i.view().map(
                    Message::ItemMessage.with(self.items.iter().position(|r| r == i).unwrap()),
                ))
            })
            .into();
        let a: Element<_> = c.push(it).into();

        let column: Scrollable<'_, Message> = scrollable(
            column![
                text!("Name: {}", self.list.Name),
                // text!("{}", self.list.generate_text()),
                a,
            ]
            .spacing(10),
        );

        container(column).center(Fill).into()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Item {
    material: Material,
    completed: bool,
}

#[derive(Debug, Clone)]
pub enum ItemMessage {
    Completed(bool),
}

impl Item {
    fn new(material: Material) -> Self {
        Item {
            material: material,
            completed: false,
        }
    }

    fn update(&mut self, message: ItemMessage) {
        match message {
            ItemMessage::Completed(completed) => {
                self.completed = completed;
            }
        }
    }

    fn view(&self) -> Element<'_, ItemMessage> {
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
