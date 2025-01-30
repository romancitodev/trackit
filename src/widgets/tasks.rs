use std::time::Duration;

use dragking::{DragEvent, DropPosition};
use iced::{
    alignment::Horizontal,
    widget::{button, column, container, horizontal_rule, mouse_area, row, text},
    Element, Length,
};
use trackit_core::{chrono::Utc, Task};

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
    Card::new(index, task.clone()).view()
}

#[derive(Debug, Clone)]
pub enum Message {
    Start(u8),
    Delete(u8),
    Stop,
    StartHover(u8),
    EndHover(u8),
    Reorder(DragEvent),
}

pub struct Card {
    pub task: Task,
    pub index: u8,
    pub hovered: bool,
}

impl<'a> Card {
    pub fn new(index: u8, task: Task) -> Self {
        Self {
            task,
            index,
            hovered: false,
        }
    }

    pub fn placeholder() -> Element<'a, Message> {
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
    pub fn view(&self) -> Element<'a, Message> {
        let actions = row![
            button("Delete")
                .style(button::danger)
                .on_press(Message::Delete(self.index)),
            button("Start").on_press(Message::Start(self.index)),
            button("Stop")
                .style(button::secondary)
                .on_press(Message::Stop)
        ]
        .spacing(8);

        let card_content = column![
            text(self.task.name.clone()),
            horizontal_rule(1),
            column![
                text(format!("{} cycles", self.task.cycles)).style(text::secondary),
                text(calculate_cycles(self.task.cycles)).style(text::secondary)
            ],
        ]
        .push_maybe(self.hovered.then_some(actions))
        .spacing(4)
        .padding(8);

        let all = mouse_area(container(card_content).style(container::rounded_box))
            .on_enter(Message::StartHover(self.index))
            .on_exit(Message::EndHover(self.index));

        all.into()
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::StartHover(_) => self.hovered = true,
            Message::EndHover(_) => self.hovered = false,
            _ => {}
        }
    }
}

#[derive(Default)]
pub struct Cards {
    elements: Vec<Card>,
}

impl<'a> Cards {
    pub fn new(cards: Vec<Card>) -> Cards {
        Cards { elements: cards }
    }

    pub fn view(&self) -> Element<'a, Message> {
        let elements = self
            .elements
            .iter()
            .rev()
            .map(Card::view)
            .chain(self.elements.is_empty().then(Card::placeholder));

        let view = dragking::column(elements)
            .on_drag(Message::Reorder)
            .width(Length::FillPortion(2))
            .deadband_zone(0.0)
            .padding(8)
            .spacing(8);

        view.into()
    }

    pub fn elements(&self) -> &Vec<Card> {
        &self.elements
    }

    pub fn mut_elements(&mut self) -> &mut Vec<Card> {
        &mut self.elements
    }

    pub fn add_card(&mut self, task: Task) {
        let index = self.elements.len();
        self.elements.push(Card::new(index as u8, task));
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Start(index) => {
                let element = self
                    .elements
                    .get_mut(index as usize)
                    .expect("Unable to update");
                element.task.started_at = Some(Utc::now());
            }
            Message::Delete(index) => {
                self.elements.remove(index as usize);
                self.elements = self
                    .elements
                    .iter()
                    .enumerate()
                    .map(|(index, card)| Card {
                        index: index as u8,
                        task: card.task.clone(),
                        hovered: card.hovered,
                    })
                    .collect::<Vec<_>>();
            }
            Message::Stop => todo!(),
            msg @ Message::StartHover(index) => {
                let card = self.elements.get_mut(index as usize).unwrap();
                card.update(msg)
            }
            msg @ Message::EndHover(index) => {
                let card = self.elements.get_mut(index as usize).unwrap();
                card.update(msg)
            }
            Message::Reorder(event) => self.handle_reorder(event),
        }
    }
    fn handle_reorder(&mut self, event: DragEvent) {
        if let DragEvent::Dropped {
            index,
            target_index,
            drop_position,
        } = event
        {
            let len = self.elements.len();
            if len == 1 {
                return;
            };

            let index = len - index - 1;
            let target_index = len - target_index - 1;

            match drop_position {
                DropPosition::Before | DropPosition::After => {
                    if target_index != index && target_index != index + 1 {
                        let mut item = self.elements.remove(index);
                        let insert_index = if index < target_index {
                            target_index - 1
                        } else {
                            target_index
                        };

                        item.index = insert_index as u8;

                        self.elements.insert(insert_index, item);
                    }
                }
                DropPosition::Swap => {
                    if target_index != index {
                        self.elements.swap(index, target_index);
                        self.elements[index].index = index as u8;
                        self.elements[target_index].index = target_index as u8;
                    }
                }
            }
        }
    }
}
