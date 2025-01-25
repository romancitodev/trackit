use iced::{
    alignment::Horizontal,
    widget::{column, container, horizontal_rule, horizontal_space, row, text},
    Element, Length,
};
use trackit_core::Task;

pub fn task_card<'a, Message>(task: &Task) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(
        column![
            text(task.name.clone()),
            horizontal_rule(1),
            row![
                text(format!("{} cycles remaining", task.cycles)).style(text::secondary),
                horizontal_space(),
                text("(25:00m)").style(text::secondary)
            ]
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
