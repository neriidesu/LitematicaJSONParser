use iced::{
    Alignment::Center,
    Element,
    Length::{self, Fill},
    Task,
    widget::{button, checkbox, container, image, image::Handle, row, text},
};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{APP_NAME, Message, SaveData, download_file, material_list::material::Material};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    #[serde(skip)]
    icon_handle: Option<Handle>,
    pub material: Material,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub enum ItemMessage {
    Completed(bool),
    Load,
    Loaded(Vec<u8>),
    Delete,
}

impl Item {
    pub fn new(material: Material) -> Self {
        Self {
            icon_handle: None,
            material: material,
            completed: false,
        }
    }

    pub fn update(&mut self, message: ItemMessage, index: usize) -> Task<Message> {
        match message {
            ItemMessage::Completed(completed) => {
                self.completed = completed;
            }
            ItemMessage::Load => {
                return Task::perform(
                    get_indexed_image_message(self.material.Item.clone(), index),
                    Message::TupledItemMessage,
                );
            }
            ItemMessage::Loaded(data) => self.icon_handle = Some(Handle::from_bytes(data)),
            ItemMessage::Delete => {}
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, ItemMessage> {
        let mut icon = image("./assets/loading.png").width(32).height(32);

        match &self.icon_handle {
            Some(h) => icon = image(h.clone()).width(32).height(32),
            None => {}
        }

        let checkbox = checkbox(self.completed)
            .label(&self.material.Item)
            .on_toggle(ItemMessage::Completed)
            .width(Fill)
            .size(17)
            .text_shaping(text::Shaping::Advanced);

        let label = text(self.material.format_item_count());

        let delete_button = button("Delete")
            .on_press(ItemMessage::Delete)
            .style(button::danger);

        row![icon, checkbox, label, delete_button]
            .spacing(20)
            .align_y(Center)
            .into()
    }
}

async fn get_indexed_image_message(item_name: String, index: usize) -> (usize, ItemMessage) {
    let buffer = get_image(item_name).await;
    (index, ItemMessage::Loaded(buffer))
}

pub async fn get_image(item_name: String) -> Vec<u8> {
    let path = AppDirs::new(Some(APP_NAME), true)
        .unwrap()
        .data_dir
        .join("icons")
        .join(&item_name)
        .with_extension("png");

    if path.exists() {
        let mut file = File::open(path).await.unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await.unwrap();
        buffer
    } else {
        let name: Vec<&str> = item_name.split_terminator(":").collect();
        let split: Vec<&str> = name[1].split_inclusive("_").collect();
        let mut name: Vec<String> = vec![];
        for s in split {
            let mut v: Vec<char> = s.chars().collect();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            let s_upper: String = v.into_iter().collect();
            name.push(s_upper);
        }
        let name = name.join("");

        let url = format!("https://minecraft.wiki/images/Invicon_{}.png", name);
        match download_file(&url, &path).await {
            Ok(_) => println!("file downloaded to: {}", &path.to_string_lossy()),
            Err(e) => println!("error while downloading image: {}", e),
        }
        let mut file = File::open(path).await.unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await.unwrap();
        buffer
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
