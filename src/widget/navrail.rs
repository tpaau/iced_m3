use iced::{Alignment, Element, Length, Padding, Pixels, advanced::text, padding};
use iced_widget::{column, container, row, space, text::LineHeight};

use crate::{
    style::Elevation,
    theme::ColorScheme,
    widget::{
        self, Icon, OnPress,
        button::{self},
    },
};

pub const CONTAINER_EXPANDED_MIN_WIDTH: f32 = 220.0;
pub const CONTAINER_EXPANDED_MAX_WIDTH: f32 = 360.0;
const CONTAINER_TOP_SPACE: f32 = 44.0;
const CONTAINER_COLLAPSED_WIDTH: f32 = 96.0;
const MENU_ICON_SIZE: f32 = 24.0;
const ITEM_ICON_SIZE: f32 = 24.0;
const INDICATOR_VERTICAL_HEIGHT: f32 = 32.0;
const INDICATOR_VERTICAL_WIDTH: f32 = 56.0;
const INDICATOR_VERTICAL_LABEL_SIZE: f32 = 12.0;
const INDICATOR_VERTICAL_ICON_LABEL_SPACE: f32 = 8.0;
const INDICATOR_HORIZONTAL_HEIGHT: f32 = 56.0;
const INDICATOR_HORIZONTAL_LABEL_SIZE: f32 = 14.0;
const INDICATOR_HORIZONTAL_ICON_LABEL_SPACE: f32 = 8.0;
const MENU_BUTTON_PADDING: f32 = 4.0;
const INDICATOR_CONTENT_PADDING: f32 = 16.0;
const COLLAPSED_ITEM_VERTICAL_SPACE: f32 = 16.0;
const MENU_FAB_SPACE: f32 = 16.0;
const FAB_ITEMS_SPACE: f32 = 32.0;

struct Menu<'a, Message> {
    icon: Box<dyn Fn(bool) -> text::Fragment<'a> + 'a>,
    on_press: Box<dyn Fn(bool) -> Message + 'a>,
}

pub struct Fab<'a, Message> {
    pub icon: text::Fragment<'a>,
    pub label: text::Fragment<'a>,
    pub style: crate::widget::fab::Style,
    pub on_press: OnPress<'a, Message>,
}

pub struct Item<'a, Message> {
    pub icon: Icon<'a>,
    pub label: text::Fragment<'a>,
    pub on_press: OnPress<'a, Message>,
}

#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum Status {
    #[default]
    Collapsed,
    Expanded {
        /// The width of the navigation rail container.
        ///
        ///This value will be clamped between the minimum and maximum container width.
        width: Pixels,
    },
}

impl Status {
    fn width(&self) -> Pixels {
        match self {
            Status::Collapsed => Pixels(CONTAINER_COLLAPSED_WIDTH),
            Status::Expanded { width } => Pixels(
                Into::<f32>::into(*width)
                    .clamp(CONTAINER_EXPANDED_MIN_WIDTH, CONTAINER_EXPANDED_MAX_WIDTH),
            ),
        }
    }

    fn expanded(&self) -> bool {
        matches!(self, Self::Expanded { width: _ })
    }
}

pub struct NavRail<'a, Message>
where
    Message: 'a + Clone,
{
    theme: &'a dyn ColorScheme,
    status: Status,
    menu: Option<Menu<'a, Message>>,
    fab: Option<Fab<'a, Message>>,
    items: Vec<Item<'a, Message>>,
    label_font: Option<iced::Font>,
    icon_font: Option<iced::Font>,
    icon_font_inactive: Option<iced::Font>,
    icon_font_active: Option<iced::Font>,
    active_index: usize,
}

