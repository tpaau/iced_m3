use std::sync::LazyLock;

use iced::{
    Alignment, Element, Font, Length, Task,
    widget::{Container, center, column, container, row, text::IntoFragment},
};
use iced_m3::{
    theme::{Accent, ColorScheme, Mode, Theme},
    widget::{
        button::{self, Content, CornerRadius, CornerStyle, Style},
        hybrid_icon::Icon,
        switch,
    },
};

#[derive(Clone)]
enum Message {
    Noop,
    ToggleDarkTheme,
    ToggleSquareButtons,
}

struct State {
    theme: Theme,
    accent: Accent,
    icons_filled: bool,
    square: bool,
    icon: char,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            accent: Accent::Primary,
            icon: *EDIT,
            square: false,
            icons_filled: false,
        }
    }
}

const APP_NAME: &str = "Buttons demo";
static EDIT: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe3c9).unwrap());
const CONTAINER_HEIGHT: f32 = 50.0;
const CONTAINER_WIDTH: f32 = 250.0;

fn wrapper<'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    center(content)
        .height(CONTAINER_HEIGHT)
        .width(CONTAINER_WIDTH)
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        let corner_style = match self.square {
            true => CornerStyle::Square,
            false => CornerStyle::Rounded,
        };
        let size = button::Size {
            corner_radius: CornerRadius::default().style(corner_style),
            ..button::Size::default()
        };
        let icon = Icon::Text {
            text: self.icon.into_fragment(),
            font: Some(match self.icons_filled {
                true => fonts::icons_filled(),
                false => fonts::icons_outlined(),
            }),
        };
        let buttons = container(
            row![
                column![
                    wrapper(
                        iced_m3::widget::button(
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated button".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated unselected".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                        .selected(false),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated selected".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                        .selected(true),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated disabled".into(),
                            },
                        )
                        .size(size),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated disabled unselected".into(),
                            },
                        )
                        .size(size)
                        .selected(false),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated disabled selected".into(),
                            },
                        )
                        .size(size)
                        .selected(true),
                    ),
                ],
                column![
                    wrapper(
                        iced_m3::widget::button(
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled button".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled unselected".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                        .selected(false),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled selected".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                        .selected(true),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled disabled".into(),
                            },
                        )
                        .size(size)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled disabled unselected".into(),
                            },
                        )
                        .size(size)
                        .selected(false)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled disabled selected".into(),
                            },
                        )
                        .size(size)
                        .selected(true)
                    ),
                ],
                column![
                    wrapper(
                        iced_m3::widget::button(
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal button".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal unselected".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                        .selected(false),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal selected".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                        .selected(true),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal disabled".into(),
                            },
                        )
                        .size(size)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal disabled unselected".into(),
                            },
                        )
                        .size(size)
                        .selected(false)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal disabled selected".into(),
                            },
                        )
                        .size(size)
                        .selected(true)
                    ),
                ],
                column![
                    wrapper(
                        iced_m3::widget::button(
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined button".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined unselected".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                        .selected(false),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined selected".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                        .selected(true),
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined disabled".into(),
                            },
                        )
                        .size(size)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined disabled unselected".into(),
                            },
                        )
                        .size(size)
                        .selected(false)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined disabled selected".into(),
                            },
                        )
                        .size(size)
                        .selected(true)
                    ),
                ],
                column![
                    wrapper(
                        iced_m3::widget::button(
                            Style::text(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Text button".into(),
                            },
                        )
                        .size(size)
                        .on_press(Message::Noop)
                    ),
                    wrapper(
                        iced_m3::widget::button(
                            Style::text(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Text disabled".into(),
                            },
                        )
                        .size(size)
                        .on_press_maybe(None),
                    ),
                ]
            ]
            .spacing(40.0),
        );

        let sized_buttons = row![
            iced_m3::widget::button(
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(button::Size {
                corner_radius: CornerRadius::extra_large().style(corner_style),
                ..button::Size::extra_large()
            })
            .on_press(Message::Noop),
            iced_m3::widget::button(
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(button::Size {
                corner_radius: CornerRadius::large().style(corner_style),
                ..button::Size::large()
            })
            .on_press(Message::Noop),
            iced_m3::widget::button(
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(button::Size {
                corner_radius: CornerRadius::medium().style(corner_style),
                ..button::Size::medium()
            })
            .on_press(Message::Noop),
            iced_m3::widget::button(
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(button::Size {
                corner_radius: CornerRadius::small().style(corner_style),
                ..button::Size::small()
            })
            .on_press(Message::Noop),
            iced_m3::widget::button(
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(button::Size {
                corner_radius: CornerRadius::extra_small().style(corner_style),
                ..button::Size::extra_small()
            })
            .on_press(Message::Noop),
        ]
        .spacing(12.0);

        let dark_mode_switch = row![
            iced::widget::text("Dark mode").color(self.theme.on_surface()),
            switch(&self.theme, self.theme.mode == Mode::Dark).on_toggle(Message::ToggleDarkTheme)
        ]
        .align_y(Alignment::Center)
        .spacing(8.0);

        let square_buttons_switch = row![
            iced::widget::text("Square buttons").color(self.theme.on_surface()),
            switch(&self.theme, self.square).on_toggle(Message::ToggleSquareButtons)
        ]
        .align_y(Alignment::Center)
        .spacing(8.0);

        container(
            column![
                iced::widget::text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                column![buttons, sized_buttons].spacing(20.0),
                dark_mode_switch,
                square_buttons_switch
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
            Message::Noop => {}
            Message::ToggleDarkTheme => {
                self.theme.mode = match self.theme.mode {
                    Mode::Light => Mode::Dark,
                    Mode::Dark => Mode::Light,
                }
            }
            Message::ToggleSquareButtons => self.square = !self.square,
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
