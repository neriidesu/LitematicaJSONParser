use iced::{
    Element, Function, Length, Task,
    widget::{Column, button, center_x, column, container, row, scrollable, text},
};

use crate::{
    material_list::MaterialList,
    widgets::{Item, ItemMessage},
};
use std::{fs, vec};

pub mod material_list;
pub mod widgets;

/* ERROR MESSAGE CONSTANTS */
const ERR_NO_MATERIAL_LIST: &str = "No material list loaded!";

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
enum Message {
    PagePreload(PagePreloadMessage),
    PageListLoaded(PageListLoadedMessage),
    ItemMessage(usize, ItemMessage),
}

trait Page {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>>;
    fn view(&self) -> iced::Element<'_, Message>;
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

    for material in material_list.Materials {
        material_list_items.push(Item::new(material));
    }

    let material_list: MaterialList = MaterialList::from_str(&contents);
    (material_list, material_list_items)
}

// PagePreload

#[derive(Debug, Clone)]
enum PagePreloadMessage {
    ButtonPressed,
}

struct PagePreload;

impl PagePreload {
    fn new() -> Self {
        Self
    }
}

impl Page for PagePreload {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        if let Message::PagePreload(msg) = message {
            match msg {
                PagePreloadMessage::ButtonPressed => return Some(Box::new(PageListLoaded::new())),
            }
        }
        None
    }

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            text("Hello!"),
            button("Load List").on_press(Message::PagePreload(PagePreloadMessage::ButtonPressed)),
        ]
        .into()
    }
}

// PageListLoaded

#[derive(Debug, Clone)]
enum PageListLoadedMessage {
    ExitButtonPressed,
}

struct PageListLoaded {
    list: Option<MaterialList>,
    items: Option<Vec<Item>>,
}

impl PageListLoaded {
    fn new() -> Self {
        let file_path = "./testdata/materials.json";
        let list = load_list(file_path);

        Self {
            //*
            list: Some(list.0),
            items: Some(list.1),
            // */
            /*
            list: None,
            items: None,
            // */
        }
    }
}

impl Page for PageListLoaded {
    fn update(&mut self, message: Message) -> Option<Box<dyn Page>> {
        match message {
            Message::ItemMessage(i, item_message) => {
                if let Some(item) = self.items.as_mut().expect(ERR_NO_MATERIAL_LIST).get_mut(i) {
                    item.update(item_message);
                }
            }
            _ => {
                if let Message::PageListLoaded(msg) = message {
                    match msg {
                        PageListLoadedMessage::ExitButtonPressed => {
                            return Some(Box::new(PagePreload::new()));
                        }
                    }
                }
            }
        }
        None
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // check if material list exists
        match &self.items {
            None => text!("{}", ERR_NO_MATERIAL_LIST).into(),
            Some(items) => {
                let c = Column::new();
                let it: Element<_> = items
                    .iter()
                    .fold(Column::new().spacing(10), |col, i| {
                        col.push(i.view().map(
                            Message::ItemMessage.with(items.iter().position(|r| r == i).unwrap()),
                        ))
                    })
                    .into();
                let a: Element<_> = c.push(it).into();

                let column = column![a,].spacing(10).max_width(800);

                let header = row![
                    text!(
                        "Material List for: {}",
                        self.list.clone().expect(ERR_NO_MATERIAL_LIST).Name
                    ),
                    button("Exit").on_press(Message::PageListLoaded(
                        PageListLoadedMessage::ExitButtonPressed
                    ))
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
    }
}
