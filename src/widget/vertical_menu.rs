use std::sync::LazyLock;

use iced::{Alignment, Border, Color, Element, Font, Length, Pixels, border::Radius, padding};
use iced_widget::{button, column, container, row, space, svg, svg::Handle, text};

mod constants {
    pub const CONTAINER_BORDER_RADIUS: f32 = 16.0;
}

use crate::{
    style::{DIM_ALPHA, Elevation, StateLayer, shadow},
    theme::ColorScheme,
    widget::{common_icons::ARROW_RIGHT, hybrid_icon::Icon, spacer},
};

pub enum Action<'a, Message> {
    Menu(Vec<Group<'a, Message>>),
    Message(Option<Message>),
}

pub enum Entry<'a, Message> {
    Button {
        icon: Option<Icon<'a>>,
        label: text::Fragment<'a>,
        supporting_text: Option<text::Fragment<'a>>,
        error: bool,
        action: Action<'a, Message>,
    },
    Separator,
}

pub struct Group<'a, Message> {
    pub label: Option<text::Fragment<'a>>,
    pub entries: Vec<Entry<'a, Message>>,
}

impl<'a, Message> Group<'a, Message> {
    pub fn new(entries: Vec<Entry<'a, Message>>) -> Self {
        Self {
            label: None,
            entries,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub container_color: Color,
    pub container_border: Border,
    pub shadow_color: Color,
    pub elevation: Elevation,
    pub supporting_text_color: Color,
    pub label_color: Color,
    pub icon_color: Color,
    pub button_label_color: Color,
    pub state_layer: StateLayer,
    pub error_content_color: Color,
    pub error_state_layer: StateLayer,
    pub separator_color: Color,
}

impl Style {
    pub fn new(theme: &(impl ColorScheme + ?Sized), vibrant: bool) -> Self {
        match vibrant {
            true => Self {
                container_color: theme.tertiary_container(),
                container_border: Border::default().rounded(constants::CONTAINER_BORDER_RADIUS),
                shadow_color: theme.shadow(),
                elevation: Elevation::Level3,
                supporting_text_color: theme.on_tertiary_container(),
                label_color: theme.on_tertiary_container(),
                icon_color: theme.on_tertiary_container(),
                button_label_color: theme.on_tertiary_container(),
                state_layer: StateLayer::new(theme.on_tertiary_container()),
                error_content_color: theme.error(),
                error_state_layer: StateLayer::new(theme.on_error_container()),
                separator_color: theme.tertiary().scale_alpha(DIM_ALPHA),
            },
            false => Self {
                container_color: theme.surface_container_low(),
                container_border: Border::default().rounded(constants::CONTAINER_BORDER_RADIUS),
                shadow_color: theme.shadow(),
                elevation: Elevation::Level3,
                supporting_text_color: theme.on_surface_variant(),
                label_color: theme.on_surface_variant(),
                icon_color: theme.on_surface_variant(),
                button_label_color: theme.on_surface(),
                state_layer: StateLayer::new(theme.on_surface()),
                error_content_color: theme.error(),
                error_state_layer: StateLayer::new(theme.on_error_container()),
                separator_color: theme.outline_variant(),
            },
        }
    }
}

pub struct Menu<'a, Message> {
    groups: Vec<Group<'a, Message>>,
    // TODO: Grab the default font
    font: Font,
    width: Option<f32>,
    style: Style,
}

impl<'a, Message> Menu<'a, Message> {
    #[must_use]
    pub fn new(groups: Vec<Group<'a, Message>>, style: Style, font: Font) -> Self {
        Self {
            groups,
            font,
            width: Some(192.0),
            style,
        }
    }

    #[must_use]
    pub fn width(mut self, width: Option<f32>) -> Self {
        self.width = width;
        self
    }
}

const SECTION_PADDING: f32 = 4.0;
const SPACING: f32 = 2.0;
const LABEL_HEIGHT: Length = Length::Fixed(32.0);
const BUTTON_HEIGHT: Length = Length::Fixed(48.0);
const LABEL_TEXT_SIZE: Pixels = Pixels(16.0);
const SUPPORTING_TEXT_SIZE: Pixels = Pixels(12.0);
const ICON_SIZE: Pixels = Pixels(20.0);
static BUTTON_RADIUS: LazyLock<Radius> = LazyLock::new(|| Radius::new(12.0));

impl<'a, Message> From<Menu<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(menu: Menu<'a, Message>) -> Self {
        let trailing_icon = LazyLock::new(|| Handle::from_memory(ARROW_RIGHT));
        let container_width = match menu.width {
            Some(pixels) => Length::Fixed(pixels),
            None => Length::Shrink,
        };

