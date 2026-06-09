use iced::{
    Element, Function, Length,
    widget::{Column, button, center_x, column, container, row, scrollable, text},
};
use serde::{Deserialize, Serialize};

use crate::{ERR_NO_MATERIAL_LIST, Message, SaveData, material_list::MaterialList, widgets::Item};

#[derive(Debug)]
pub struct ListData {
    pub list: Option<MaterialList>,
    pub items: Option<Vec<Item>>,
    pub filter: Filter,
}

impl ListData {
    pub fn from_data(data: SaveData) -> Self {
        Self {
            list: Some(data.material_list),
            items: Some(data.items),
            filter: Filter::All,
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
            filter: Filter::All,
        }
    }
}

pub fn view(list_data: &ListData) -> Element<'_, Message> {
    match &list_data.items {
        None => text!("{}", ERR_NO_MATERIAL_LIST).into(),
        Some(items) => {
            let c = Column::new();
            let filtered_items = items.iter().filter(|item| list_data.filter.matches(item));
            let it: Element<_> = if filtered_items.count() > 0 {
                items
                    .iter()
                    .filter(|item| list_data.filter.matches(item))
                    .fold(Column::new().spacing(10), |col, i| {
                        col.push(i.view().map(
                            Message::ItemMessage.with(items.iter().position(|r| r == i).unwrap()),
                        ))
                    })
                    .into()
            } else {
                match list_data.filter {
                    Filter::All => text("No items in list...").into(),
                    Filter::Active => text("All items completed! :D").into(),
                    Filter::Completed => text("You have not completed any items yet...").into(),
                }
            };

            let a: Element<_> = c.push(it).into();

            let column = column![a,].spacing(10).max_width(800);
            let header = row![
                text!(
                    "Material List for: {}",
                    list_data.list.clone().expect(ERR_NO_MATERIAL_LIST).Name
                ),
                button("Exit").on_press(Message::ExitListButtonPressed)
            ]
            .spacing(20)
            .padding(10);

            let content = column![
                container(header).center(Length::Fill).height(50.0),
                container(view_controls(
                    &list_data.items.as_ref().expect(ERR_NO_MATERIAL_LIST),
                    list_data.filter
                ))
                .center(Length::Fill)
                .height(50.0),
                scrollable(center_x(column).padding(40)).height(Length::Fill)
            ]
            .spacing(20);

            container(content).center(Length::Fill).into()
        }
    }
}

fn view_controls(items: &Vec<Item>, current_filter: Filter) -> Element<'_, Message> {
    let filter_button = |label, filter, current_filter| {
        let button = button(label).style(if filter == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChanged(filter))
    };

    row![
        filter_button("All", Filter::All, current_filter),
        filter_button("Incomplete", Filter::Active, current_filter),
        filter_button("Completed", Filter::Completed, current_filter),
    ]
    .spacing(20)
    .padding(10)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}
impl Filter {
    fn matches(self, item: &Item) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !item.completed,
            Filter::Completed => item.completed,
        }
    }
}
