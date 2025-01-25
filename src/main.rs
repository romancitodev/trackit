use atoms::widgets::radial_progress_bar;
mod widgets;

use iced::{
    application, time,
    widget::{button, column, container, row},
    Color, Length, Renderer, Subscription, Theme,
};
use widgets::modal::Modal;

#[derive(Default)]
pub struct App {
    modal: Modal,
    show_modal: bool,
    progress: f32,
    should_stop: bool,
}

/// The Message enum fort the app
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Stop,
    Resume,
    Restart,
    Modal(widgets::modal::Message),
    OpenModal,
    CloseModal,
}

type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;

impl App {
    const TITLE: &str = "Demo app";

    pub fn view(&self) -> Element<Message> {
        let content: Element<_> = column![
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
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        let content = content.explain(Color::from_rgb(255., 0., 0.));

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
            Message::Modal(widgets::modal::Message::Cancel) | Message::CloseModal => {
                self.modal.reset();
                self.show_modal = false
            }
            Message::Modal(msg) => self.modal.update(msg),
            Message::OpenModal => self.show_modal = true,
        };
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        if self.should_stop || self.progress >= 100. {
            Subscription::none()
        } else {
            time::every(std::time::Duration::from_millis(10)).map(|_| Message::Tick)
        }
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
