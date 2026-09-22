use std::sync::LazyLock;

use fonts::{icons_filled, icons_outlined, text_regular};
use iced::{
    Alignment, Element, Font, Length, Task, padding,
    widget::{column, container, row, text::IntoFragment},
};
use iced_m3::{
    theme::{Accent, ColorScheme, Mode, Theme},
    widget::{
        Badge, BadgeIcon, OnPress, button,
        navrail::{self, Fab, Item, ItemAlignment, Status},
        switch,
    },
};

const APP_NAME: &str = "Navigation Rail Demo";
static STAR: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8d0).unwrap());
static FAVORITE: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe87d).unwrap());
static SEARCH: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8b6).unwrap());
static SETTINGS: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8b8).unwrap());
static EDIT: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe3c9).unwrap());

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
    ToggleExpanded,
    SetAlignment(ItemAlignment),
    Noop,
    ToggleMenu,
    ToggleFAB,
}

struct State {
    theme: Theme,
    tab: Tab,
    expanded: bool,
    fab: bool,
    menu: bool,
    alignment: ItemAlignment,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            tab: Tab::default(),
            expanded: false,
            fab: true,
            menu: true,
            alignment: ItemAlignment::default(),
        }
    }
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        let items = vec![
            Item {
                icon: BadgeIcon {
                    icon: STAR.into_fragment(),
                    badge: None,
                },
                label: "Star".into_fragment(),
                on_press: OnPress::Direct(Message::GoTo(Tab::Star)),
            },
            Item {
                icon: BadgeIcon {
                    icon: FAVORITE.into_fragment(),
                    badge: None,
                },
                label: "Favorites".into_fragment(),
                on_press: OnPress::Direct(Message::GoTo(Tab::Favorites)),
            },
            Item {
                icon: BadgeIcon {
                    icon: SEARCH.into_fragment(),
                    badge: Some(Badge { label: None }),
                },
                label: "Search".into_fragment(),
                on_press: OnPress::Direct(Message::GoTo(Tab::Search)),
            },
            Item {
                icon: BadgeIcon {
                    icon: SETTINGS.into_fragment(),
                    badge: Some(Badge {
                        label: Some("3".into_fragment()),
                    }),
                },
                label: "Settings".into_fragment(),
                on_press: OnPress::Direct(Message::GoTo(Tab::Settings)),
            },
        ];
        let current_index = self.tab.index();
        let status = match self.expanded {
            true => Status::Expanded {
                width: iced::Pixels(navrail::CONTAINER_EXPANDED_MIN_WIDTH),
            },
            false => Status::Collapsed,
        };
        let fab = Fab {
            icon: EDIT.to_string().into_fragment(),
            label: "Edit".into_fragment(),
            style: iced_m3::widget::fab::Style::TonalPrimary,
            on_press: OnPress::Direct(Message::Noop),
        };

        let navrail = iced_m3::widget::navrail(&self.theme, items)
            .active(current_index)
            .label_font(text_regular())
            .icon_font(icons_filled())
            .icon_font_inactive(icons_outlined())
            .status(status)
            .fab_maybe(self.fab.then_some(fab))
            .item_alignment(self.alignment)
            .on_menu_pressed_maybe(self.menu.then_some(|_| Message::ToggleExpanded));

        let content = column![
            iced::widget::text("Alignment").color(self.theme.on_surface()),
            row![
                button(
                    button::Style::elevated(&self.theme, Accent::default()),
                    button::Content::Label("Top".into())
                )
                .on_press(Message::SetAlignment(ItemAlignment::Top)),
                button(
                    button::Style::elevated(&self.theme, Accent::default()),
                    button::Content::Label("Center".into())
                )
                .on_press(Message::SetAlignment(ItemAlignment::Center)),
                button(
                    button::Style::elevated(&self.theme, Accent::default()),
                    button::Content::Label("Bottom".into())
                )
                .on_press(Message::SetAlignment(ItemAlignment::Bottom)),
            ]
            .spacing(8),
            row![
                iced::widget::text("Menu").color(self.theme.on_surface()),
                switch(&self.theme, self.menu).on_toggle(Message::ToggleMenu)
            ]
            .align_y(Alignment::Center)
            .spacing(8),
            row![
                iced::widget::text("Floating Action Button").color(self.theme.on_surface()),
                switch(&self.theme, self.fab).on_toggle(Message::ToggleFAB)
            ]
            .align_y(Alignment::Center)
            .spacing(8),
            iced::widget::text(format!("Current tab: {}", self.tab)).color(self.theme.on_surface())
        ]
        .spacing(8);

        container(column![
            container(
                iced::widget::text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface())
            )
            .padding(10.0),
            row![
                container(navrail).style(|_| {
                    iced::widget::container::Style::default()
                        .background(self.theme.surface_container())
                }),
                container(content).padding(padding::horizontal(12))
            ],
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| iced::widget::container::Style::default().background(self.theme.background()))
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GoTo(tab) => self.tab = tab,
            Message::ToggleExpanded => self.expanded = !self.expanded,
            Message::Noop => {}
            Message::SetAlignment(item_alignment) => self.alignment = item_alignment,
            Message::ToggleMenu => self.menu = !self.menu,
            Message::ToggleFAB => self.fab = !self.fab,
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
