//! Small wrapper widgets that can easily be expressed using a single function.

use iced::{Color, Element, Length, Pixels, border::Radius};
use iced_widget::{
    Text, rule,
    text::{self, IntoFragment, LineHeight},
};

/// Wrapper around the [`Text`](iced_widget::Text) widget, with some default presets for an icon.
///
/// It ensures that wrapping is disabled and that the icon will be `size` in width and height.
pub fn icon<'a, P>(content: impl IntoFragment<'a>, size: P) -> Text<'a>
where
    P: Into<Pixels> + Copy,
{
    iced_widget::text(content)
        .size(size)
        .width(Length::Fixed(size.into().into()))
        .wrapping(text::Wrapping::None)
        .line_height(LineHeight::Absolute(size.into()))
}

#[must_use]
pub fn spacer<'a, Message>(color: Color) -> Element<'a, Message>
where
    Message: 'a,
{
    rule::horizontal(1.0)
        .style(move |_| rule::Style {
            color,
            radius: Radius::default(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        })
        .into()
}
