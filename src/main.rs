use iced::{Element, Task};

use crate::{
    material_list::MaterialList,
    pages::{
        Page,
        page_list_loaded::PageListLoadedMessage,
        page_preload::{PagePreload, PagePreloadMessage},
    },
    widgets::{Item, ItemMessage},
};
use std::{fs, vec};

pub mod material_list;
pub mod widgets;

/* ---------- CONFIG ---------- */
/* ERROR MESSAGE CONSTANTS */
const ERR_NO_MATERIAL_LIST: &str = "No material list loaded!";

/* PATHS */
const DEMO_PATH: &str = "./testdata/materials.json";
const LIST_FOLDER: &str = "./testdata/lists/";

/* ---------- CONFIG ---------- */

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
    page: Box<dyn Page>,
}

#[derive(Debug, Clone)]
pub enum Message {
    PagePreload(PagePreloadMessage),
    PageListLoaded(PageListLoadedMessage),
    ItemMessage(usize, ItemMessage),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                page: Box::new(PagePreload::new()),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        let page = self.page.update(message);
        if let Some(p) = page {
            self.page = p;
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.page.view()
    }
}

fn load_list(file_path: &str) -> (MaterialList, Vec<Item>) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let material_list: MaterialList = MaterialList::from_str(&contents);
    let mut material_list_items = vec![];

    for material in &material_list.Materials {
        material_list_items.push(Item::new(material.clone()));
    }

    let material_list: MaterialList = MaterialList::from_str(&contents);
    (material_list, material_list_items)
}

pub mod pages;
