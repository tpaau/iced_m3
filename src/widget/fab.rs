use iced::{Element, Length, advanced::text, padding};
use iced_widget::text::IntoFragment;

pub const EDGE_SPACING: f32 = 16.0;

use crate::{
    style::Elevation,
    widget::{OnPress, button, hybrid_icon::Icon},
};

pub type Style = button::Style;

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

    fn label_size(&self) -> f32 {
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
            label_size: self.label_size(),
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
        icon: Icon<'a>,
    },
    Extended {
        icon: Icon<'a>,
        label: text::Fragment<'a>,
    },
}

impl<'a> Content<'a> {
    fn icon(&'a self) -> &'a Icon<'a> {
        match self {
            Content::Reguar { icon } => icon,
            Content::Extended { icon, label: _ } => icon,
        }
    }

    pub fn with_label(&'a mut self, label: impl IntoFragment<'a>) -> Self {
        Self::Extended {
            icon: self.icon().clone(),
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

pub struct Fab<'a, Message, Renderer = iced_widget::Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_widget::core::text::Renderer,
{
    content: Content<'a>,
    size: Size,
    style: Style,
    label_font: Option<Renderer::Font>,
    on_press: OnPress<'a, Message>,
}

impl<'a, Message, Renderer> Fab<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced::advanced::text::Renderer,
{
    #[must_use]
    pub fn new(style: Style, content: Content<'a>, on_press: OnPress<'a, Message>) -> Self {
        Self {
            content,
            size: Size::default(),
            style,
            label_font: None,
            on_press: on_press,
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
    pub fn label_font(mut self, label_font: Renderer::Font) -> Self {
        self.label_font = Some(label_font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, maybe_label_font: Option<Renderer::Font>) -> Self {
        self.label_font = maybe_label_font;
        self
    }
}

impl<'a, Message> From<Fab<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(value: Fab<'a, Message>) -> Self {
        let is_extended_fab = matches!(value.content, Content::Extended { icon: _, label: _ });
        let button = crate::widget::button(value.style, value.content.into())
            .corner_style(value.size.to_corner_style())
            .label_font_maybe(value.label_font)
            .size(value.size.to_button_size(is_extended_fab))
            .elevation(Elevation::Level3);

        match value.on_press {
            OnPress::Direct(on_press) => button.on_press(on_press),
            OnPress::Closure(on_press) => button.on_press_with(on_press),
        }
        .into()
    }
}
