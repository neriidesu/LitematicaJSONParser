use crate::{
    material_list::MaterialList,
    widgets::{Item, ItemMessage, ListPreview},
};
use iced::{
    Element, Function, Length, Task,
    widget::{Column, button, center_x, column, container, row, rule, scrollable, text},
};
use platform_dirs::AppDirs;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Error,
    fs::{self, File},
    io::{self, Write, copy},
    path::{self, Path, PathBuf},
    process::Command,
    vec,
};

pub mod material_list;
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
struct PreloadData {
    list_previews: Option<Vec<ListPreview>>,
}

impl PreloadData {
    pub fn new() -> Self {
        let app_dirs = AppDirs::new(Some(APP_NAME), true).unwrap();
        let binding = app_dirs.data_dir.join("lists");
        let list_folder = binding.to_str().expect("err");

        let lists = parse_lists_in_folder(list_folder);

        let mut list_previews = vec![];
        let list_previews: Option<Vec<ListPreview>> = match &lists {
            None => None,
            Some(l) => {
                for list in l {
                    list_previews.push(ListPreview::new(list.clone()));
                }
                Some(list_previews)
            }
        };

        Self {
            list_previews: list_previews,
        }
    }
}
#[derive(Debug)]
struct ListData {
    pub list: Option<MaterialList>,
    pub items: Option<Vec<Item>>,
}

impl ListData {
    pub fn from_data(data: SaveData) -> Self {
        Self {
            list: Some(data.material_list),
            items: Some(data.items),
        }
    }

    pub fn from_list(material_list: MaterialList) -> Self {
        let mut material_list_items = vec![];

        for material in &material_list.Materials {
            material_list_items.push(Item::new(material.clone()));
        }

        Self {
            list: Some(material_list),
            items: Some(material_list_items),
        }
    }
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

            Message::ItemMessage(i, item_message) => {
                if let Some(ld) = &mut self.list_data {
                    if let Some(item) = ld.items.as_mut().expect(ERR_NO_MATERIAL_LIST).get_mut(i) {
                        return item.update(item_message);
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
            }

            Message::LoadSavedList(data) => {
                self.list_data = Some(ListData::from_data(data));
                self.page = Page::ListLoaded;
            }
            _ => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        match self.page {
            Page::Preload => {
                let list_preview_column: Column<'_, Message> =
                    match &self.preload_data.list_previews {
                        None => {
                            column![]
                        }
                        Some(lists) => {
                            let c = Column::new();
                            let it: Element<_> = lists
                                .iter()
                                .fold(Column::new().spacing(10), |col, l| col.push(l.view()))
                                .into();
                            let a: Element<_> = c.push(it).into();

                            let column = column![a];

                            column.spacing(10).max_width(800)
                        }
                    };

                let left_column = column![
                    text("Hello!"),
                    button("Load List").on_press(Message::LoadNewList),
                ]
                .width(Length::FillPortion(1));

                let right_column = column![
                    row![
                        text("Lists in listfolder").width(Length::Fill),
                        button("Open Folder").on_press(Message::OpenListsFolder)
                    ]
                    .width(Length::Fill)
                    .padding(10),
                    rule::horizontal(2),
                    scrollable(container(list_preview_column).center(Length::Fill))
                ]
                .width(Length::FillPortion(1));
                let content = row![left_column, rule::vertical(2), right_column];

                container(content).center(Length::Fill).padding(10).into()
            }
            Page::ListLoaded => {
                // check if material list exists
                if let Some(ld) = &self.list_data {
                    match &ld.items {
                        None => text!("{}", ERR_NO_MATERIAL_LIST).into(),
                        Some(items) => {
                            let c = Column::new();
                            let it: Element<_> = items
                                .iter()
                                .fold(Column::new().spacing(10), |col, i| {
                                    col.push(
                                        i.view().map(
                                            Message::ItemMessage
                                                .with(items.iter().position(|r| r == i).unwrap()),
                                        ),
                                    )
                                })
                                .into();
                            let a: Element<_> = c.push(it).into();

                            let column = column![a,].spacing(10).max_width(800);

                            let header = row![
                                text!(
                                    "Material List for: {}",
                                    ld.list.clone().expect(ERR_NO_MATERIAL_LIST).Name
                                ),
                                button("Exit").on_press(Message::ExitListButtonPressed)
                            ]
                            .spacing(20)
                            .padding(10);

                            let content = column![
                                container(header).center(Length::Fill).height(50.0),
                                scrollable(center_x(column).padding(40))
                            ]
                            .spacing(20);

                            container(content).center(Length::Fill).into()
                        }
                    }
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

fn load_data(file_path: &str) -> SaveData {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let data: SaveData = SaveData::from_str(&contents);

    data
}

pub fn download_file(url: &str, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let destination_dir = destination.parent().expect("err");
    fs::create_dir_all(destination_dir).unwrap();

    let response = blocking::get(url).map_err(|e| format!("Failed to send request: {}", e))?;
    let mut dest =
        File::create(destination).map_err(|e| format!("Failed to create file: {}", e))?;
    let content = response
        .bytes()
        .map_err(|e| format!("Failed to read response bytes: {}", e))?;
    copy(&mut content.as_ref(), &mut dest).map_err(|e| format!("Failed to copy content: {}", e))?;
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

fn parse_lists_in_folder(folder_path: &str) -> Option<Vec<SaveData>> {
    let lists = get_lists_in_folder(folder_path)
        .ok()
        .expect("ERR while getting lists");

    let mut l: Vec<SaveData> = vec![];

    for list in lists {
        let data: SaveData = load_data(&list.to_str().expect("err"));

        l.push(data);
    }

    if l.is_empty() { None } else { Some(l) }
}

fn get_lists_in_folder(folder_path: &str) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(folder_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}