impl<'a, Message> NavRail<'a, Message>
where
    Message: 'a + Clone,
{
    #[must_use]
    pub fn new(theme: &'a dyn ColorScheme, items: Vec<Item<'a, Message>>) -> Self {
        Self {
            theme,
            status: Status::default(),
            menu: None,
            fab: None,
            items,
            label_font: None,
            icon_font: None,
            icon_font_inactive: None,
            icon_font_active: None,
            active_index: 0,
        }
    }

    #[must_use]
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn status_maybe(self, maybe_status: Option<Status>) -> Self {
        match maybe_status {
            Some(status) => self.status(status),
            None => self,
        }
    }

    #[must_use]
    pub fn fab(mut self, fab: Fab<'a, Message>) -> Self {
        self.fab = Some(fab);
        self
    }

    #[must_use]
    pub fn fab_maybe(mut self, maybe_fab: Option<Fab<'a, Message>>) -> Self {
        self.fab = maybe_fab;
        self
    }

    #[must_use]
    pub fn menu(
        mut self,
        icon: &'a dyn Fn(bool) -> text::Fragment<'a>,
        on_press: &'a dyn Fn(bool) -> Message,
    ) -> Self {
        self.menu = Some(Menu {
            icon: Box::new(icon),
            on_press: Box::new(on_press),
        });
        self
    }

    #[must_use]
    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }

    #[must_use]
    pub fn label_font(mut self, font: iced::Font) -> Self {
        self.label_font = Some(font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, maybe_font: Option<iced::Font>) -> Self {
        self.label_font = maybe_font;
        self
    }

    /// Sets the icon font for the menu and FAB.
    #[must_use]
    pub fn icon_font(mut self, font: iced::Font) -> Self {
        self.icon_font = Some(font);
        self
    }

    /// Sets the icon font for the menu and FAB.
    #[must_use]
    pub fn icon_font_maybe(mut self, maybe_font: Option<iced::Font>) -> Self {
        self.icon_font = maybe_font;
        self
    }

    /// Sets the icon font for active item icons.
    #[must_use]
    pub fn icon_font_active(mut self, font: iced::Font) -> Self {
        self.icon_font_active = Some(font);
        self
    }

    /// Sets the icon font for active item icons.
    #[must_use]
    pub fn icon_font_active_maybe(mut self, maybe_font: Option<iced::Font>) -> Self {
        self.icon_font_active = maybe_font;
        self
    }

    /// Sets the icon font for inactive item icons.
    #[must_use]
    pub fn icon_font_inactive(mut self, font: iced::Font) -> Self {
        self.icon_font_inactive = Some(font);
        self
    }

    /// Sets the icon font for inactive item icons.
    #[must_use]
    pub fn icon_font_inactive_maybe(mut self, maybe_font: Option<iced::Font>) -> Self {
        self.icon_font_inactive = maybe_font;
        self
    }
}

fn item_widget<'a, Message>(
    theme: &'a dyn ColorScheme,
    active: bool,
    expanded: bool,
    label_font: Option<iced::Font>,
    icon_font_active: Option<iced::Font>,
    icon_font_inactive: Option<iced::Font>,
    item: Item<'a, Message>,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    let icon_font = match active {
        true => icon_font_active,
        false => icon_font_inactive,
    };
    let icon = crate::widget::icon(item.icon.icon, ITEM_ICON_SIZE).font_maybe(icon_font);
    let icon: Element<'_, Message> = match item.icon.badge {
        Some(badge) => crate::widget::badge(theme, icon)
            .label_maybe(badge.label)
            .into(),
        None => icon.into(),
    };
    let content = match expanded {
        true => {
            let label = iced_widget::text(item.label.clone())
                .font_maybe(label_font)
                .size(INDICATOR_HORIZONTAL_LABEL_SIZE);
            row![icon, label]
                .spacing(INDICATOR_HORIZONTAL_ICON_LABEL_SPACE)
                .align_y(Alignment::Center)
                .into()
        }
        false => icon,
    };

    let style = button::Style::Custom {
        surface: active.then_some(theme.secondary_container()),
        content: if active {
            theme.on_secondary_container()
        } else {
            theme.on_surface()
        },
        outline: None,
        surface_disabled: None,
        content_disabled: theme.on_surface(),
        outline_disabled: None,
    };
    let (width, height) = match expanded {
        true => (Length::Shrink, Length::Fixed(INDICATOR_HORIZONTAL_HEIGHT)),
        false => (
            Length::Fixed(INDICATOR_VERTICAL_WIDTH),
            Length::Fixed(INDICATOR_VERTICAL_HEIGHT),
        ),
    };
    let button = iced_widget::button(container(content).center_y(Length::Fill))
        .padding(padding::horizontal(INDICATOR_CONTENT_PADDING))
        .style(move |_, status| {
            crate::widget::button::style(
                status,
                None,
                Elevation::Level0,
                button::Size::Medium,
                style,
                button::CornerStyle::Custom {
                    resting: f32::MAX.into(),
                    pressed: f32::MAX.into(),
                },
                theme,
            )
        })
        .width(width)
        .height(height);

    let button: Element<'_, Message> = match item.on_press {
        OnPress::Direct(on_press) => button.on_press(on_press),
        OnPress::Closure(on_press) => button.on_press_with(on_press),
    }
    .into();

    let offset = (CONTAINER_COLLAPSED_WIDTH - INDICATOR_VERTICAL_WIDTH) / 2.0;
    let content = match expanded {
        true => button,
        false => column![
            button,
            iced_widget::text(item.label)
                .font_maybe(label_font)
                .wrapping(text::Wrapping::None)
                .size(INDICATOR_VERTICAL_LABEL_SIZE)
                .line_height(LineHeight::Absolute(Pixels(INDICATOR_VERTICAL_LABEL_SIZE)))
        ]
        .align_x(Alignment::Center)
        .spacing(INDICATOR_VERTICAL_ICON_LABEL_SPACE)
        .into(),
    };
    row!(space().width(offset), content).into()
}

