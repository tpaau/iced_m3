use std::{ops::Not, sync::LazyLock};

use fonts::icons_filled;
use iced::{
    Element, Font, Length, Task,
    advanced::svg,
    widget::{column, container, row, text::IntoFragment},
};
use iced_m3::{
    theme::{ColorScheme, Mode, Theme},
    widget::{
        Action, advanced, button,
        hybrid_icon::Icon,
        menu::{self, Menu, button::MenuAction},
    },
};

const APP_NAME: &str = "Menu Demo";
static VISIBILITY: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8f4).unwrap());
static EDIT: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe3c9).unwrap());
static STARS: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8d0).unwrap());
static MORE_HORIZ: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe5d3).unwrap());

const CHECK_CIRCLE_BYTES: &[u8] = include_bytes!("../assets/check_circle.svg");
static CHECK_CIRCLE_HANDLE: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(CHECK_CIRCLE_BYTES));

#[derive(Clone)]
enum Message {
    Noop,
    ToggleDropDownMenu,
    ToggleSubMenu1,
    ToggleSubMenu2,
}

struct State {
    theme: Theme,
    drop_down_menu_opened: bool,
    sub_menu_1_opened: bool,
    sub_menu_2_opened: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            drop_down_menu_opened: false,
            sub_menu_1_opened: false,
            sub_menu_2_opened: false,
        }
    }
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        let surface = |submenu_opened: bool, submenu_toggle: Message| -> Element<'_, Message> {
            let style = menu::Style::new(&self.theme, false);
            let submenu = move || -> Menu<'_, Message> {
                let groups = vec![menu::Group {
                    label: Some("Submenus!".into()),
                    items: vec![menu::Item::Button(menu::button::Button {
                        icon: Some(Icon::Text {
                            text: EDIT.into_fragment(),
                            font: Some(fonts::icons_outlined()),
                        }),
                        label: "Sub item!!".into(),
                        supporting_text: None,
                        action: MenuAction::Message {
                            is_selected: false,
                            on_press: Some(Action::Direct(Message::Noop)),
                        },
                    })],
                }];
                iced_m3::widget::menu(style, groups)
            };

            let groups = vec![
                menu::Group::new(vec![
                    menu::Item::Button(menu::button::Button {
                        icon: Some(Icon::Text {
                            text: VISIBILITY.into_fragment(),
                            font: Some(fonts::icons_outlined()),
                        }),
                        label: "Item 1".into(),
                        supporting_text: None,
                        action: MenuAction::Message {
                            is_selected: false,
                            on_press: Some(Action::Direct(Message::Noop)),
                        },
                    }),
                    menu::Item::Button(menu::button::Button {
                        icon: Some(Icon::Text {
                            text: STARS.into_fragment(),
                            font: Some(fonts::icons_outlined()),
                        }),
                        label: "Item 2".into(),
                        supporting_text: None,
                        action: MenuAction::Message {
                            is_selected: false,
                            on_press: Some(Action::Direct(Message::Noop)),
                        },
                    }),
                    menu::Item::Button(menu::button::Button {
                        icon: Some(Icon::Text {
                            text: EDIT.into_fragment(),
                            font: Some(fonts::icons_outlined()),
                        }),
                        label: "Item 3".into(),
                        supporting_text: None,
                        action: MenuAction::SubMenu {
                            is_opened: submenu_opened,
                            surface: Box::new(submenu),
                            on_press: Action::Direct(submenu_toggle),
                        },
                    }),
                    menu::Item::Separator,
                    menu::Item::Button(menu::button::Button {
                        icon: Some(Icon::Text {
                            text: STARS.into_fragment(),
                            font: Some(fonts::icons_filled()),
                        }),
                        label: "Item 4".into(),
                        supporting_text: None,
                        action: MenuAction::Message {
                            is_selected: true,
                            on_press: Some(Action::Direct(Message::Noop)),
                        },
                    }),
                ]),
                menu::Group {
                    label: Some("Label text".into()),
                    items: vec![
                        menu::Item::Button(menu::button::Button {
                            icon: None,
                            label: "Item 5".into(),
                            supporting_text: Some("Supporting text".into()),
                            action: MenuAction::Message {
                                is_selected: false,
                                on_press: Some(Action::Direct(Message::Noop)),
                            },
                        }),
                        menu::Item::Button(menu::button::Button {
                            icon: Some(Icon::Text {
                                text: EDIT.into_fragment(),
                                font: Some(fonts::icons_outlined()),
                            }),
                            label: "Item 6".into(),
                            supporting_text: Some("Supporting text".into()),
                            action: MenuAction::Message {
                                is_selected: false,
                                on_press: None,
                            },
                        }),
                        menu::Item::Button(menu::button::Button {
                            icon: Some(Icon::Svg(CHECK_CIRCLE_HANDLE.clone())),
                            label: "SVG icons!".into(),
                            supporting_text: None,
                            action: MenuAction::Message {
                                is_selected: false,
                                on_press: Some(Action::Direct(Message::Noop)),
                            },
                        }),
                    ],
                },
            ];

            iced_m3::widget::menu(style, groups).into()
        };

        let drop_down = advanced::menu_overlay(
            button(
                button::Style::outlined(&self.theme),
                button::Content::Icon(Icon::Text {
                    text: MORE_HORIZ.into_fragment(),
                    font: Some(icons_filled()),
                }),
            )
            .selected(self.drop_down_menu_opened)
            .on_press(Message::ToggleDropDownMenu),
            move || surface(self.sub_menu_2_opened, Message::ToggleSubMenu2),
            self.drop_down_menu_opened,
        )
        .on_press(|properties| {
            if let Some(position) = properties.position {
                properties
                    .bounds
                    .contains(position)
                    .not()
                    .then_some(Message::ToggleDropDownMenu)
            } else {
                None
            }
        });

        container(
            column![
                iced::widget::text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                row![
                    surface(self.sub_menu_1_opened, Message::ToggleSubMenu1),
                    drop_down
                ]
                .spacing(40),
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
            Message::Noop => {
                self.sub_menu_1_opened = false;
                self.sub_menu_2_opened = false;
                self.drop_down_menu_opened = false
            }
            Message::ToggleDropDownMenu => {
                self.sub_menu_1_opened = false;
                self.sub_menu_2_opened = false;
                self.drop_down_menu_opened = !self.drop_down_menu_opened
            }
            Message::ToggleSubMenu1 => self.sub_menu_1_opened = !self.sub_menu_1_opened,
            Message::ToggleSubMenu2 => self.sub_menu_2_opened = !self.sub_menu_2_opened,
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
