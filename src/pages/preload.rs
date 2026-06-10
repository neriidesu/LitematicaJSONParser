use std::{fs, io, path::PathBuf};

use iced::{
    Alignment, Element, Length, Theme,
    widget::{Column, button, column, container, pick_list, row, rule, scrollable, text},
};
use platform_dirs::AppDirs;

use crate::{APP_NAME, AUTHOR, Message, SaveData, VERSION, load_data, widgets::ListPreview};

#[derive(Default)]
pub struct PreloadData {
    pub list_previews: Option<Vec<ListPreview>>,
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

pub fn view(preload_data: &PreloadData, theme: Option<Theme>) -> Element<'_, Message> {
    let list_preview_column: Column<'_, Message> = match &preload_data.list_previews {
        None => column![
            text("No lists imported..."),
            text("Try importing one by pressing \"Import List\" to the left."),
        ]
        .align_x(Alignment::Center),
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
        container(column![
            text("Litematica JSON Parser").size(24),
            row![
                text(format!("v{}", VERSION))
                    .size(10)
                    .style(text::secondary)
                    .width(Length::Fill),
                text(format!("by {}", AUTHOR))
                    .size(10)
                    .style(text::secondary)
            ]
        ])
        .padding(10)
        .height(60),
        rule::horizontal(2),
        column![
            text("Hello!"),
            button("Import List")
                .on_press(Message::LoadNewList)
                .style(button::primary),
            rule::horizontal(2),
            text("Options").size(20),
            row![
                text("Theme").width(Length::Fill),
                pick_list(Theme::ALL, theme, Message::ThemeChanged)
                    .placeholder("Choose a theme...")
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("User Auto Hides").width(Length::Fill),
                button("Open File")
                    .on_press(Message::OpenHides)
                    .style(button::secondary)
            ]
            .align_y(Alignment::Center)
            .spacing(10)
        ]
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_x(Alignment::Center)
    ]
    .width(Length::FillPortion(1))
    .align_x(Alignment::Center);

    let right_column = column![
        row![
            text("Imported Lists").width(Length::Fill),
            button("Open Folder")
                .on_press(Message::OpenListsFolder)
                .style(button::secondary)
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .padding(10)
        .height(60),
        rule::horizontal(2),
        scrollable(container(list_preview_column).center(Length::Fill))
    ]
    .width(Length::FillPortion(1));
    let content = column![
        row![left_column, rule::vertical(2), right_column],
        text("Copyright © 2026 neriidesu")
            .size(10)
            .style(text::secondary)
    ];
    iced::Element::new(container(content).center(Length::Fill).padding(10))
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
