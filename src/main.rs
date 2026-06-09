use crate::{
    material_list::MaterialList,
    pages::{
        list_loaded::{self, Filter, ListData},
        preload::{self, PreloadData},
    },
    widgets::{Item, ItemMessage},
};
use iced::{Element, Task, futures::StreamExt, widget::text};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Error,
    fs::{self, File},
    io::{Cursor, Write, copy},
    path::{self, Path},
    process::Command,
    vec,
};
use tokio::io::AsyncWriteExt;

pub mod material_list;
pub mod pages;
pub mod widgets;

/* ---------- CONFIG ---------- */
const APP_NAME: &str = "LitematicaJSONParser";
/* ERROR MESSAGE CONSTANTS */
const ERR_NO_MATERIAL_LIST: &str = "No material list loaded!";

/* ---------- CONFIG ---------- */

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Litematica JSON Parser")
        .run()
}

#[derive(Default)]
enum Page {
    #[default]
    Preload,
    ListLoaded,
}

struct App {
    page: Page,
    list_data: Option<ListData>,
    preload_data: PreloadData,
}

#[derive(Debug, Clone)]
pub enum Message {
    ExitListButtonPressed,
    ItemMessage(usize, ItemMessage),
    TupledItemMessage((usize, ItemMessage)),
    ListLoaded(Result<MaterialList, Error>),
    FileCancelled,
    LoadNewList,
    PageLoaded,
    OpenListsFolder,
    LoadList(MaterialList),
    LoadSavedList(SaveData),
    FilterChanged(Filter),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let app_dirs = AppDirs::new(Some(APP_NAME), true).unwrap();
        fs::create_dir_all(&app_dirs.data_dir.join("lists")).unwrap();

        (
            Self {
                page: Page::Preload,
                list_data: None,
                preload_data: PreloadData::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ExitListButtonPressed => {
                if let Some(ld) = &self.list_data {
                    let _ = save_data(
                        ld.items.clone().expect(ERR_NO_MATERIAL_LIST),
                        ld.list.clone().expect(ERR_NO_MATERIAL_LIST),
                    );

                    self.list_data = None;
                    self.preload_data = PreloadData::new();
                    self.page = Page::Preload;
                }
            }

            Message::ItemMessage(i, ItemMessage::Delete) => {
                self.list_data
                    .as_mut()
                    .unwrap()
                    .items
                    .as_mut()
                    .unwrap()
                    .remove(i);
            }

            Message::ItemMessage(i, item_message) => {
                if let Some(ld) = &mut self.list_data {
                    if let Some(item) = ld.items.as_mut().expect(ERR_NO_MATERIAL_LIST).get_mut(i) {
                        return item.update(item_message, i);
                    }
                }
            }

            Message::LoadNewList => return open_list(),
            Message::ListLoaded(res) => {
                return Task::done(Message::LoadList(res.expect("err couldn't load list")));
            }

            Message::TupledItemMessage((i, m)) => return Task::done(Message::ItemMessage(i, m)),

            Message::OpenListsFolder => {
                let app_dirs = AppDirs::new(Some(APP_NAME), true).unwrap();
                let binding = app_dirs.data_dir.join("lists");
                let list_folder = binding.to_str().expect("err");
                Command::new("xdg-open")
                    .arg(path::absolute(list_folder).expect("Could not get absolute path"))
                    .spawn()
                    .unwrap();
            }

            Message::LoadList(list) => {
                self.list_data = Some(ListData::from_list(list));
                self.page = Page::ListLoaded;

                let mut tasks = vec![];
                let items = self.list_data.as_ref().unwrap().items.as_ref().unwrap();
                for item in items {
                    tasks.push(Task::done(Message::ItemMessage(
                        items.iter().position(|r| r == item).unwrap(),
                        ItemMessage::Load,
                    )))
                }

                return Task::batch(tasks);
            }

            Message::LoadSavedList(data) => {
                self.list_data = Some(ListData::from_data(data));
                self.page = Page::ListLoaded;

                let mut tasks = vec![];
                let items = self.list_data.as_ref().unwrap().items.as_ref().unwrap();
                for item in items {
                    tasks.push(Task::done(Message::ItemMessage(
                        items.iter().position(|r| r == item).unwrap(),
                        ItemMessage::Load,
                    )))
                }

                return Task::batch(tasks);
            }

            Message::FilterChanged(filter) => {
                if let Some(ld) = &mut self.list_data {
                    ld.filter = filter;
                }
            }
            _ => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        match self.page {
            Page::Preload => preload::view(&self.preload_data),
            Page::ListLoaded => {
                // check if material list exists
                if let Some(ld) = &self.list_data {
                    list_loaded::view(ld)
                } else {
                    text!("{}", ERR_NO_MATERIAL_LIST).into()
                }
            }
        }
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

pub async fn download_file(url: &str, destination: &Path) -> anyhow::Result<()> {
    let destination_dir = destination.parent().expect("err");
    fs::create_dir_all(destination_dir).unwrap();

    let mut file = tokio::fs::File::create(destination).await.expect("err");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "curl/8.20.0")
        .send()
        .await?
        .error_for_status()?;

    let mut byte_stream = response.bytes_stream();

    while let Some(chunk) = byte_stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }
    Ok(())
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
    let app_dirs = AppDirs::new(Some(APP_NAME), true).unwrap();
    let binding = app_dirs.data_dir.join("lists");
    let list_folder = binding.to_str().expect("err");
    let mut file = File::create(format!("{}/{}.json", list_folder, list.Name))?;

    file.write_all(json_string.as_bytes())?;

    Ok(())
}

fn load_data(file_path: &str) -> SaveData {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let data: SaveData = SaveData::from_str(&contents);

    data
}
