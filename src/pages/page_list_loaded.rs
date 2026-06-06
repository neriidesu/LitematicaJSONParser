// PageListLoaded

use iced::{
    Element, Function, Length,
    widget::{Column, button, center_x, column, container, row, scrollable, text},
};

use crate::{
    ERR_NO_MATERIAL_LIST, Message, load_list,
    material_list::MaterialList,
    pages::{Page, page_preload::PagePreload},
    widgets::Item,
};

#[derive(Debug, Clone)]
pub enum PageListLoadedMessage {
    ExitButtonPressed,
}

pub struct PageListLoaded {
    list: Option<MaterialList>,
    items: Option<Vec<Item>>,
}

impl PageListLoaded {
    pub fn new() -> Self {
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
