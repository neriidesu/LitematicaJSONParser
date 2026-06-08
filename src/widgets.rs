use std::path::{self, Path};

use iced::{
    Alignment::Center,
    Element,
    Length::{self, Fill},
    Task,
    widget::{Image, button, checkbox, column, container, image, row, text},
};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};

use crate::{
    APP_NAME, App, Message, SaveData, download_file,
    material_list::{MaterialList, material::Material},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    icon_path: Option<String>,
    pub material: Material,
    completed: bool,
}

#[derive(Debug, Clone)]
pub enum ItemMessage {
    Completed(bool),
    FetchIcon(String),
    IconLoaded(String),
}

impl Item {
    pub fn new(material: Material) -> Self {
        Self {
            icon_path: None,
            material: material,
            completed: false,
        }
    }

    pub fn update(&mut self, message: ItemMessage) -> Task<Message> {
        match message {
            ItemMessage::Completed(completed) => {
                self.completed = completed;
            }

            ItemMessage::FetchIcon(item) => {}

            ItemMessage::IconLoaded(path) => self.icon_path = Some(path),
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, ItemMessage> {
        let icon = image(get_image_path(self.material.Item.clone()))
            .width(32)
            .height(32);

        let checkbox = checkbox(self.completed)
            .label(&self.material.Item)
            .on_toggle(ItemMessage::Completed)
            .width(Fill)
            .size(17)
            .text_shaping(text::Shaping::Advanced);

        let label = text(self.material.format_item_count());

        row![icon, checkbox, label]
            .spacing(20)
            .align_y(Center)
            .into()
    }
}

pub fn get_image_path(item_name: String) -> String {
    let path = AppDirs::new(Some(APP_NAME), true)
        .unwrap()
        .data_dir
        .join("icons")
        .join(&item_name)
        .with_extension("png");

    if path.exists() {
        path.to_str().expect("err").to_string()
    } else {
        let url = format!(
            "https://www.mcworldtools.com/textures/rendered/{}.png",
            &item_name[10..]
        );
        match download_file(&url, &path) {
            Ok(_) => println!("imag saved at {}", path.to_str().expect("err")),
            Err(e) => println!("error while downloading image: {}", e),
        }

        path.to_str().expect("err").to_string()
    }
}

#[derive(Clone)]
pub struct ListPreview {
    pub data: SaveData,
}

pub enum ListPreviewMessage {}

impl ListPreview {
    pub fn new(data: SaveData) -> Self {
        Self { data: data }
    }

    pub fn update(&mut self, message: ListPreviewMessage) {
        match message {}
    }

    pub fn view(&self) -> Element<'_, Message> {
        let label = text(&self.data.material_list.Name);

        let button = button("Load List").on_press(Message::LoadSavedList(self.data.clone()));

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
