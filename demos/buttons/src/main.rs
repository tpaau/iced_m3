use std::sync::LazyLock;

use iced::{
    Alignment, Element, Font, Length, Task,
    widget::{Container, center, column, container, row, text, text::IntoFragment},
};
use iced_m3::{
    theme::{Accent, ColorScheme, Mode, Theme},
    widget::{
        button::{Button, Content, CornerStyle, Style},
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

fn button<'a>(state: &'a State, style: Style, content: Content<'a>) -> Button<'a, Message> {
    iced_m3::widget::button(style, content)
        .icon_font(match state.icons_filled {
            true => fonts::icons_filled(),
            false => fonts::icons_outlined(),
        })
        .corner_style(match state.square {
            true => CornerStyle::Square,
            false => CornerStyle::Round,
        })
        .on_press(Message::Noop)
}

fn wrapper<'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    center(content)
        .height(CONTAINER_HEIGHT)
        .width(CONTAINER_WIDTH)
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        let icon = self.icon.to_string().into_fragment();
        let buttons = container(
            row![
                column![
                    wrapper(button(
                        self,
                        Style::elevated(&self.theme, self.accent),
                        Content::Full {
                            icon: icon.clone(),
                            label: "Elevated button".into(),
                        },
                    )),
                    wrapper(
                        button(
                            self,
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated unselected".into(),
                            },
                        )
                        .selected(false),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated selected".into(),
                            },
                        )
                        .selected(true),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated disabled".into(),
                            },
                        )
                        .on_press_maybe(None),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated disabled unselected".into(),
                            },
                        )
                        .selected(false)
                        .on_press_maybe(None),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::elevated(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Elevated disabled selected".into(),
                            },
                        )
                        .selected(true)
                        .on_press_maybe(None),
                    ),
                ],
                column![
                    wrapper(button(
                        self,
                        Style::filled(&self.theme, self.accent),
                        Content::Full {
                            icon: icon.clone(),
                            label: "Filled button".into(),
                        },
                    )),
                    wrapper(
                        button(
                            self,
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled unselected".into(),
                            },
                        )
                        .selected(false),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled selected".into(),
                            },
                        )
                        .selected(true),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled disabled".into(),
                            },
                        )
                        .on_press_maybe(None),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled disabled unselected".into(),
                            },
                        )
                        .selected(false)
                        .on_press_maybe(None),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::filled(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Filled disabled selected".into(),
                            },
                        )
                        .selected(true)
                        .on_press_maybe(None),
                    ),
                ],
                column![
                    wrapper(button(
                        self,
                        Style::tonal(&self.theme, self.accent),
                        Content::Full {
                            icon: icon.clone(),
                            label: "Tonal button".into(),
                        },
                    )),
                    wrapper(
                        button(
                            self,
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal unselected".into(),
                            },
                        )
                        .selected(false),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal selected".into(),
                            },
                        )
                        .selected(true),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal disabled".into(),
                            },
                        )
                        .on_press_maybe(None),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal disabled unselected".into(),
                            },
                        )
                        .selected(false)
                        .on_press_maybe(None),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::tonal(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Tonal disabled selected".into(),
                            },
                        )
                        .selected(true)
                        .on_press_maybe(None),
                    ),
                ],
                column![
                    wrapper(button(
                        self,
                        Style::outlined(&self.theme),
                        Content::Full {
                            icon: icon.clone(),
                            label: "Outlined button".into(),
                        },
                    )),
                    wrapper(
                        button(
                            self,
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined unselected".into(),
                            },
                        )
                        .selected(false),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined selected".into(),
                            },
                        )
                        .selected(true),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined disabled".into(),
                            },
                        )
                        .on_press_maybe(None),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined disabled unselected".into(),
                            },
                        )
                        .selected(false)
                        .on_press_maybe(None),
                    ),
                    wrapper(
                        button(
                            self,
                            Style::outlined(&self.theme),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Outlined disabled selected".into(),
                            },
                        )
                        .selected(true)
                        .on_press_maybe(None),
                    ),
                ],
                column![
                    wrapper(button(
                        self,
                        Style::text(&self.theme, self.accent),
                        Content::Full {
                            icon: icon.clone(),
                            label: "Text button".into(),
                        },
                    )),
                    wrapper(
                        button(
                            self,
                            Style::text(&self.theme, self.accent),
                            Content::Full {
                                icon: icon.clone(),
                                label: "Text disabled".into(),
                            },
                        )
                        .on_press_maybe(None),
                    ),
                ]
            ]
            .spacing(40.0),
        );

        let sized_buttons = row![
            button(
                self,
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::ExtraLarge),
            button(
                self,
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::Large),
            button(
                self,
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::Medium),
            button(
                self,
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::Small),
            button(
                self,
                Style::elevated(&self.theme, self.accent),
                Content::Full {
                    icon: icon.clone(),
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::ExtraSmall),
        ]
        .spacing(12.0);

        let dark_mode_switch = row![
            text("Dark mode").color(self.theme.on_surface()),
            switch(&self.theme, self.theme.mode == Mode::Dark).on_toggle(Message::ToggleDarkTheme)
        ]
        .align_y(Alignment::Center)
        .spacing(8.0);

        let square_buttons_switch = row![
            text("Square buttons").color(self.theme.on_surface()),
            switch(&self.theme, self.square).on_toggle(Message::ToggleSquareButtons)
        ]
        .align_y(Alignment::Center)
        .spacing(8.0);

        container(
            column![
                text(APP_NAME)
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
