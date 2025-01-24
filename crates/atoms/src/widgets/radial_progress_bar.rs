use std::f32::consts;

use iced::{
    alignment::{Horizontal, Vertical},
    mouse,
    widget::canvas::{self, path::Arc, Canvas, Frame, Text},
    Color, Point, Radians, Renderer, Theme,
};

pub struct RadialProgressBar(f32, String);

impl<Message, Theme: Catalog> canvas::Program<Message, Theme> for RadialProgressBar {
    type State = ();

    fn draw(
        &self,
        _: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        _: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let radius = frame.width().min(frame.height()) / 2.0;

        let inner_ball_radius = (radius * 0.2) / 2.0;
        let fixed_radius = radius * 0.9;
        let start_angle = Radians(-consts::FRAC_PI_2);
        let end_angle = Radians(consts::TAU) * self.0 / 100.0 + start_angle;

        let status = match self.0 {
            0.0 => Status::Idle,
            100.0 => Status::Finished,
            _ => Status::Progressing,
        };

        let style = <Theme as Catalog>::style(theme, &<Theme as Catalog>::default(), status);

        let circle_at_angle = |angle: Radians| {
            canvas::Path::circle(
                Point::new(
                    center.x + fixed_radius * angle.0.cos(),
                    center.y + fixed_radius * angle.0.sin(),
                ),
                inner_ball_radius,
            )
        };

        let circle = canvas::Path::circle(center, radius);
        let segment = canvas::Path::new(|builder| {
            builder.arc(Arc {
                center,
                radius,
                start_angle,
                end_angle,
            });
            builder.line_to(center);
            builder.close();
        });

        let start_point = circle_at_angle(start_angle);
        let end_point = circle_at_angle(end_angle);
        let inner_circle = canvas::Path::circle(center, radius * 0.8);

        frame.fill_text(Text {
            content: if self.1.is_empty() {
                format!("{:.2}%", self.0)
            } else {
                self.1.clone()
            },
            position: center,
            vertical_alignment: Vertical::Center,
            horizontal_alignment: Horizontal::Center,
            size: (radius / 3.).into(),
            color: style.text,
            ..default()
        });

        frame.fill(&circle, style.background);
        frame.fill(&segment, style.bar);
        frame.fill(&start_point, style.bar);
        frame.fill(&end_point, style.bar);
        frame.fill(&inner_circle, style.rail);

        vec![frame.into_geometry()]
    }
}

fn default<T: Default>() -> T {
    Default::default()
}

/// Creates a radial progress bar widget.
///
/// This function returns a `Canvas` widget that displays a radial progress bar
/// with a specified percentage and content. If the content is empty, the
/// percentage will be displayed by default.
///
/// # Example
///
/// ```rust
/// use atoms::widgets::radial_progress_bar;
///
/// let progress = radial_progress_bar(75., "75% Complete");
/// let default_progress = radial_progress_bar(75., "");
/// ```
pub fn radial_progress_bar<Message>(
    percentage: f32,
    content: impl Into<String>,
) -> Canvas<RadialProgressBar, Message, Theme, Renderer> {
    iced::widget::canvas(RadialProgressBar(percentage, content.into()))
}

#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// The [`Background`] of the progress bar widget
    pub background: Color,
    /// The [`Color`] of the progress bar
    pub bar: Color,
    /// The [`Color`] of the rail
    pub rail: Color,
    /// The [`Color`] of the text
    pub text: Color,
}

/// The theme Calatog of a [`RadialProgressBar`]
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`]
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// The appearance of the [`RadialProgressBar`].
#[derive(Debug, Clone, Copy)]
pub enum Status {
    /// The progress bar is idle.
    Idle,
    /// The progress bar is currently progressing.
    Progressing,
    /// The progress bar finished.
    Finished,
    /// The progress bar has failed.
    Failed,
}

/// A styling function for the [`RadialProgressBar`]
///
/// This is just a bloxed closure: `Fn(&Theme, Status) -> Style`.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The primary style of a [`RadialProgressBar`].
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let idle = Style {
        background: palette.background.weak.color.scale_alpha(0.2),
        rail: palette.background.base.color,
        bar: palette.background.strong.color,
        text: palette.background.base.text,
    };

    match status {
        Status::Idle => idle,
        Status::Progressing => Style {
            bar: palette.primary.base.color,
            ..idle
        },
        Status::Finished => Style {
            bar: palette.success.base.color,
            ..idle
        },
        Status::Failed => Style {
            bar: palette.danger.base.color,
            ..idle
        },
    }
}
