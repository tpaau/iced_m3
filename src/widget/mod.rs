#[cfg(feature = "advanced")]
pub mod advanced;
#[cfg(not(feature = "advanced"))]
pub(crate) mod advanced;
pub mod badge;
pub mod button;
pub mod card;
mod common_icons;
mod common_types;
pub mod dialog;
pub mod fab;
pub mod fab_menu;
pub mod hybrid_icon;
pub mod navbar;
pub mod navrail;
pub mod progress_bar;
pub mod slider;
pub mod switch;
pub mod text_input;
pub mod vertical_menu;
mod wrappers;

pub use common_types::*;
pub use wrappers::*;

use std::ops::RangeInclusive;

use iced::{Color, Element, Font, Pixels};

use crate::{
    theme::{Accent, ColorScheme},
    widget::{
        button::Button,
        card::Card,
        dialog::Dialog,
        fab::Fab,
        fab_menu::FABMenu,
        hybrid_icon::{HybridIcon, Icon},
        navbar::Navbar,
        navrail::NavRail,
        progress_bar::ProgressBar,
        switch::Switch,
        text_input::TextInput,
    },
};

#[must_use]
pub fn menu<'a, Message>(
    sections: Vec<vertical_menu::Group<'a, Message>>,
    theme: &'a dyn ColorScheme,
) -> vertical_menu::Menu<'a, Message> {
    vertical_menu::Menu::new(sections, theme)
}

#[must_use]
pub fn text_input<'a, Message: Clone, Renderer>(
    placeholder: &str,
    value: &'a str,
    theme: &'a impl ColorScheme,
) -> TextInput<'a, Message> {
    TextInput::new(placeholder, value, theme)
}

#[must_use]
pub fn navbar<'a, Message, Theme, Renderer>(
    items: Vec<navbar::Item<'a, Message>>,
    label_font: Font,
    theme: &'a impl ColorScheme,
) -> Navbar<'a, Message> {
    Navbar::new(items, label_font, theme)
}

#[must_use]
pub fn button<'a, Message, Renderer>(
    style: button::Style,
    content: button::Content<'a>,
) -> Button<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    Button::new(style, content)
}

// impl<'a, Message> FABMenu<'a, Message>
// where
//     Message: 'a + Clone,
// {
//     #[must_use]
//     pub fn new<I>(
//         items: I,
//         size: fab::Size,
//         label_font: Option<Font>,
//         theme: &(impl ColorScheme + ?Sized),
//         accent: Accent,
//     ) -> Self
//     where
//         I: IntoIterator<Item = Item<'a, Message>>,
//     {

#[must_use]
pub fn fab_menu<'a, Message, I>(
    items: I,
    size: fab::Size,
    label_font: Option<Font>,
    theme: &(impl ColorScheme + ?Sized),
    accent: Accent,
) -> FABMenu<'a, Message>
where
    Message: 'a + Clone,
    I: IntoIterator<Item = fab_menu::Item<'a, Message>>,
{
    FABMenu::new(items, size, label_font, theme, accent)
}

// #[must_use]
// pub fn fab_menu<'a, Message, I>(
//     entries: I,
//     icon: &'a dyn Fn(bool) -> Icon<'a>,
//     theme: &impl ColorScheme,
//     accent: Accent,
// ) -> modname::FABMenu<'a, Message>
// where
//     I: IntoIterator<Item = fab_menu::Entry<'a, Message>>,
// {
//     modname::FABMenu::new(entries, icon, accent, theme)
// }

#[must_use]
pub fn slider<'a, F, T, Message>(
    range: RangeInclusive<T>,
    value: T,
    on_change: F,
    style: slider::Style,
) -> slider::Slider<'a, T, Message>
where
    F: 'a + Fn(T) -> Message,
    T: 'a + Copy + From<u8> + PartialOrd + num_traits::cast::FromPrimitive,
    Message: 'a + Clone,
    f64: std::convert::From<T>,
{
    slider::Slider::new(range, value, on_change, style)
}

#[must_use]
pub fn dialog<'a, Message>(
    theme: &'a impl ColorScheme,
    body: impl Into<Element<'a, Message>>,
    buttons: Vec<dialog::Button<Message>>,
) -> Dialog<'a, Message> {
    Dialog::new(theme, body, buttons)
}

#[must_use]
pub fn progress_bar(style: progress_bar::Style) -> ProgressBar {
    ProgressBar::new(style)
}

#[must_use]
pub fn fab<'a, Message>(
    style: fab::Style,
    content: fab::Content<'a>,
    on_press: OnPress<'a, Message>,
) -> Fab<'a, Message>
where
    Message: 'a + Clone,
{
    Fab::new(style, content, on_press)
}

#[must_use]
pub fn badge<'a, Message, Theme, Renderer>(
    theme: &'a dyn ColorScheme,
    base: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> badge::Badge<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced_widget::core::text::Renderer,
{
    badge::Badge::new(theme, base)
}

#[must_use]
pub fn navrail<'a, Message>(
    theme: &'a dyn ColorScheme,
    items: Vec<navrail::Item<'a, Message>>,
) -> NavRail<'a, Message>
where
    Message: 'a + Clone,
{
    NavRail::new(theme, items)
}

#[must_use]
pub fn switch<Message>(theme: &(impl ColorScheme + ?Sized), toggled: bool) -> Switch<Message>
where
    Message: Clone,
{
    Switch::new(theme, toggled)
}

#[must_use]
pub fn card<'a, Message, Theme, Renderer>(
    style: card::Style,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Card<'a, Message, Theme, Renderer>
where
    Message: Clone,
{
    Card::new(style, content)
}

#[must_use]
pub fn hybrid_icon<'a>(icon: Icon<'a>, size: impl Into<Pixels>, color: Color) -> HybridIcon<'a> {
    HybridIcon::new(icon, size, color)
}
