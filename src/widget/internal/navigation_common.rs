use iced::{Color, advanced::text};

use crate::{
    animation::{Interpolable, motion::SpringValue},
    style::StateLayer,
    theme::ColorScheme,
    widget::{self, Badge, hybrid_icon::Icon},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationItemStyle {
    pub content_active: Color,
    pub content_inactive: Color,
    pub container_color: Color,
    pub badge_style: widget::badge::Style,
    pub state_layer: StateLayer,
}

impl NavigationItemStyle {
    pub fn new(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            container_color: theme.secondary_container(),
            content_active: theme.secondary(),
            content_inactive: theme.on_surface_variant(),
            badge_style: widget::badge::Style::new(theme),
            state_layer: StateLayer::new(theme.on_secondary_container()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NavigationItemContent<'a, Message>
where
    Message: Clone,
{
    pub icon_active: Icon<'a>,
    pub icon_inactive: Icon<'a>,
    pub badge: Option<Badge<'a>>,
    pub label: text::Fragment<'a>,
    pub on_press: Message,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Colors {
    pub state_layer: Color,
    pub content: Color,
}

impl Interpolable for Colors {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Self {
            state_layer: self.state_layer.interpolate(other.state_layer, t),
            content: self.content.interpolate(other.content, t),
        }
    }
}

pub struct State {
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub active_transition_spring: SpringValue<f32>,
    pub color_spring: SpringValue<Colors>,
}
