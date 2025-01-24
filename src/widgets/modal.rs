use crate::Message as AppMessage;
use atoms::widgets::modal;
use iced::{
    widget::{column, container, text_input},
    Element,
};

/// The `Message` enum represents the different messages that can be sent within the modal.
#[derive(Debug, Clone)]
pub enum Message {
    /// Message variant for when the username changes.
    UsernameChanged(String),
}

/// The `Modal` struct represents the state of the modal
#[derive(Debug, Default)]
pub struct Modal {
    pub username: String,
}

impl<'a> Modal {
    pub fn view(&self, bg: impl Into<Element<'a, AppMessage>>) -> Element<'a, AppMessage> {
        let content: Element<Message> = container(column!(
            text_input("name", &self.username).on_input(Message::UsernameChanged)
        ))
        .width(400)
        .height(400)
        .into();

        modal(bg, content.map(AppMessage::Modal), AppMessage::CloseModal)
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::UsernameChanged(user) => self.username = user,
        }
    }
}
