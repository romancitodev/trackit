use std::time::Duration;

use atoms::widgets::radial_progress_bar;

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
use widgets::{modal::Modal, tasks::Cards};

#[derive(Default)]
pub struct App {
    modal: Modal,
    show_modal: bool,
    progress: f32,
    should_stop: bool,
    started_task: Option<Task>,
    cards: Cards,
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
    OpenModal,
    CloseModal,
}

type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;

impl App {
    const TITLE: &str = "Demo app";

    pub fn view(&self) -> Element<Message> {
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
                self.cards.view().map(Message::Card),
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
            Message::Tick => self.progress = (self.progress + 0.1).clamp(0.0, 100.),
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
                let task = Task::new(self.modal.task_name.clone(), self.modal.cycles);
                self.cards.add_card(task);
                self.modal.reset();
                self.show_modal = false;
            }
            Message::Modal(widgets::modal::Message::Cancel) | Message::CloseModal => {
                self.modal.reset();
                self.show_modal = false
            }
            Message::Modal(msg) => self.modal.update(msg),
            Message::OpenModal => self.show_modal = true,
            Message::Card(msg @ widgets::tasks::Message::Delete(index)) => {
                self.remove_task(index);
                self.cards.update(msg);
            }
            Message::Card(msg @ widgets::tasks::Message::Start(index)) => {
                // we ensure that the `card.started_task` is `Some(...)`
                self.cards.update(msg);
                self.start_task(index);
            }
            Message::Card(msg) => self.cards.update(msg),
        }
    }

    fn remove_task(&mut self, index: u8) {
        if let Some(task) = &self.started_task {
            let card = self
                .cards
                .elements()
                .get(index as usize)
                .expect("Index expected");
            if *task == card.task {
                self.started_task = None;
            }
        }
    }

    fn start_task(&mut self, index: u8) {
        let card = self
            .cards
            .mut_elements()
            .get_mut(index as usize)
            .expect("Index expected");

        if let Some(started) = &self.started_task {
            if card.task == *started {
                return;
            };
        }

        self.started_task = Some(card.task.clone());
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
