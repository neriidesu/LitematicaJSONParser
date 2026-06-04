use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    widget::{checkbox, column, container, keyed_column, row, scrollable, text},
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
fn print_materials(list: &MaterialList) {
    let name = &list.Name;
    println!("Printing Materials for {name}");
    for material in &list.Materials {
        println!("{0}: {1}", material.Item, material.format_item_count())
    }
}

struct App {
    list: MaterialList,
    items: Vec<Item>,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

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
        iced::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        /*
        let witems = self.items;
        let items: Element<_> = if witems.iter().count() > 0 {
            keyed_column(items.iter().enumerate()).spacing(10).into()
        // FIXME: find out how tf you display all the items
        } else {
            text!("hi").into()
        };

        */

        let column = scrollable(
            column![
                text!("Name: {}", self.list.Name),
                text!("{}", self.list.generate_text()),
            ]
            .spacing(10),
        );

        container(column).center(Fill).into()
    }
}

struct Item {
    material: Material,
    completed: bool,
}

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

    fn view(&self, i: usize) -> Element<'_, ItemMessage> {
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
