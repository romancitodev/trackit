use iced::{
    alignment::Horizontal,
    widget::{button, column, container, horizontal_rule, horizontal_space, row, text},
    Element, Length,
};
use trackit_core::Task;

pub fn task_card<'a>(task: &Task, index: u8) -> Element<'a, Message> {
    container(
        column![
            text(task.name.clone()),
            horizontal_rule(1),
            row![
                text(format!("{} cycles remaining", task.cycles)).style(text::secondary),
                horizontal_space(),
                text("(25:00m)").style(text::secondary)
            ],
            row![
                button("Delete")
                    .style(button::danger)
                    .on_press(Message::Delete(index)),
                button("Start").on_press(Message::Start(index)),
                button("Stop")
                    .style(button::secondary)
                    .on_press(Message::Stop)
            ]
            .spacing(8)
        ]
        .spacing(4)
        .padding(8),
    )
    .style(container::rounded_box)
    .into()
}

pub fn placeholder<'a, Message>() -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(
        column![
            text("Empty task list").style(text::secondary),
            text("try adding one!").style(text::secondary),
        ]
        .align_x(Horizontal::Center)
        .spacing(4)
        .padding(8),
    )
    .width(Length::Fill)
    .align_x(Horizontal::Center)
    .style(container::transparent)
    .into()
}

#[derive(Debug, Clone)]
pub enum Message {
    Start(u8),
    Delete(u8),
    Stop,
}
