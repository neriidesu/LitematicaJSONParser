use iced::{Element, Task};
use serde::{Deserialize, Serialize};

use crate::{
    material_list::MaterialList,
    pages::{
        Page,
        page_list_loaded::PageListLoadedMessage,
        page_preload::{
            PagePreload,
            PagePreloadMessage::{self},
        },
    },
    widgets::{Item, ItemMessage},
};
use std::{
    fmt::Error,
    fs::{self, File},
    io::Write,
    vec,
};

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
    ListLoaded(Result<MaterialList, Error>),
    FileCancelled,
    LoadNewList,
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

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message.clone() {
            Message::LoadNewList => return open_list(),
            Message::ListLoaded(res) => Task::done(Message::PagePreload(
                PagePreloadMessage::LoadList(res.expect("err couldn't load list")),
            )),
            _ => {
                let page = self.page.update(message);
                if let Some(p) = page {
                    self.page = p;
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        self.page.view()
    }
}

fn open_list() -> Task<Message> {
    Task::future(
        rfd::AsyncFileDialog::new()
            .add_filter("Litematica JSON List", &["json"])
            .pick_file(),
    )
    .then(|handle| match handle {
        Some(file_handle) => Task::perform(load_list(file_handle), Message::ListLoaded),

        None => Task::done(Message::FileCancelled),
    })
}

async fn load_list(handle: rfd::FileHandle) -> Result<MaterialList, Error> {
    let file_path = handle.path().to_str().expect("err");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let material_list: MaterialList = MaterialList::from_str(&contents);
    let mut material_list_items = vec![];

    for material in &material_list.Materials {
        material_list_items.push(Item::new(material.clone()));
    }

    let material_list: MaterialList = MaterialList::from_str(&contents);
    Ok(material_list)
}

fn load_data(file_path: &str) -> SaveData {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let data: SaveData = SaveData::from_str(&contents);

    data
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveData {
    material_list: MaterialList,
    items: Vec<Item>,
}

impl SaveData {
    pub fn from_str(str: &str) -> Self {
        serde_json::from_str(&str).expect("err")
    }
}

fn save_data(items: Vec<Item>, list: MaterialList) -> serde_json::Result<()> {
    let data = SaveData {
        material_list: list.clone(),
        items: items,
    };

    let json_string = serde_json::to_string_pretty(&data)?;

    let _ = write_save_file(list, json_string);
    Ok(())
}

fn write_save_file(list: MaterialList, json_string: String) -> std::io::Result<()> {
    let mut file = File::create(format!("{}{}.json", LIST_FOLDER, list.Name))?;

    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub mod pages;
