use crate::Message as AppMessage;
use atoms::widgets::modal;
use iced::{
    alignment::Vertical,
    keyboard::{key::Named, Key},
    widget::{button, column, container, horizontal_space, row, text, text_input},
    Element, Length,
};

use iced_aw::number_input;

/// The `Message` enum represents the different messages that can be sent within the modal.
#[derive(Debug, Clone)]
pub enum Message {
    /// Message variant for when the task name changes.
    TaskNameChanged(String),
    /// Message variant for when the cycles changes.
    CyclesChanged(u8),
    Cancel,
    CreateNewTask,
}

/// The `Modal` struct represents the state of the modal
#[derive(Debug)]
pub struct Modal {
    pub task_name: String,
    pub cycles: u8,
    pub error_msg: Option<String>,
}

impl Default for Modal {
    fn default() -> Self {
        Self {
            cycles: 1,
            task_name: String::default(),
            error_msg: None,
        }
    }
}

impl<'a> Modal {
    pub fn view(&self, bg: impl Into<Element<'a, AppMessage>>) -> Element<'a, AppMessage> {
        let full_field = column([
            text("Task name:").into(),
            text_input("Do some stuff", &self.task_name)
                .on_input(Message::TaskNameChanged)
                .into(),
        ])
        .push_maybe(self.error_msg.clone().map(|t| text(t).style(text::danger)));

        let content: Element<_> = container(
            column!(
                full_field.spacing(8),
                row![
                    text("Cycle count:"),
                    horizontal_space(),
                    number_input(self.cycles, 1..=10u8, Message::CyclesChanged)
                        .style(number_input::number_input::primary)
                ]
                .align_y(Vertical::Center),
                container(
                    row![
                        button("Cancel")
                            .style(button::danger)
                            .on_press(Message::Cancel),
                        button("Create").on_press(Message::CreateNewTask)
                    ]
                    .spacing(8)
                )
                .align_right(Length::Fill)
            )
            .spacing(16),
        )
        .width(400)
        .height(Length::Shrink)
        .padding(16)
        .style(container::rounded_box)
        .into();

        modal(bg, content.map(AppMessage::Modal), AppMessage::CloseModal)
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::TaskNameChanged(user) => {
                self.task_name = user;
                if self.task_name.is_empty() {
                    self.set_error("You must provide a name for the task")
                } else {
                    self.error_msg = None
                };
            }
            Message::CyclesChanged(cycles) => self.cycles = cycles,
            // catched at app level
            Message::Cancel | Message::CreateNewTask => {}
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::on_key_press(|k, _| {
            if k == Key::Named(Named::Enter) {
                Some(Message::CreateNewTask)
            } else if k == Key::Named(Named::Escape) {
                Some(Message::Cancel)
            } else {
                None
            }
        })
    }

    pub fn set_error(&mut self, error: impl Into<String>) {
        self.error_msg = Some(error.into())
    }
}
