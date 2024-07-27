use crate::consts;

use iced::{
    widget::{Column, Row, TextInput},
    Sandbox,
};

pub struct Browser {
    url_field: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
}

impl Sandbox for Browser {
    type Message = Message;

    fn new() -> Self {
        Self {
            url_field: "about:new".to_owned(),
        }
    }

    fn title(&self) -> String {
        format!("{} - Fiser", self.url_field)
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::UrlChanged(new_url) => {
                self.url_field = new_url;
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .padding(consts::PADDING)
            .spacing(consts::PADDING)
            .push(
                Row::new().spacing(consts::PADDING).push(
                    TextInput::new("URL here", &self.url_field).on_input(Message::UrlChanged)
                ),
            )
            .into()
    }
}
