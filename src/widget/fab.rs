use iced::{Element, Length, advanced::text, padding};
use iced_widget::text::IntoFragment;

use crate::{
    style::Elevation,
    theme::{Accent, ColorScheme},
    widget::button::{self, OnPress},
};

#[derive(Default, Clone, Copy)]
pub enum Size {
    #[default]
    Regular,
    Medium,
    Large,
}

impl Size {
    fn container_size(&self) -> f32 {
        match self {
            Size::Regular => 56.0,
            Size::Medium => 80.0,
            Size::Large => 96.0,
        }
    }

    fn content_spacing(&self) -> f32 {
        match self {
            Size::Regular => 8.0,
            Size::Medium => 12.0,
            Size::Large => 16.0,
        }
    }

    fn font_size(&self) -> f32 {
        match self {
            Size::Regular => 16.0,
            Size::Medium => 22.0,
            Size::Large => 24.0,
        }
    }

    fn padding(&self) -> f32 {
        match self {
            Size::Regular => 16.0,
            Size::Medium => 26.0,
            Size::Large => 28.0,
        }
    }

    fn icon_size(&self) -> f32 {
        match self {
            Size::Regular => 24.0,
            Size::Medium => 28.0,
            Size::Large => 36.0,
        }
    }

    fn to_button_size(&self, extended: bool) -> button::Size {
        let (padding, container_width) = match extended {
            true => (self.padding(), Length::Shrink),
            false => (0.0, Length::Fixed(self.container_size())),
        };
        button::Size::Custom {
            width: container_width,
            height: Length::Fixed(self.container_size()),
            spacing: self.content_spacing(),
            padding: padding::horizontal(padding),
            icon_size: self.icon_size(),
            font_size: self.font_size(),
        }
    }

    fn to_corner_style(&self) -> button::CornerStyle {
        let radius = match self {
            Size::Regular => 16.0,
            Size::Medium => 20.0,
            Size::Large => 28.0,
        }
        .into();
        button::CornerStyle::Custom {
            resting: radius,
            pressed: radius,
        }
    }
}

#[derive(Clone)]
pub enum Content<'a> {
    Reguar {
        icon: char,
    },
    Extended {
        icon: char,
        label: text::Fragment<'a>,
    },
}

impl<'a> Content<'a> {
    fn icon(&'a self) -> char {
        match self {
            Content::Reguar { icon } => *icon,
            Content::Extended { icon, label: _ } => *icon,
        }
    }

    pub fn with_label(self, label: impl IntoFragment<'a>) -> Self {
        Self::Extended {
            icon: self.icon(),
            label: label.into_fragment(),
        }
    }
}

impl<'a> From<Content<'a>> for button::Content<'a> {
    fn from(value: Content<'a>) -> Self {
        match value {
            Content::Reguar { icon } => button::Content::Icon(icon),
            Content::Extended { icon, label } => button::Content::Full { icon, label },
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Style {
    #[default]
    PrimaryContainer,
    SecondaryContainer,
    TertiaryContainer,
    Primary,
    Secondary,
    Tertiary,
}

impl From<Style> for button::Style {
    fn from(value: Style) -> Self {
        match value {
            Style::PrimaryContainer => button::Style::Tonal(Accent::Primary),
            Style::SecondaryContainer => button::Style::Tonal(Accent::Secondary),
            Style::TertiaryContainer => button::Style::Tonal(Accent::Tertiary),
            Style::Primary => button::Style::Filled(Accent::Primary),
            Style::Secondary => button::Style::Filled(Accent::Secondary),
            Style::Tertiary => button::Style::Filled(Accent::Tertiary),
        }
    }
}

pub struct Fab<'a, Message, Renderer = iced_widget::Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_widget::core::text::Renderer,
{
    content: Content<'a>,
    size: Size,
    style: Style,
    label_font: Option<Renderer::Font>,
    icon_font: Option<Renderer::Font>,
    theme: &'a dyn ColorScheme,
    on_press: OnPress<'a, Message>,
}

impl<'a, Message, Renderer> Fab<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced::advanced::text::Renderer,
{
    #[must_use]
    pub fn new(theme: &'a impl ColorScheme, content: Content<'a>, on_press: Message) -> Self {
        Self {
            content,
            size: Size::default(),
            style: Style::default(),
            label_font: None,
            icon_font: None,
            theme,
            on_press: OnPress::Direct(on_press),
        }
    }

    #[must_use]
    pub fn new_with(
        theme: &'a impl ColorScheme,
        content: Content<'a>,
        on_press: impl Fn() -> Message + 'a,
    ) -> Self {
        Self {
            content,
            size: Size::default(),
            style: Style::default(),
            label_font: None,
            icon_font: None,
            theme,
            on_press: OnPress::Closure(Box::new(on_press)),
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn size_maybe(self, maybe_size: Option<Size>) -> Self {
        match maybe_size {
            Some(size) => self.size(size),
            None => self,
        }
    }

    #[must_use]
    pub fn icon_font(mut self, icon_font: Renderer::Font) -> Self {
        self.icon_font = Some(icon_font);
        self
    }

    #[must_use]
    pub fn icon_font_maybe(mut self, maybe_icon_font: Option<Renderer::Font>) -> Self {
        self.icon_font = maybe_icon_font;
        self
    }

    #[must_use]
    pub fn label_font(mut self, label_font: Renderer::Font) -> Self {
        self.label_font = Some(label_font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, maybe_label_font: Option<Renderer::Font>) -> Self {
        self.label_font = maybe_label_font;
        self
    }

    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    #[must_use]
    pub fn style_maybe(self, maybe_style: Option<Style>) -> Self {
        match maybe_style {
            Some(style) => self.style(style),
            None => self,
        }
    }
}

impl<'a, Message> From<Fab<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(value: Fab<'a, Message>) -> Self {
        let is_extended_fab = matches!(value.content, Content::Extended { icon: _, label: _ });
        let button = crate::widget::button(value.theme, value.content.into())
            .corner_style(value.size.to_corner_style())
            .icon_font_maybe(value.icon_font)
            .label_font_maybe(value.label_font)
            .style(value.style.into())
            .size(value.size.to_button_size(is_extended_fab))
            .elevation(Elevation::Level3);

        match value.on_press {
            OnPress::Direct(on_press) => button.on_press(on_press),
            OnPress::Closure(on_press) => button.on_press_with(on_press),
        }
        .into()
    }
}
