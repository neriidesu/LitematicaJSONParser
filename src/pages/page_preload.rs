// PagePreload

use std::{
    fs::{self, read_dir},
    io,
    path::PathBuf,
};

use iced::{
    Element, Length,
    widget::{Column, button, column, container, row, rule, scrollable, text},
};

use crate::{
    LIST_FOLDER, Message,
    material_list::MaterialList,
    pages::{Page, page_list_loaded::PageListLoaded},
    widgets::ListPreview,
};

#[derive(Debug, Clone)]
pub enum PagePreloadMessage {
    ButtonPressed,
    LoadList(MaterialList),
}

pub struct PagePreload {
    lists: Option<Vec<MaterialList>>,
    list_previews: Option<Vec<ListPreview>>,
}

impl PagePreload {
    pub fn new() -> Self {
        let lists = parse_lists_in_folder(LIST_FOLDER);

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
            lists: lists,
            list_previews: list_previews,
        }
    }
}

impl Page for PagePreload {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        if let Message::PagePreload(msg) = message {
            match msg {
                PagePreloadMessage::ButtonPressed => return Some(Box::new(PageListLoaded::new())),

                PagePreloadMessage::LoadList(list) => {
                    return Some(Box::new(PageListLoaded::from_list(list)));
                }
            }
        }
        None
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let list_preview_column: Column<'_, Message> = match &self.list_previews {
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

        let left_column =
            column![text("Hello!"), button("Load List"),].width(Length::FillPortion(1));

        let right_column = column![
            row![
                text("Lists in listfolder").width(Length::Fill),
                button("Open Folder")
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
}

fn parse_lists_in_folder(folder_path: &str) -> Option<Vec<MaterialList>> {
    let lists = get_lists_in_folder(folder_path)
        .ok()
        .expect("ERR while getting lists");

    let mut l: Vec<MaterialList> = vec![];

    for list in lists {
        let contents = fs::read_to_string(list).expect("Should have been able to read the file");

        let material_list: MaterialList = MaterialList::from_str(&contents);

        l.push(material_list);
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
