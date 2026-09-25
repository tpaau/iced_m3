use iced::{Element, Font, Length, advanced::text, border::Radius, padding};
use iced_widget::text::IntoFragment;

pub const EDGE_SPACING: f32 = 16.0;

use crate::{
    style::Elevation,
    widget::{
        OnPress,
        button::{self, CornerStyle},
        hybrid_icon::Icon,
    },
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
    pub(crate) fn container_height(&self) -> f32 {
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

    fn rounding(&self) -> Radius {
        match self {
            Size::Regular => 16.0,
            Size::Medium => 20.0,
            Size::Large => 28.0,
        }
        .into()
    }

    fn to_button_size(&self, extended: bool) -> button::Size {
        let (padding, container_width) = match extended {
            true => (self.padding(), Length::Shrink),
            false => (0.0, Length::Fixed(self.container_height())),
        };
        button::Size {
            width: container_width,
            height: Length::Fixed(self.container_height()),
            spacing: self.content_spacing(),
            padding: padding::horizontal(padding),
            icon_size: self.icon_size(),
            label_size: self.label_size(),
            corner_radius: button::CornerRadius {
                style: CornerStyle::default(),
                shape_morph: false,
                rounded: f32::MAX.into(),
                square: self.rounding(),
                pressed: 0.0.into(),
            },
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

pub struct Fab<'a, Message>
where
    Message: 'a + Clone,
{
    content: Content<'a>,
    size: Size,
    style: Style,
    label_font: Option<Font>,
    on_press: Option<OnPress<'a, Message>>,
}

impl<'a, Message> Fab<'a, Message>
where
    Message: 'a + Clone,
{
    /// Creates a new FAB menu that doesn't emit messages but appears enabled.
    #[must_use]
    pub fn new_dummy(style: Style, content: Content<'a>) -> Self {
        Self {
            content,
            size: Size::default(),
            style,
            label_font: None,
            on_press: None,
        }
    }

    #[must_use]
    pub fn new(style: Style, content: Content<'a>, on_press: OnPress<'a, Message>) -> Self {
        Self {
            content,
            size: Size::default(),
            style,
            label_font: None,
            on_press: Some(on_press),
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
    pub fn label_font(mut self, label_font: Font) -> Self {
        self.label_font = Some(label_font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, maybe_label_font: Option<Font>) -> Self {
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
            .label_font_maybe(value.label_font)
            .size(value.size.to_button_size(is_extended_fab))
            .elevation(Elevation::Level3);

        match value.on_press {
            Some(on_press) => match on_press {
                OnPress::Direct(on_press) => button.on_press(on_press),
                OnPress::Closure(on_press) => button.on_press_with(on_press),
            },
            None => button.force_enabled(true),
        }
        .into()
    }
}
