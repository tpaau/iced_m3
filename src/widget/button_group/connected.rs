//! Connected button groups help people select options, switch views, or sort elements in a page.

use iced::{Element, Font, Length, Padding, Pixels, border::Radius, padding};

use crate::{
    animation::motion::{self, Spring},
    widget::{
        button::{self, Content, CornerRadius, CornerStyle, Style},
        button_group::standard::{self, GroupItem},
    },
};

#[derive(Debug, Clone)]
pub struct Button<'a, Message>
where
    Message: Clone,
{
    pub content: Content<'a>,
    pub on_press: Option<Message>,
    pub selected: bool,
}

/// The size of elements in the connected button group.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub container_height: Pixels,
    pub button_spacing: Pixels,
    pub outer_corner_size: Pixels,
    pub inner_corner_size: Pixels,
    pub pressed_corner_size: Pixels,
    pub icon_size: Pixels,
    pub label_size: Pixels,
    pub content_spacing: Pixels,
    pub content_padding: Padding,
}

impl Default for Size {
    fn default() -> Self {
        Self::small()
    }
}

impl Size {
    pub fn extra_small() -> Self {
        Self {
            container_height: Pixels(32.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(8.0),
            pressed_corner_size: Pixels(4.0),
            icon_size: button::constants::ICON_SIZE_EXTRA_SMALL,
            label_size: button::constants::LABEL_SIZE_EXTRA_SMALL,
            content_spacing: button::constants::CONTENT_SPACING_EXTRA_SMALL,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_EXTRA_SMALL),
        }
    }

    pub fn small() -> Self {
        Self {
            container_height: Pixels(40.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(8.0),
            pressed_corner_size: Pixels(4.0),
            icon_size: button::constants::ICON_SIZE_SMALL,
            label_size: button::constants::LABEL_SIZE_SMALL,
            content_spacing: button::constants::CONTENT_SPACING_SMALL,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_SMALL),
        }
    }

    pub fn medium() -> Self {
        Self {
            container_height: Pixels(56.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(8.0),
            pressed_corner_size: Pixels(4.0),
            icon_size: button::constants::ICON_SIZE_MEDIUM,
            label_size: button::constants::LABEL_SIZE_MEDIUM,
            content_spacing: button::constants::CONTENT_SPACING_MEDIUM,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_MEDIUM),
        }
    }

    pub fn large() -> Self {
        Self {
            container_height: Pixels(96.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(16.0),
            pressed_corner_size: Pixels(12.0),
            icon_size: button::constants::ICON_SIZE_LARGE,
            label_size: button::constants::LABEL_SIZE_LARGE,
            content_spacing: button::constants::CONTENT_SPACING_LARGE,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_LARGE),
        }
    }

    pub fn extra_large() -> Self {
        Self {
            container_height: Pixels(136.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(20.0),
            pressed_corner_size: Pixels(16.0),
            icon_size: button::constants::ICON_SIZE_EXTRA_LARGE,
            label_size: button::constants::LABEL_SIZE_EXTRA_LARGE,
            content_spacing: button::constants::CONTENT_SPACING_EXTRA_LARGE,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_EXTRA_LARGE),
        }
    }
}

/// Connected button groups help people select options, switch views, or sort elements in a page.
pub struct ButtonGroup<'a, Message>
where
    Message: Clone,
{
    model: Vec<Button<'a, Message>>,
    style: Style,
    size: Size,
    label_font: Option<Font>,
    motion_scheme: Option<motion::Scheme>,
    style_spring: Option<Spring>,
    state_layer_spring: Option<Spring>,
    corner_radius_spring: Option<Spring>,
    button_expansion_spring: Option<Spring>,
    expanded_ratio: f32,
}

