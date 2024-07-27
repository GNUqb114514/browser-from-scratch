use crate::{consts, history::History};

use iced::{
    widget::{Button, Column, Row, Text, TextInput}, Sandbox
};

pub struct Browser {
    url_field: String,
    history: History
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    BackwardRequested,
    ForwardRequested,
    UrlSubmitted,
}

impl Browser {
    fn drop_url_field(&mut self) {
        self.url_field = self.history.current().clone();
    }
}

impl Sandbox for Browser {
    type Message = Message;

    fn new() -> Self {
        Self {
            url_field: "about:new".to_owned(),
            history: History::new("about:new".to_owned()),
        }
    }

    fn title(&self) -> String {
        format!("{} - Fiser", self.history.current())
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::UrlChanged(new_url) => {
                self.url_field = new_url;
            }
            Message::BackwardRequested => {
                self.history.backward().unwrap();
                self.drop_url_field();
            },
            Message::ForwardRequested => {
                self.history.forward().unwrap();
                self.drop_url_field();
            },
            Message::UrlSubmitted => {
                self.history.goto(self.url_field.clone());
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .padding(consts::PADDING)
            .spacing(consts::PADDING)
            .push(
                Row::new().spacing(consts::PADDING)
                    .push(
                        Button::new(Text::new("<")).on_press_maybe(if self.history.can_backward() {Some(Message::BackwardRequested)} else {None})
                    )
                    .push(
                        Button::new(Text::new(">")).on_press_maybe(if self.history.can_forward() {Some(Message::ForwardRequested)} else {None})
                    )
                    .push(
                        TextInput::new("URL here", &self.url_field).on_input(Message::UrlChanged).on_submit(Message::UrlSubmitted)
                    ),
            )
            .into()
    }
}
