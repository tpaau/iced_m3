use std::sync::LazyLock;

use iced::{
    Element, Font, Length, Task,
    widget::{column, container, text::IntoFragment},
};
use iced_m3::{
    theme::{ColorScheme, Mode, Theme},
    widget::{Badge, hybrid_icon::Icon, navbar},
};

const APP_NAME: &str = "Navigation Bar Demo";
static STAR: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8d0).unwrap());
static FAVORITE: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe87d).unwrap());
static SEARCH: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8b6).unwrap());
static SETTINGS: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8b8).unwrap());

#[derive(Clone, Copy, Default)]
#[repr(usize)]
enum Tab {
    #[default]
    Star,
    Favorites,
    Search,
    Settings,
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tab::Star => write!(f, "Star"),
            Tab::Favorites => write!(f, "Favorites"),
            Tab::Search => write!(f, "Search"),
            Tab::Settings => write!(f, "Settings"),
        }
    }
}

impl Tab {
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone)]
enum Message {
    GoTo(Tab),
}

struct State {
    tab: Tab,
    theme: Theme,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            tab: Tab::default(),
        }
    }
}

impl State {
    fn view<'a>(&'a self) -> Element<'a, Message> {
        let items = vec![
            navbar::Item {
                icon_active: Icon::Text {
                    text: STAR.into_fragment(),
                    font: Some(fonts::icons_filled()),
                },
                icon_inactive: Icon::Text {
                    text: STAR.into_fragment(),
                    font: Some(fonts::icons_outlined()),
                },
                badge: None,
                label: format!("{}", Tab::Star).into(),
                on_press: Message::GoTo(Tab::Star),
            },
            navbar::Item {
                icon_active: Icon::Text {
                    text: FAVORITE.into_fragment(),
                    font: Some(fonts::icons_filled()),
                },
                icon_inactive: Icon::Text {
                    text: FAVORITE.into_fragment(),
                    font: Some(fonts::icons_outlined()),
                },
                badge: Some(Badge { label: None }),
                label: format!("{}", Tab::Favorites).into(),
                on_press: Message::GoTo(Tab::Favorites),
            },
            navbar::Item {
                icon_active: Icon::Text {
                    text: SEARCH.into_fragment(),
                    font: Some(fonts::icons_filled()),
                },
                icon_inactive: Icon::Text {
                    text: SEARCH.into_fragment(),
                    font: Some(fonts::icons_outlined()),
                },
                badge: Some(Badge {
                    label: Some("1".into()),
                }),
                label: format!("{}", Tab::Search).into(),
                on_press: Message::GoTo(Tab::Search),
            },
            navbar::Item {
                icon_active: Icon::Text {
                    text: SETTINGS.into_fragment(),
                    font: Some(fonts::icons_filled()),
                },
                icon_inactive: Icon::Text {
                    text: SETTINGS.into_fragment(),
                    font: Some(fonts::icons_outlined()),
                },
                badge: Some(Badge {
                    label: Some("999+".into()),
                }),
                label: format!("{}", Tab::Settings).into(),
                on_press: Message::GoTo(Tab::Settings),
            },
        ];

        let bar = |label: &'a str, content: Element<'a, Message>| -> Element<'a, Message> {
            column![
                container(
                    iced::widget::text(label)
                        .size(18.0)
                        .font(fonts::text_bold())
                )
                .center_x(Length::Fill),
                content
            ]
            .spacing(10)
            .into()
        };

        let active_index = self.tab.index();
        container(
            column![
                iced::widget::text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                container(
                    column![
                        bar(
                            "Compact",
                            navbar(navbar::Style::new(&self.theme), items.clone())
                                .active_index(active_index)
                                .mode(navbar::Mode::Compact)
                                .into()
                        ),
                        bar(
                            "Large",
                            navbar(navbar::Style::new(&self.theme), items.clone())
                                .active_index(active_index)
                                .mode(navbar::Mode::Large)
                                .into()
                        ),
                        bar(
                            "Auto",
                            navbar(navbar::Style::new(&self.theme), items)
                                .active_index(active_index)
                                .mode(navbar::Mode::Auto)
                                .into()
                        )
                    ]
                    .spacing(20)
                )
                .width(600)
            ]
            .spacing(20),
        )
        .padding(10.0)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| iced::widget::container::Style::default().background(self.theme.background()))
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GoTo(tab) => self.tab = tab,
        }
        Task::none()
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
