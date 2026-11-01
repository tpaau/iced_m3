pub mod drop_down_menu;
#[cfg(feature = "advanced")]
pub mod menu_overlay;

use iced::Element;

#[cfg(feature = "advanced")]
use crate::widget::advanced::menu_overlay::MenuOverlay;

#[must_use]
pub fn drop_down_menu<'a, Message, Theme, Renderer>(
    trigger: impl Fn(bool) -> Element<'a, Message, Theme, Renderer> + 'a,
    menu: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
    placement: drop_down_menu::Placement,
) -> drop_down_menu::DropDownMenu<'a, Message, Theme, Renderer> {
    drop_down_menu::DropDownMenu::<'a, Message, Theme, Renderer>::new(trigger, menu, placement)
}

#[cfg(feature = "advanced")]
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