impl<'a, Message> From<NavRail<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(value: NavRail<'a, Message>) -> Self {
        let expanded = value.status.expanded();
        let container_width = value.status.width();

        let menu = value.menu.map(|menu| {
            crate::widget::button(
                value.theme,
                widget::button::Content::Icon((menu.icon)(expanded)),
            )
            .icon_font_maybe(value.icon_font)
            .label_font_maybe(value.label_font)
            .style(button::Style::Custom {
                surface: None,
                content: value.theme.on_surface(),
                outline: None,
                surface_disabled: None,
                content_disabled: value.theme.on_surface(),
                outline_disabled: None,
            })
            .size(button::Size::Custom {
                width: Length::Shrink,
                height: Length::Shrink,
                spacing: 0.0,
                padding: Padding::from(MENU_BUTTON_PADDING),
                icon_size: MENU_ICON_SIZE,
                font_size: 0.0,
            })
            .corner_style(button::CornerStyle::Custom {
                resting: f32::MAX.into(),
                pressed: f32::MAX.into(),
            })
            .on_press((menu.on_press)(expanded))
        });
        let menu = menu.map(|menu| {
            let offset =
                (CONTAINER_COLLAPSED_WIDTH - MENU_ICON_SIZE - MENU_BUTTON_PADDING * 2.0) / 2.0;
            row![space().width(offset), menu]
        });

        let fab = value.fab.map(|fab| {
            let content = match expanded {
                true => widget::fab::Content::Extended {
                    icon: fab.icon,
                    label: fab.label,
                },
                false => widget::fab::Content::Reguar { icon: fab.icon },
            };

            widget::fab(value.theme, content, fab.on_press)
                .label_font_maybe(value.label_font)
                .icon_font_maybe(value.icon_font)
                .style(fab.style)
        });
        let fab = fab.map(|fab| {
            // Small FAB container size
            let offset = (CONTAINER_COLLAPSED_WIDTH - 56.0) / 2.0;
            row![space().width(offset), fab]
        });

        let active_index = value.active_index.min(value.items.len());
        let items = value.items.into_iter().enumerate().map(|(i, item)| {
            let active = i == active_index;
            item_widget(
                value.theme,
                active,
                expanded,
                value.label_font,
                value.icon_font_active,
                value.icon_font_inactive,
                item,
            )
        });
        let spacing = match expanded {
            true => 0.0,
            false => COLLAPSED_ITEM_VERTICAL_SPACE,
        };
        let item_column = column(items).spacing(spacing);

        let column = column![
            space().height(CONTAINER_TOP_SPACE),
            menu,
            space().height(MENU_FAB_SPACE),
            fab,
            space().height(FAB_ITEMS_SPACE),
            item_column
        ];

        container(column)
            .height(Length::Fill)
            .width(container_width)
            .into()
    }
}
