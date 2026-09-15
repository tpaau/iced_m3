use std::sync::LazyLock;

use fonts::{icons_outlined, text_bold};
use iced::{
    Alignment, Border, Element, Font, Length, Task, color,
    widget::{column, container, row, space, text},
};
use iced_m3::{
    theme::{ColorScheme, Theme},
    widget::badge,
};

const APP_NAME: &str = "Badges Demo";
static HOME: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe88a).unwrap());

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

impl State {
    fn view<'a>(&'a self) -> Element<'a, Message> {
        let icon = || -> Element<'_, Message> {
            iced_m3::widget::icon(*HOME, 24.0)
                .font(icons_outlined())
                .into()
        };

        let nav_bar_container = |content: Element<'a, Message>| -> Element<'a, Message> {
            container(content)
                .style(|_| {
                    container::Style::default()
                        .background(self.theme.secondary_container())
                        .border(Border::default().rounded(f32::MAX))
                })
                .center_x(56.0)
                .center_y(32.0)
                .into()
        };

        let badge_column = |content: Vec<Element<'a, Message>>| -> Element<'a, Message> {
            column(content)
                .align_x(Alignment::Center)
                .spacing(10)
                .into()
        };

        let badges = vec![
            badge_column(vec![
                // So that the icon is as big as the ones with padding
                column![badge(&self.theme, icon()), space().height(4.0)].into(),
                nav_bar_container(badge(&self.theme, icon()).into()),
            ]),
            badge_column(vec![
                badge(&self.theme, icon())
                    .label_font(text_bold())
                    .label("1")
                    .bounds_mode(badge::BoundsMode::Symmetrical)
                    .into(),
                nav_bar_container(
                    badge(&self.theme, icon())
                        .label_font(text_bold())
                        .label("1")
                        .bounds_mode(badge::BoundsMode::Symmetrical)
                        .into(),
                ),
            ]),
            badge_column(vec![
                badge(&self.theme, icon())
                    .label_font(text_bold())
                    .label("999+")
                    .bounds_mode(badge::BoundsMode::Symmetrical)
                    .into(),
                nav_bar_container(
                    badge(&self.theme, icon())
                        .label_font(text_bold())
                        .label("999+")
                        .bounds_mode(badge::BoundsMode::Symmetrical)
                        .into(),
                ),
            ]),
        ];

        let base = || -> Element<'_, Message> {
            container(space().width(24.0).height(24.0))
                .style(|_| container::Style::default().background(self.theme.secondary_container()))
                .into()
        };

        let badge_container =
            |label: &'a str, content: Element<'a, Message>| -> Element<'a, Message> {
                column![
                    text(label),
                    container(content)
                        .style(|_| container::Style::default().background(color!(0x0000ff)))
                ]
                .spacing(8.0)
                .into()
            };

        let badge_tests = vec![
            badge_container(
                "small badge",
                badge(&self.theme, base()).label_font(text_bold()).into(),
            ),
            badge_container(
                "large badge",
                badge(&self.theme, base())
                    .label_font(text_bold())
                    .label("1")
                    .into(),
            ),
            badge_container(
                "large badge with symmetrical padding",
                badge(&self.theme, base())
                    .label_font(text_bold())
                    .label("1")
                    .bounds_mode(badge::BoundsMode::Symmetrical)
                    .into(),
            ),
            badge_container(
                "large badge with max size",
                badge(&self.theme, base())
                    .label_font(text_bold())
                    .label("999+")
                    .into(),
            ),
            badge_container(
                "large badge with max size and symmetrical padding",
                badge(&self.theme, base())
                    .label_font(text_bold())
                    .label("999+")
                    .bounds_mode(badge::BoundsMode::Symmetrical)
                    .into(),
            ),
        ];

        container(
            column![
                text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                row(badges).spacing(20),
                text("those are just dirty tests, please ignore them: "),
                row(badge_tests).spacing(20.0),
            ]
            .spacing(20.0),
        )
        .padding(10.0)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| iced::widget::container::Style::default().background(self.theme.background()))
        .into()
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }
}

enum Message {}

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