        let children: Vec<Element<'_, Message>> = menu
            .groups
            .into_iter()
            .map(|group| -> Element<'_, Message> {
                let mut children: Vec<Element<'a, Message>> = Vec::new();

                group.label.map(|label| {
                    children.push(
                        container(
                            text(label)
                                .size(LABEL_TEXT_SIZE)
                                .font(menu.font)
                                .height(Length::Fill)
                                .center()
                                .style(move |_| iced_widget::text::Style {
                                    color: Some(menu.style.label_color),
                                }),
                        )
                        .padding(12.0)
                        .height(LABEL_HEIGHT)
                        .width(container_width)
                        .into(),
                    )
                });

                for entry in group.entries {
                    match entry {
                        Entry::Button {
                            icon,
                            label,
                            supporting_text,
                            error,
                            action,
                        } => {
                            let button_disabled = match &action {
                                Action::Menu(groups) => groups.is_empty(),
                                Action::Message(message) => message.is_none(),
                            };
                            let content_alpha = if button_disabled { DIM_ALPHA } else { 1.0 };
                            let trailing_icon =
                                matches!(action, Action::Menu(_)).then_some(trailing_icon.clone());
                            let icon_color = if error {
                                menu.style.error_content_color.scale_alpha(content_alpha)
                            } else {
                                menu.style.icon_color.scale_alpha(content_alpha)
                            };
                            let label_color = if error {
                                menu.style.error_content_color.scale_alpha(content_alpha)
                            } else {
                                menu.style.button_label_color.scale_alpha(content_alpha)
                            };
                            let supporting_text_color = if error {
                                menu.style.error_content_color.scale_alpha(content_alpha)
                            } else {
                                menu.style.supporting_text_color.scale_alpha(content_alpha)
                            };
                            let content = move || -> Element<'a, Message> {
                                row![
                                    icon.clone().map(|i| {
                                        crate::widget::hybrid_icon(i, ICON_SIZE, icon_color)
                                    }),
                                    column![
                                        space().height(Length::Fill),
                                        text(label.clone())
                                            .size(LABEL_TEXT_SIZE)
                                            .font(menu.font)
                                            .style(move |_| text::Style {
                                                color: Some(label_color)
                                            }),
                                        supporting_text.clone().map(|t| {
                                            text(t)
                                                .size(SUPPORTING_TEXT_SIZE)
                                                .font(menu.font)
                                                .style(move |_| text::Style {
                                                    color: Some(supporting_text_color),
                                                })
                                        }),
                                        space().height(Length::Fill),
                                    ],
                                    space().width(Length::Fill),
                                    trailing_icon.clone().map(|icon| {
                                        svg(icon).width(ICON_SIZE).height(ICON_SIZE).style(
                                            move |_, _| svg::Style {
                                                color: Some(icon_color),
                                            },
                                        )
                                    }),
                                ]
                                .spacing(8.0)
                                .align_y(Alignment::Center)
                                .into()
                            };

                            match action {
                                Action::Menu(groups) => {
                                    let submenu = super::advanced::drop_down_menu(
                                        move |_| container(content()).height(BUTTON_HEIGHT).into(),
                                        if groups.is_empty() {
                                            None
                                        } else {
                                            Some(Menu::new(groups, menu.style, menu.font))
                                        },
                                        super::advanced::drop_down_menu::Placement::RightBottom,
                                    )
                                    .trigger_transparent(true)
                                    .into();

                                    children.push(submenu)
                                }
                                Action::Message(message) => {
                                    let entry = button(content())
                                        .style(move |_, status| {
                                            let state_layer = match error {
                                                true => menu.style.error_state_layer,
                                                false => menu.style.state_layer,
                                            };
                                            button::Style {
                                                background: Some(iced::Background::Color(
                                                    match status {
                                                        button::Status::Active
                                                        | button::Status::Disabled => {
                                                            Color::TRANSPARENT
                                                        }
                                                        button::Status::Hovered => {
                                                            state_layer.hovered
                                                        }
                                                        button::Status::Pressed => {
                                                            state_layer.pressed
                                                        }
                                                    },
                                                )),
                                                border: Border::default().rounded(*BUTTON_RADIUS),
                                                ..Default::default()
                                            }
                                        })
                                        .on_press_maybe(message)
                                        .height(BUTTON_HEIGHT)
                                        .into();

                                    children.push(entry)
                                }
                            }
                        }
                        Entry::Separator => {
                            let separator = container(spacer(menu.style.separator_color))
                                .width(Length::Fill)
                                .padding(padding::horizontal(8.0).vertical(2.0))
                                .into();

                            children.push(separator);
                        }
                    }
                }

                container(column(children).spacing(2.0))
                    .style(move |_| container::Style {
                        background: Some(iced::Background::Color(menu.style.container_color)),
                        border: iced::Border {
                            radius: Radius::new(16),
                            ..Default::default()
                        },
                        shadow: shadow(menu.style.shadow_color, menu.style.elevation),
                        ..Default::default()
                    })
                    .padding(SECTION_PADDING)
                    .width(if let Length::Fixed(val) = container_width {
                        Length::Fixed(val - 2.0 * SECTION_PADDING)
                    } else {
                        container_width
                    })
                    .into()
            })
            .collect();

        column(children).spacing(SPACING).into()
    }
}
