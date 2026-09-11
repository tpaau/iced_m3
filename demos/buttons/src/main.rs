use std::sync::LazyLock;

use iced::{
    Alignment, Element, Font, Length, Task,
    widget::{Container, center, column, container, row, text, toggler},
};
use iced_m3::{
    theme::{Accent, ColorScheme, Mode, Theme},
    widget::button::{Button, Content, CornerStyle, Style},
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
const CONTAINER_WIDTH: f32 = 220.0;

fn button<'a>(state: &'a State, style: Style, content: Content<'a>) -> Button<'a, Message> {
    iced_m3::widget::button(&state.theme, content)
        .style(style)
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
        .height(Length::Shrink)
        .width(Length::Fixed(CONTAINER_WIDTH))
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        let buttons = container(
            column![
                row![
                    wrapper(button(
                        self,
                        Style::Elevated(self.accent),
                        Content::Full {
                            icon: self.icon,
                            label: "Elevated button".into()
                        }
                    )),
                    wrapper(
                        button(
                            self,
                            Style::Elevated(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Elevated unselected".into()
                            }
                        )
                        .selected(false)
                    ),
                    wrapper(
                        button(
                            self,
                            Style::Elevated(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Elevated selected".into()
                            }
                        )
                        .selected(true)
                    ),
                    wrapper(
                        button(
                            self,
                            Style::Elevated(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Elevated disabled".into()
                            }
                        )
                        .on_press_maybe(None)
                    ),
                ],
                row![
                    wrapper(button(
                        self,
                        Style::Filled(self.accent),
                        Content::Full {
                            icon: self.icon,
                            label: "Filled button".into()
                        }
                    )),
                    wrapper(
                        button(
                            self,
                            Style::Filled(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Filled unselected".into()
                            }
                        )
                        .selected(false)
                    ),
                    wrapper(
                        button(
                            self,
                            Style::Filled(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Filled selected".into()
                            }
                        )
                        .selected(true)
                    ),
                    wrapper(
                        button(
                            self,
                            Style::Filled(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Filled disabled".into()
                            }
                        )
                        .on_press_maybe(None)
                    ),
                ],
                row![
                    wrapper(button(
                        self,
                        Style::Tonal(self.accent),
                        Content::Full {
                            icon: self.icon,
                            label: "Tonal button".into()
                        }
                    )),
                    wrapper(
                        button(
                            self,
                            Style::Tonal(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Tonal unselected".into()
                            }
                        )
                        .selected(false)
                    ),
                    wrapper(
                        button(
                            self,
                            Style::Tonal(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Tonal selected".into()
                            }
                        )
                        .selected(true)
                    ),
                    wrapper(
                        button(
                            self,
                            Style::Tonal(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Tonal disabled".into()
                            }
                        )
                        .on_press_maybe(None)
                    ),
                ],
                row![
                    wrapper(button(
                        self,
                        Style::Outlined,
                        Content::Full {
                            icon: self.icon,
                            label: "Outlined button".into()
                        }
                    )),
                    wrapper(
                        button(
                            self,
                            Style::Outlined,
                            Content::Full {
                                icon: self.icon,
                                label: "Outlined unselected".into()
                            }
                        )
                        .selected(false)
                    ),
                    wrapper(
                        button(
                            self,
                            Style::Outlined,
                            Content::Full {
                                icon: self.icon,
                                label: "Outlined selected".into()
                            }
                        )
                        .selected(true)
                    ),
                    wrapper(
                        button(
                            self,
                            Style::Outlined,
                            Content::Full {
                                icon: self.icon,
                                label: "Outlined disabled".into()
                            }
                        )
                        .on_press_maybe(None)
                    ),
                ],
                row![
                    wrapper(button(
                        self,
                        Style::Text(self.accent),
                        Content::Full {
                            icon: self.icon,
                            label: "Text button".into()
                        }
                    )),
                    wrapper(
                        button(
                            self,
                            Style::Text(self.accent),
                            Content::Full {
                                icon: self.icon,
                                label: "Text disabled".into()
                            }
                        )
                        .on_press_maybe(None)
                    )
                ]
            ]
            .spacing(40.0),
        );

        let sized_buttons = column![
            button(
                self,
                Style::Elevated(self.accent),
                Content::Full {
                    icon: self.icon,
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::ExtraSmall),
            button(
                self,
                Style::Elevated(self.accent),
                Content::Full {
                    icon: self.icon,
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::Small),
            button(
                self,
                Style::Elevated(self.accent),
                Content::Full {
                    icon: self.icon,
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::Medium),
            button(
                self,
                Style::Elevated(self.accent),
                Content::Full {
                    icon: self.icon,
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::Large),
            button(
                self,
                Style::Elevated(self.accent),
                Content::Full {
                    icon: self.icon,
                    label: "Common button".into()
                }
            )
            .size(iced_m3::widget::button::Size::ExtraLarge),
        ]
        .spacing(12.0);

        let dark_mode_toggler = row![
            text("Dark mode").color(self.theme.on_surface()),
            toggler(self.theme.mode.is_dark()).on_toggle(|_| Message::ToggleDarkTheme)
        ]
        .align_y(Alignment::Center)
        .spacing(8.0);

        let square_buttons_toggler = row![
            text("Square buttons").color(self.theme.on_surface()),
            toggler(self.square).on_toggle(|_| Message::ToggleSquareButtons)
        ]
        .align_y(Alignment::Center)
        .spacing(8.0);

        container(
            column![
                text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                row![buttons, sized_buttons].spacing(20.0),
                dark_mode_toggler,
                square_buttons_toggler
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
