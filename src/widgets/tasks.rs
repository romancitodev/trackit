use std::time::Duration;

use iced::{
    alignment::Horizontal,
    widget::{button, column, container, horizontal_rule, row, text},
    Element, Length,
};
use trackit_core::Task;

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;

    if hours > 0 {
        format!("{}h {:02}m", hours, minutes)
    } else {
        format!("{:02}m", minutes)
    }
}

fn calculate_cycles(cycles: u8) -> String {
    let time = Duration::from_secs(60 * 25 * cycles as u64);
    let breaks = if cycles > 1 {
        Duration::from_secs((cycles as u64 - 1) * 5 * 60)
    } else {
        Duration::ZERO
    };

    if breaks.is_zero() {
        format_duration(time)
    } else {
        format!(
            "{} + {} (tot. break time)",
            format_duration(time),
            format_duration(breaks)
        )
    }
}

pub fn task_card<'a>(task: &Task, index: u8) -> Element<'a, Message> {
    container(
        column![
            text(task.name.clone()),
            horizontal_rule(1),
            column![
                text(format!("{} cycles", task.cycles)).style(text::secondary),
                text(calculate_cycles(task.cycles)).style(text::secondary)
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