impl<'a, Message> ButtonGroup<'a, Message>
where
    Message: Clone,
{
    #[must_use]
    pub fn new(style: Style, model: Vec<Button<'a, Message>>) -> Self {
        Self {
            model,
            style,
            size: Size::default(),
            label_font: None,
            motion_scheme: None,
            style_spring: None,
            state_layer_spring: None,
            corner_radius_spring: None,
            button_expansion_spring: None,
            expanded_ratio: 0.0,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn button(mut self, button: Button<'a, Message>) -> Self {
        self.model.push(button);
        self
    }

    #[must_use]
    pub fn button_maybe(mut self, button: Option<Button<'a, Message>>) -> Self {
        if let Some(button) = button {
            self.model.push(button);
        }
        self
    }

    #[must_use]
    pub fn label_font(mut self, font: Font) -> Self {
        self.label_font = Some(font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, font: Option<Font>) -> Self {
        self.label_font = font;
        self
    }

    #[must_use]
    pub fn motion_scheme(mut self, scheme: motion::Scheme) -> Self {
        self.motion_scheme = Some(scheme);
        self
    }

    #[must_use]
    pub fn motion_scheme_maybe(mut self, scheme: Option<motion::Scheme>) -> Self {
        self.motion_scheme = scheme;
        self
    }

    #[must_use]
    pub fn style_spring(mut self, spring: Spring) -> Self {
        self.style_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn style_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.style_spring = spring;
        self
    }

    #[must_use]
    pub fn state_layer_spring(mut self, spring: Spring) -> Self {
        self.state_layer_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn state_layer_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.state_layer_spring = spring;
        self
    }

    #[must_use]
    pub fn corner_radius_spring(mut self, spring: Spring) -> Self {
        self.corner_radius_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn corner_radius_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.corner_radius_spring = spring;
        self
    }

    #[must_use]
    pub fn button_expansion_spring(mut self, spring: Spring) -> Self {
        self.button_expansion_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn button_expansion_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.button_expansion_spring = spring;
        self
    }

    #[must_use]
    pub fn button_expanded_ratio(mut self, ratio: f32) -> Self {
        self.expanded_ratio = ratio;
        self
    }
}

impl<'a, Message, Theme, Renderer> From<ButtonGroup<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn from(value: ButtonGroup<'a, Message>) -> Self {
        let buttons = value
            .model
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, model)| {
                let left_radius = if index == 0 {
                    value.size.outer_corner_size.0
                } else {
                    value.size.inner_corner_size.0
                };
                let right_radius = if index == value.model.len() - 1 {
                    value.size.outer_corner_size.0
                } else {
                    value.size.inner_corner_size.0
                };
                let corner_radius = CornerRadius {
                    style: CornerStyle::Rounded,
                    shape_morph: true,
                    rounded: Radius {
                        top_left: left_radius,
                        top_right: right_radius,
                        bottom_right: right_radius,
                        bottom_left: left_radius,
                    },
                    square: value.size.outer_corner_size.0.into(),
                    pressed: value.size.pressed_corner_size.0.into(),
                };

                let size = button::Size {
                    width: Length::Shrink,
                    height: value.size.container_height.into(),
                    spacing: value.size.content_spacing.into(),
                    padding: value.size.content_padding,
                    icon_size: value.size.icon_size,
                    label_size: value.size.label_size,
                    corner_radius,
                };

                crate::widget::button(value.style, model.content)
                    .selected(model.selected)
                    .label_font_maybe(value.label_font)
                    .on_press_maybe(model.on_press)
                    .motion_scheme_maybe(value.motion_scheme)
                    .style_spring_maybe(value.style_spring)
                    .state_layer_spring_maybe(value.state_layer_spring)
                    .corner_radius_spring_maybe(value.corner_radius_spring)
                    .size(size)
            });

        let items = buttons.into_iter().map(|b| {
            let is_disabled = b.is_disabled();
            GroupItem::new(b, is_disabled)
        });
        let button_group = standard::ButtonGroup::new(items)
            .motion_scheme_maybe(value.motion_scheme)
            .expansion_spring_maybe(value.button_expansion_spring)
            .expanded_ratio(value.expanded_ratio)
            .spacing(value.size.button_spacing);

        Element::new(button_group)
    }
}
