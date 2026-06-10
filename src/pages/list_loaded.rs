use std::{
    fs::{self, File},
    io::Write,
};

use iced::{
    Alignment, Element, Function, Length,
    widget::{Column, button, center_x, column, container, progress_bar, row, scrollable, text},
};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};

use crate::{
    APP_NAME, ERR_NO_MATERIAL_LIST, Message, SaveData, material_list::MaterialList, widgets::Item,
};

const UNOBTAINABLES: [&str; 30] = [
    "minecraft:bedrock",
    "minecraft:budding_amethyst",
    "minecraft:chorus_plant",
    "minecraft:end_gateway",
    "minecraft:end_portal",
    "minecraft:end_portal_frame",
    "minecraft:farmland",
    "minecraft:frog_spawn",
    "minecraft:infested_stone",
    "minecraft:infested_cobblestone",
    "minecraft:infested_stone_bricks",
    "minecraft:infested_mossy_stone_bricks",
    "minecraft:infested_cracked_stone_bricks",
    "minecraft:infested_chiseled_stone_bricks",
    "minecraft:infested_deepslate",
    "minecraft:mob_spawner",
    "minecraft:reinforced_deepslate",
    "minecraft:trial_spawner",
    "minecraft:vault",
    "minecraft:barrier",
    "minecraft:command_block",
    "minecraft:chain_command_block",
    "minecraft:repeating_command_block",
    "minecraft:jigsaw",
    "minecraft:light",
    "minecraft:petrified_oak_slab",
    "minecraft:structure_block",
    "minecraft:structure_void",
    "minecraft:test_block",
    "minecraft:test_instance_block",
];

#[derive(Debug)]
pub struct ListData {
    pub list: Option<MaterialList>,
    pub items: Option<Vec<Item>>,
    pub filter: Filter,
}

impl ListData {
    pub fn from_data(data: SaveData) -> Self {
        let items = remove_auto_hides(data.items);

        Self {
            list: Some(data.material_list),
            items: Some(items),
            filter: Filter::All,
        }
    }

    pub fn from_list(material_list: MaterialList) -> Self {
        let mut material_list_items = vec![];

        for material in &material_list.Materials {
            material_list_items.push(Item::new(material.clone()));
        }

        let items = remove_auto_hides(material_list_items);

        Self {
            list: Some(material_list),
            items: Some(items),
            filter: Filter::All,
        }
    }
}

fn remove_auto_hides(items: Vec<Item>) -> Vec<Item> {
    let mut items = items;
    // auto remove unobtainables
    for item in items.clone() {
        if UNOBTAINABLES.contains(&item.material.Item.as_str()) {
            items.remove(items.iter().position(|r| *r == item).unwrap());
        }
    }

    // check if user has specified hides in config

    let user_hides = match read_user_hides() {
        Ok(v) => v,
        Err(err) => {
            println!("Error reading user hides: {}", err);
            vec![]
        }
    };
    for item in items.clone() {
        if user_hides.contains(&item.material.Item) {
            items.remove(items.iter().position(|r| *r == item).unwrap());
        }
    }

    items
}

pub fn read_user_hides() -> anyhow::Result<Vec<String>> {
    #[derive(Serialize, Deserialize)]
    struct Hides {
        user_hides: Vec<String>,
    }

    let config_path = AppDirs::new(Some(APP_NAME), true).unwrap().config_dir;

    if !config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }

    let auto_hide_path = &config_path.join("hide_items").with_extension("json");

    let hides = if !auto_hide_path.exists() {
        let mut file = File::create(auto_hide_path)?;
        file.write_all(b"{\"user_hides\":[]}")?;

        vec![]
    } else {
        let data = fs::read_to_string(auto_hide_path)?;

        let h: Hides = serde_json::from_str(&data)?;

        h.user_hides
    };

    Ok(hides)
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
            .align_y(Alignment::Center)
            .spacing(20)
            .padding(10);

            let content = column![
                container(header).center(Length::Fill).height(50.0),
                container(view_controls(
                    &list_data.items.as_ref().expect(ERR_NO_MATERIAL_LIST),
                    list_data.filter
                ))
                .center(Length::Fill)
                .height(100.0),
                scrollable(center_x(column).padding(40)).height(Length::Fill)
            ]
            .spacing(20);

            container(content).center(Length::Fill).into()
        }
    }
}

fn view_controls(items: &Vec<Item>, current_filter: Filter) -> Element<'_, Message> {
    let items_left = items.iter().filter(|item| !item.completed).count();

    let filter_button = |label, filter, current_filter| {
        let button = button(label).style(if filter == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChanged(filter))
    };
    column![
        column![
            text!(
                "{items_left} {} left",
                if items_left == 1 { "item" } else { "items" }
            ),
            progress_bar(0.0..=items.iter().count() as f32, items_left as f32)
        ]
        .height(25)
        .align_x(Alignment::Center)
        .max_width(800),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Incomplete", Filter::Active, current_filter),
            filter_button("Completed", Filter::Completed, current_filter),
        ]
    ]
    .align_x(Alignment::Center)
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
