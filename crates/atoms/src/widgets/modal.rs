use iced::{
    widget::{center, container, mouse_area, opaque, stack},
    Color, Renderer, Theme,
};

type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;

/// Creates a modal view with a background, content, and an action to trigger when the modal loses focus.
///
/// This function returns a stacked view with the background and content. The content is displayed
/// within a semi-transparent background and is interactive, allowing the modal to close when clicked outside
/// of the content area.
///
/// # Example
///
/// ```rust
/// use iced::{Element, Theme, Renderer};
/// use atom::widget::modal;
///
/// pub struct App;
///
/// The Message enum fort the app
/// #[derive(Debug, Clone)]
/// pub enum Message {
///  // ...
/// }
///
///
/// impl App {
///     pub fn view(&self) -> Element<Message> {
///       let background = ...; // Define your background element
///       let content = ...; // Define your content element
///       modal(background, content, Message::OnBlur)
///   }
/// }
///
/// ```
pub fn modal<'a, Message>(
    bg: impl Into<Element<'a, Message>>,
    content: Element<'a, Message>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack!(
        bg.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    )
    .into()
}
