use iced::{Alignment, Border, Element, Font, Length};
use iced_widget::{center, column, space};

use crate::{
    style::{Elevation, shadow},
    theme::{Accent, ColorScheme},
    widget::{self, button, hybrid_icon::Icon},
};

pub use crate::widget::advanced::drop_down_menu::Placement;

const TRIGGER_SIZE: f32 = 56.0;
const TRIGGER_ICON_SIZE: f32 = 20.0;
const BUTTON_TRIGGER_BETWEEN_SPACE: f32 = 8.0;
const BUTTON_SPACING: f32 = 4.0;

fn trigger_style(style: button::Style, opened: bool) -> iced_widget::container::Style {
    let state_style = style.state_style(false, Some(opened));

    iced_widget::container::Style {
        text_color: Some(state_style.icon),
        background: Some(iced::Background::Color(state_style.container)),
        border: Border::default().rounded(match opened {
            true => f32::MAX,
            false => 16.0,
        }),
        shadow: shadow(style.elevation.shadow_color, Elevation::Level3),
        ..Default::default()
    }
}

pub struct Entry<'a, Message> {
    pub message: Message,
    pub label: &'a str,
    pub icon: Option<Icon<'a>>,
}

pub struct FABMenu<'a, Message> {
    label_font: Option<Font>,
    style: button::Style,
    entries: Vec<Entry<'a, Message>>,
    icon: &'a dyn Fn(bool) -> Icon<'a>,
}

impl<'a, Message> FABMenu<'a, Message> {
    pub fn new<I>(
        entries: I,
        icon: &'a dyn Fn(bool) -> Icon<'a>,
        accent: Accent,
        theme: &impl ColorScheme,
    ) -> Self
    where
        I: IntoIterator<Item = Entry<'a, Message>>,
    {
        Self {
            label_font: None,
            style: button::Style::tonal(theme, accent),
            entries: entries.into_iter().collect(),
            icon,
        }
    }

    #[must_use]
    pub fn label_font(mut self, font: Font) -> Self {
        self.label_font = Some(font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, maybe_font: Option<Font>) -> Self {
        self.label_font = maybe_font;
        self
    }

    #[must_use]
    pub fn entry(mut self, entry: Entry<'a, Message>) -> Self {
        self.entries.push(entry);
        self
    }

    #[must_use]
    pub fn maybe_entry(mut self, maybe_entry: Option<Entry<'a, Message>>) -> Self {
        if let Some(e) = maybe_entry {
            self.entries.push(e)
        }
        self
    }
}

impl<'a, Message: 'a + Clone> From<FABMenu<'a, Message>> for Element<'a, Message> {
    fn from(menu: FABMenu<'a, Message>) -> Self {
        let mut buttons: Vec<Element<'a, Message>> = menu
            .entries
            .into_iter()
            .map(|e| {
                crate::widget::button(
                    menu.style,
                    button::Content::Label(e.label.into()).icon_maybe(e.icon),
                )
                .on_press(e.message)
                .size(crate::widget::button::Size::Medium) // TEST: Height should be 56px
                .label_font_maybe(menu.label_font)
                .corner_style(button::CornerStyle::Custom {
                    resting: f32::MAX.into(),
                    pressed: f32::MAX.into(),
                })
                .into()
            })
            .collect();
        buttons.push(
            space()
                .height(Length::Fixed(BUTTON_TRIGGER_BETWEEN_SPACE))
                .into(),
        );

        super::advanced::drop_down_menu(
            move |opened| {
                // FIX: Doesn't change the color
                center(widget::hybrid_icon(
                    (menu.icon)(opened),
                    TRIGGER_ICON_SIZE,
                    menu.style.regular.icon,
                ))
                .width(Length::Fixed(TRIGGER_SIZE))
                .height(Length::Fixed(TRIGGER_SIZE))
                .style(move |_| trigger_style(menu.style, opened))
                .into()
            },
            Some(
                column(buttons)
                    .spacing(BUTTON_SPACING)
                    .align_x(Alignment::End),
            ),
            Placement::TopLeft,
        )
        .menu_transparent(true)
        .into()
    }
}
