use iced::{
    Alignment, Element, Length, Padding, Pixels,
    advanced::{svg, text},
    padding,
};
use iced_widget::{column, container, row, space, text::LineHeight};

use crate::{
    style::Elevation,
    theme::ColorScheme,
    widget::{
        self, Icon, OnPress,
        button::{self},
        common_icons::{MENU, MENU_OPEN},
    },
};

const CONTAINER_VERTICAL_PADDING: f32 = 44.0;
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
const SECTION_SPACE: f32 = 16.0;
// Small FAB container size
const FAB_OFFSET: f32 = (CONTAINER_COLLAPSED_WIDTH - 56.0) / 2.0;
const MENU_OFFSET: f32 =
    (CONTAINER_COLLAPSED_WIDTH - MENU_ICON_SIZE - MENU_BUTTON_PADDING * 2.0) / 2.0;

mod constants {
    use crate::widget::navrail::{CONTAINER_COLLAPSED_WIDTH, INDICATOR_VERTICAL_WIDTH};

    pub const CONTAINER_EXPANDED_MIN_WIDTH: f32 = 220.0;
    pub const CONTAINER_EXPANDED_MAX_WIDTH: f32 = 360.0;
    pub const ITEM_OFFSET: f32 = (CONTAINER_COLLAPSED_WIDTH - INDICATOR_VERTICAL_WIDTH) / 2.0;
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemAlignment {
    #[default]
    Top,
    Center,
    Bottom,
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
            Status::Expanded { width } => Pixels(Into::<f32>::into(*width).clamp(
                constants::CONTAINER_EXPANDED_MIN_WIDTH,
                constants::CONTAINER_EXPANDED_MAX_WIDTH,
            )),
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
    on_menu_pressed: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    fab: Option<Fab<'a, Message>>,
    items: Vec<Item<'a, Message>>,
    label_font: Option<iced::Font>,
    icon_font: Option<iced::Font>,
    icon_font_inactive: Option<iced::Font>,
    icon_font_active: Option<iced::Font>,
    active_index: usize,
    item_alignment: ItemAlignment,
    container_vertical_padding: Option<f32>,
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
            on_menu_pressed: None,
            fab: None,
            items,
            label_font: None,
            icon_font: None,
            icon_font_inactive: None,
            icon_font_active: None,
            active_index: 0,
            item_alignment: ItemAlignment::default(),
            container_vertical_padding: None,
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
    pub fn on_menu_pressed(mut self, on_press: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_menu_pressed = Some(Box::new(on_press));
        self
    }

    #[must_use]
    pub fn on_menu_pressed_maybe(
        mut self,
        on_press: Option<impl Fn(bool) -> Message + 'a>,
    ) -> Self {
        self.on_menu_pressed =
            on_press.map(|callback| Box::new(callback) as Box<dyn Fn(bool) -> Message + 'a>);
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

    #[must_use]
    pub fn item_alignment(mut self, alignment: ItemAlignment) -> Self {
        self.item_alignment = alignment;
        self
    }

    #[must_use]
    pub fn item_alignment_maybe(self, maybe_alignment: Option<ItemAlignment>) -> Self {
        match maybe_alignment {
            Some(alignment) => self.item_alignment(alignment),
            None => self,
        }
    }

    #[must_use]
    pub fn container_vertical_padding(mut self, padding: f32) -> Self {
        self.container_vertical_padding = Some(padding);
        self
    }

    #[must_use]
    pub fn container_vertical_padding_maybe(mut self, maybe_padding: Option<f32>) -> Self {
        self.container_vertical_padding = maybe_padding;
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
    row!(space().width(constants::ITEM_OFFSET), content).into()
}

impl<'a, Message> From<NavRail<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(value: NavRail<'a, Message>) -> Self {
        let expanded = value.status.expanded();
        let container_width = value.status.width();

        let menu = value.on_menu_pressed.map(|on_press| {
            let handle = match expanded {
                true => svg::Handle::from_memory(MENU_OPEN),
                false => svg::Handle::from_memory(MENU),
            };
            let icon = iced_widget::svg(handle)
                .height(MENU_ICON_SIZE)
                .width(MENU_ICON_SIZE)
                .style(|_, _| iced_widget::svg::Style {
                    color: Some(value.theme.on_surface()),
                });
            let style = button::Style::Custom {
                surface: None,
                content: value.theme.on_surface(),
                outline: None,
                surface_disabled: None,
                content_disabled: value.theme.on_surface(),
                outline_disabled: None,
            };
            let corner_style = button::CornerStyle::Custom {
                resting: f32::MAX.into(),
                pressed: f32::MAX.into(),
            };
            let size = button::Size::Custom {
                width: Length::Shrink,
                height: Length::Shrink,
                spacing: 0.0,
                padding: Padding::from(MENU_BUTTON_PADDING),
                icon_size: MENU_ICON_SIZE,
                font_size: 0.0,
            };

            iced_widget::button(icon)
                .padding(Padding::from(MENU_BUTTON_PADDING))
                .style(move |_, status| {
                    crate::widget::button::style(
                        status,
                        None,
                        Elevation::default(),
                        size,
                        style,
                        corner_style,
                        value.theme,
                    )
                })
                .on_press((on_press)(expanded))
        });
        let menu = menu.map(|menu| {
            column![
                row![space().width(MENU_OFFSET), menu],
                space().height(SECTION_SPACE)
            ]
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
            column![
                row![space().width(FAB_OFFSET), fab],
                (value.item_alignment != ItemAlignment::Center)
                    .then_some(space().height(SECTION_SPACE))
            ]
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

        let vertical_padding = value
            .container_vertical_padding
            .unwrap_or(CONTAINER_VERTICAL_PADDING);
        let column = column![
            space().height(vertical_padding),
            menu,
            fab,
            (value.item_alignment == ItemAlignment::Bottom
                || value.item_alignment == ItemAlignment::Center)
                .then_some(space().height(Length::Fill)),
            item_column,
            (value.item_alignment == ItemAlignment::Top
                || value.item_alignment == ItemAlignment::Center)
                .then_some(space().height(Length::Fill)),
            (value.item_alignment == ItemAlignment::Center)
                .then_some(space().height(vertical_padding)),
        ];

        container(column)
            .height(Length::Fill)
            .width(container_width)
            .into()
    }
}
