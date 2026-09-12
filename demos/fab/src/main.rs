use std::sync::LazyLock;

use iced::{
    Element, Font, Length, Task,
    widget::{column, container, row, text},
};
use iced_m3::{
    theme::{ColorScheme, Theme},
    widget::fab,
    widget::fab::{Content, Size, Style},
};

const APP_NAME: &str = "FAB Demo";
static EDIT: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe3c9).unwrap());

struct State {
    theme: Theme,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(iced_m3::theme::Mode::Dark),
        }
    }
}

#[derive(Clone)]
enum Message {
    Noop,
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        let styles = [
            Style::PrimaryContainer,
            Style::SecondaryContainer,
            Style::TertiaryContainer,
            Style::Primary,
            Style::Secondary,
            Style::Tertiary,
        ];
        let sizes = [Size::Regular, Size::Medium, Size::Large];
        let content = Content::Reguar { icon: *EDIT };
        let fabs = row(sizes.into_iter().map(|size| {
            column(styles.into_iter().map(|style| {
                fab(&self.theme, content.clone(), Message::Noop)
                    .style(style)
                    .size(size)
                    .into()
            }))
            .spacing(20.0)
            .into()
        }))
        .spacing(20.0);

        let content = Content::Extended {
            icon: *EDIT,
            label: "Compose".into(),
        };
        let extended_fabs = row(sizes.into_iter().map(|size| {
            column(styles.into_iter().map(|style| {
                fab(&self.theme, content.clone(), Message::Noop)
                    .style(style)
                    .size(size)
                    .into()
            }))
            .spacing(20.0)
            .into()
        }))
        .spacing(20.0);

        container(
            column![
                text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                row![fabs, extended_fabs].spacing(20.0),
            ]
            .spacing(20.0),
        )
        .padding(10.0)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| iced::widget::container::Style::default().background(self.theme.background()))
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => Task::none(),
        }
    }
}

fn main() -> iced::Result {
    iced::application(
        || {
            (
                State::default(),
                Task::batch(
                    fonts::get_fonts()
                        .into_iter()
                        .map(|b| iced::font::load(b).discard()),
                ),
            )
        },
        State::update,
        State::view,
    )
    .title(APP_NAME)
    .default_font(Font::with_name("Noto Sans"))
    .run()
}
