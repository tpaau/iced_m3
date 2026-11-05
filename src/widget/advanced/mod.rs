pub mod menu_overlay;

use iced::Element;

use crate::widget::advanced::menu_overlay::MenuOverlay;

/// Creates a new [`MenuOverlay`] widget.
#[must_use]
pub fn menu_overlay<'a, Message, Theme, Renderer>(
    anchor: impl Into<Element<'a, Message, Theme, Renderer>>,
    surface: impl Fn() -> Element<'a, Message, Theme, Renderer> + 'a,
    opened: bool,
) -> MenuOverlay<'a, Message, Theme, Renderer>
where
    Message: Clone,
{
    MenuOverlay::new(anchor, surface, opened)
}
