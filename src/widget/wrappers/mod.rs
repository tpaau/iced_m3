//! Small wrapper widgets that can easily be expressed using a single function.

use iced::{Length, Pixels};
use iced_widget::{
    Text, text,
    text::{IntoFragment, LineHeight},
};

/// Wrapper around the [`Text`](iced_widget::Text) widget, with some default presets for an icon.
///
/// It ensures that wrapping is disabled and that the icon will be `size` in width and height.
pub fn icon<'a, P>(content: impl IntoFragment<'a>, size: P) -> Text<'a>
where
    P: Into<Pixels> + Copy,
{
    text(content)
        .size(size)
        .width(Length::Fixed(size.into().into()))
        .wrapping(text::Wrapping::None)
        .line_height(LineHeight::Absolute(size.into()))
}
