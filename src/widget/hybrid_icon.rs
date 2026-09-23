use iced::{
    Color, Element, Font, Pixels,
    advanced::{svg, text},
};

use crate::widget::icon;

/// Either a text or SVG icon to be displayed by the [`HybridIcon`] widget.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Icon<'a> {
    /// A text icon.
    Text {
        /// The text that will be used as the icon.
        text: text::Fragment<'a>,
        /// Font this icon should be displayed with.
        font: Option<Font>,
    },
    /// An SVG icon.
    Svg(svg::Handle),
}

/// An icon widget that can either display a text icon or an SVG icon.
pub struct HybridIcon<'a> {
    icon: Icon<'a>,
    color: Color,
    size: Pixels,
}

impl<'a> HybridIcon<'a> {
    #[must_use]
    pub fn new(icon: Icon<'a>, size: impl Into<Pixels>, color: Color) -> Self {
        Self {
            icon,
            color,
            size: size.into(),
        }
    }
}

impl<'a, Message> From<HybridIcon<'a>> for Element<'a, Message> {
    fn from(value: HybridIcon<'a>) -> Self {
        match value.icon {
            Icon::Text { text, font } => icon(text, value.size)
                .color(value.color)
                .font_maybe(font)
                .into(),
            Icon::Svg(svg) => iced_widget::svg(svg)
                .style(move |_, _| iced_widget::svg::Style {
                    color: Some(value.color),
                })
                .width(value.size)
                .height(value.size)
                .into(),
        }
    }
}
