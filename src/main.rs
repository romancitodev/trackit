use std::time::Duration;

use atoms::widgets::radial_progress_bar;
use dragking::{self, DragEvent, DropPosition};

use trackit_core::chrono::Utc;

mod widgets;

use iced::{
    application, time,
    widget::{
        button, column, container, row,
        rule::{self, Style},
        text, vertical_rule,
    },
    Length, Renderer, Subscription, Theme,
};
use trackit_core::Task;
use widgets::{
    modal::Modal,
    tasks::{placeholder, task_card},
};

#[derive(Default)]
pub struct App {
    modal: Modal,
    show_modal: bool,
    progress: f32,
    should_stop: bool,
    tasks: Vec<Task>,
    started_task: Option<Task>,
}

/// The Message enum for the app
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Stop,
    Resume,
    Restart,
    Modal(widgets::modal::Message),
    Card(widgets::tasks::Message),
    Reorder(DragEvent),
    OpenModal,
    CloseModal,
}

type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;

impl App {
    const TITLE: &str = "Demo app";

    pub fn view(&self) -> Element<Message> {
        let cards = self
            .tasks
            .iter()
            .rev()
            .enumerate()
            .map(|(i, task)| {
                let i = self.tasks.len() - i - 1;
                task_card(task, i as u8).map(Message::Card)
            })
            .chain(self.tasks.is_empty().then(placeholder));
        let col = dragking::column(cards)
            .width(Length::FillPortion(2))
            .on_drag(Message::Reorder)
            .deadband_zone(0.0)
            .padding(8)
            .spacing(8);

        let task_msg = match &self.started_task {
            Some(task) => format!(
                "Task {} started at: {}",
                task.name,
                task.started_at.unwrap().format("%H:%M (%d/%m/%Y)")
            ),
            None => "Not active task".into(),
        };

        let content: Element<_> = column![
            text(task_msg),
            container(
                radial_progress_bar(self.progress, "")
                    .width(100)
                    .height(100)
            )
            .width(Length::Fill)
            .center(Length::Fill),
            row![
                button("Resume").on_press(Message::Resume),
                button("Stop").on_press(Message::Stop),
                button("Restart").on_press(Message::Restart),
                button("Open / Close").on_press(Message::OpenModal)
            ]
            .spacing(5.)
            .width(Length::Fill)
        ]
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .into();

        let content: Element<_> = row![
            row![
                col,
                vertical_rule(1).style(|theme: &Theme| Style {
                    color: theme
                        .extended_palette()
                        .secondary
                        .base
                        .color
                        .scale_alpha(0.2),
                    ..rule::default(theme)
                })
            ],
            content
        ]
        .into();
        // let content = content.explain(Color::from_rgb(255., 0., 0.));

        if self.show_modal {
            self.modal.view(content)
        } else {
            content
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Tick => self.progress = (self.progress + 0.1).min(100.),
            Message::Stop => self.should_stop = true,
            Message::Resume => self.should_stop = false,
            Message::Restart => {
                self.should_stop = false;
                self.progress = 0.
            }
            Message::Modal(widgets::modal::Message::CreateNewTask) => {
                if self.modal.task_name.is_empty() {
                    self.modal.set_error("You must provide a text for the task");
                    return;
                };
                self.tasks
                    .push(Task::new(self.modal.task_name.clone(), self.modal.cycles));
                self.modal.reset();
                self.show_modal = false;
            }
            Message::Modal(widgets::modal::Message::Cancel) | Message::CloseModal => {
                self.modal.reset();
                self.show_modal = false
            }
            Message::Modal(msg) => self.modal.update(msg),
            Message::OpenModal => self.show_modal = true,
            Message::Card(widgets::tasks::Message::Delete(index)) => self.remove_task(index),
            Message::Card(widgets::tasks::Message::Start(index)) => self.start_task(index),
            Message::Card(_) => println!("from message"),
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
            let len = self.tasks.len();
            if len == 1 {
                return;
            };

            let index = len - index - 1;
            let target_index = len - target_index - 1;

            match drop_position {
                DropPosition::Before | DropPosition::After => {
                    if target_index != index && target_index != index + 1 {
                        let item = self.tasks.remove(index);
                        let insert_index = if index < target_index {
                            target_index - 1
                        } else {
                            target_index
                        };

                        self.tasks.insert(insert_index, item);
                    }
                }
                DropPosition::Swap => {
                    if target_index != index {
                        self.tasks.swap(index, target_index);
                    }
                }
            }
        }
    }

    fn remove_task(&mut self, index: u8) {
        if let Some(task) = &self.started_task {
            if *task == self.tasks[index as usize] {
                self.started_task = None;
            }
        }
        self.tasks.remove(index as usize);
    }

    fn start_task(&mut self, index: u8) {
        let task = self
            .tasks
            .get_mut(index as usize)
            .expect("Unable to get the task");

        if let Some(started) = &self.started_task {
            if task == started {
                return;
            };
        }

        task.started_at = Some(Utc::now());
        self.started_task = Some(task.clone());
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let time_sub = if !self.should_stop || self.progress >= 100. {
            time::every(Duration::from_millis(10)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        };

        iced::Subscription::batch([self.modal.subscription().map(Message::Modal), time_sub])
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    application(App::TITLE, App::update, App::view)
        .theme(|_| Theme::CatppuccinMocha)
        .antialiasing(true)
        .subscription(App::subscription)
        .run()?;
    Ok(())
}
