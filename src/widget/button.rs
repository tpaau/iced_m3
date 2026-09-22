use iced::{Alignment, Color, Element, Length, Padding, border::Radius, padding};
use iced_widget::{center, row, text};

use crate::{
    style::{DISABLED_STATE_LAYER_OPACITY, Elevation, StateLayer, mix_colors, shadow},
    theme::{Accent, ColorScheme},
    widget::{OnPress, icon},
};

const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
const DISABLED_CONTENT_OPACITY: f32 = DISABLED_STATE_LAYER_OPACITY;

#[derive(Clone, Copy, PartialEq)]
pub struct Outline {
    pub width: f32,
    pub color: Color,
}

impl Default for Outline {
    fn default() -> Self {
        Self {
            width: 0.0,
            color: Color::TRANSPARENT,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct StateStyle {
    pub container: Option<Color>,
    pub label: Color,
    // FIX: This is currently unused
    pub icon: Color,
    pub outline: Outline,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ElevationStates {
    pub shadow_color: Color,
    pub idle: Elevation,
    pub disabled: Elevation,
    pub hover: Elevation,
    pub press: Elevation,
}

impl ElevationStates {
    pub fn elevation(&self, status: iced_widget::button::Status) -> Elevation {
        match status {
            iced_widget::button::Status::Active => self.idle,
            iced_widget::button::Status::Hovered => self.hover,
            iced_widget::button::Status::Pressed => self.press,
            iced_widget::button::Status::Disabled => self.disabled,
        }
    }
}

impl ElevationStates {
    pub fn new(shadow_color: Color) -> Self {
        Self {
            shadow_color,
            idle: Elevation::default(),
            disabled: Elevation::default(),
            hover: Elevation::default(),
            press: Elevation::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Style {
    pub regular: StateStyle,
    pub unselected: StateStyle,
    pub selected: StateStyle,
    pub disabled: StateStyle,
    pub disabled_unselected: StateStyle,
    pub disabled_selected: StateStyle,
    pub elevation: ElevationStates,
    pub state_layer: StateLayer,
    pub state_layer_unselected: StateLayer,
    pub state_layer_selected: StateLayer,
}

impl Style {
    pub fn elevated(theme: &(impl ColorScheme + ?Sized), accent: Accent) -> Self {
        let disabled = StateStyle {
            container: Some(theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY)),
            label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            outline: Outline::default(),
        };
        Self {
            regular: StateStyle {
                container: Some(theme.surface_container_low()),
                label: accent.color(theme),
                icon: accent.color(theme),
                outline: Outline::default(),
            },
            unselected: StateStyle {
                container: Some(theme.surface_container_low()),
                label: accent.color(theme),
                icon: accent.color(theme),
                outline: Outline::default(),
            },
            selected: StateStyle {
                container: Some(accent.color(theme)),
                label: accent.on_color(theme),
                icon: accent.on_color(theme),
                outline: Outline::default(),
            },
            disabled,
            disabled_unselected: disabled,
            disabled_selected: disabled,
            elevation: ElevationStates {
                shadow_color: theme.shadow(),
                idle: Elevation::Level1,
                disabled: Elevation::Level0,
                hover: Elevation::Level1,
                press: Elevation::Level1,
            },
            state_layer: StateLayer::new(accent.color(theme)),
            state_layer_unselected: StateLayer::new(accent.color(theme)),
            state_layer_selected: StateLayer::new(accent.on_color(theme)),
        }
    }

    pub fn filled(theme: &(impl ColorScheme + ?Sized), accent: Accent) -> Self {
        let disabled = StateStyle {
            container: Some(theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY)),
            label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            outline: Outline::default(),
        };
        Self {
            regular: StateStyle {
                container: Some(accent.color(theme)),
                label: accent.on_color(theme),
                icon: accent.on_color(theme),
                outline: Outline::default(),
            },
            unselected: StateStyle {
                container: Some(theme.surface_container()),
                label: theme.on_surface_variant(),
                icon: theme.on_surface_variant(),
                outline: Outline::default(),
            },
            selected: StateStyle {
                container: Some(accent.color(theme)),
                label: accent.on_color(theme),
                icon: accent.on_color(theme),
                outline: Outline::default(),
            },
            disabled,
            disabled_unselected: disabled,
            disabled_selected: disabled,
            elevation: ElevationStates::new(theme.shadow()),
            state_layer: StateLayer::new(accent.on_color(theme)),
            state_layer_unselected: StateLayer::new(theme.on_surface_variant()),
            state_layer_selected: StateLayer::new(accent.on_color(theme)),
        }
    }

    pub fn tonal(theme: &(impl ColorScheme + ?Sized), accent: Accent) -> Self {
        let disabled = StateStyle {
            container: Some(theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY)),
            label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            outline: Outline::default(),
        };
        Self {
            regular: StateStyle {
                container: Some(accent.color_container(theme)),
                label: accent.on_color_container(theme),
                icon: accent.on_color_container(theme),
                outline: Outline::default(),
            },
            unselected: StateStyle {
                container: Some(accent.color_container(theme)),
                label: accent.on_color_container(theme),
                icon: accent.on_color_container(theme),
                outline: Outline::default(),
            },
            selected: StateStyle {
                container: Some(accent.color(theme)),
                label: accent.on_color(theme),
                icon: accent.on_color(theme),
                outline: Outline::default(),
            },
            disabled,
            disabled_unselected: disabled,
            disabled_selected: disabled,
            elevation: ElevationStates::new(theme.shadow()),
            state_layer: StateLayer::new(accent.on_color_container(theme)),
            state_layer_unselected: StateLayer::new(accent.on_color_container(theme)),
            state_layer_selected: StateLayer::new(accent.on_color(theme)),
        }
    }

    pub fn outlined(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            regular: StateStyle {
                container: None,
                label: theme.on_surface_variant(),
                icon: theme.on_surface_variant(),
                outline: Outline {
                    width: 1.0,
                    color: theme.outline_variant(),
                },
            },
            unselected: StateStyle {
                container: None,
                label: theme.on_surface_variant(),
                icon: theme.on_surface_variant(),
                outline: Outline {
                    width: 1.0,
                    color: theme.on_surface_variant(),
                },
            },
            selected: StateStyle {
                container: Some(theme.inverse_surface()),
                label: theme.inverse_on_surface(),
                icon: theme.inverse_on_surface(),
                outline: Outline {
                    width: 1.0,
                    color: theme.outline_variant(),
                },
            },
            disabled: StateStyle {
                container: None,
                label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                outline: Outline {
                    width: 1.0,
                    color: theme.outline_variant(),
                },
            },
            disabled_unselected: StateStyle {
                container: None,
                label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                outline: Outline {
                    width: 1.0,
                    color: theme.outline_variant(),
                },
            },
            disabled_selected: StateStyle {
                container: Some(theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY)),
                label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                outline: Outline {
                    width: 1.0,
                    color: theme.outline_variant(),
                },
            },
            elevation: ElevationStates::new(theme.shadow()),
            state_layer: StateLayer::new(theme.on_surface_variant()),
            state_layer_unselected: StateLayer::new(theme.on_surface_variant()),
            state_layer_selected: StateLayer::new(theme.inverse_on_surface()),
        }
    }

    pub fn text(theme: &(impl ColorScheme + ?Sized), accent: Accent) -> Self {
        let regular = StateStyle {
            container: None,
            label: accent.color(theme),
            icon: accent.color(theme),
            outline: Outline::default(),
        };
        let disabled = StateStyle {
            container: Some(theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY)),
            label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            outline: Outline::default(),
        };
        Self {
            regular,
            unselected: regular,
            selected: regular,
            disabled,
            disabled_unselected: disabled,
            disabled_selected: disabled,
            elevation: ElevationStates::new(theme.shadow()),
            state_layer: StateLayer::new(accent.color(theme)),
            state_layer_unselected: StateLayer::new(accent.color(theme)),
            state_layer_selected: StateLayer::new(accent.color(theme)),
        }
    }

    pub fn state_layer(&self, selected: Option<bool>) -> &StateLayer {
        match selected {
            Some(selected) => match selected {
                true => &self.state_layer_selected,
                false => &self.state_layer_unselected,
            },
            None => &self.state_layer,
        }
    }

    pub fn state_style(&self, disabled: bool, selected: Option<bool>) -> &StateStyle {
        match disabled {
            true => match selected {
                Some(selected) => match selected {
                    true => &self.disabled_selected,
                    false => &self.disabled_unselected,
                },
                None => &self.disabled,
            },
            false => match selected {
                Some(selected) => match selected {
                    true => &self.selected,
                    false => &self.unselected,
                },
                None => &self.regular,
            },
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Size {
    ExtraSmall,
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
    Custom {
        width: Length,
        height: Length,
        spacing: f32,
        padding: Padding,
        icon_size: f32,
        font_size: f32,
    },
}

impl Size {
    pub fn width(&self) -> Length {
        match self {
            Self::Custom {
                width,
                height: _,
                spacing: _,
                padding: _,
                icon_size: _,
                font_size: _,
            } => *width,
            _ => Length::Shrink,
        }
    }

    pub fn with_width(self, width: Length) -> Self {
        Self::Custom {
            width,
            height: self.height(),
            spacing: self.spacing(),
            padding: self.padding(),
            icon_size: self.icon_size(),
            font_size: self.font_size(),
        }
    }

    pub fn height(&self) -> Length {
        match self {
            Self::ExtraSmall => Length::Fixed(32.0),
            Self::Small => Length::Fixed(40.0),
            Self::Medium => Length::Fixed(56.0),
            Self::Large => Length::Fixed(96.0),
            Self::ExtraLarge => Length::Fixed(136.0),
            Self::Custom {
                width: _,
                height,
                spacing: _,
                padding: _,
                icon_size: _,
                font_size: _,
            } => *height,
        }
    }

    pub fn with_height(self, height: Length) -> Self {
        Self::Custom {
            width: self.width(),
            height,
            spacing: self.spacing(),
            padding: self.padding(),
            icon_size: self.icon_size(),
            font_size: self.font_size(),
        }
    }

    pub fn spacing(&self) -> f32 {
        match self {
            Size::ExtraSmall => 4.0,
            Size::Small | Size::Medium => 8.0,
            Size::Large => 12.0,
            Size::ExtraLarge => 16.0,
            Size::Custom {
                width: _,
                height: _,
                spacing,
                padding: _,
                icon_size: _,
                font_size: _,
            } => *spacing,
        }
    }

    pub fn with_spacing(self, spacing: f32) -> Self {
        Self::Custom {
            width: self.width(),
            height: self.height(),
            spacing,
            padding: self.padding(),
            icon_size: self.icon_size(),
            font_size: self.font_size(),
        }
    }

    pub fn padding(&self) -> Padding {
        match self {
            Self::ExtraSmall => padding::horizontal(12.0),
            Self::Small => padding::horizontal(16.0),
            Self::Medium => padding::horizontal(24.0),
            Self::Large => padding::horizontal(48.0),
            Self::ExtraLarge => padding::horizontal(64.0),
            Self::Custom {
                width: _,
                height: _,
                spacing: _,
                padding,
                icon_size: _,
                font_size: _,
            } => *padding,
        }
    }

    pub fn with_padding(self, padding: impl Into<Padding>) -> Self {
        Self::Custom {
            width: self.width(),
            height: self.height(),
            spacing: self.spacing(),
            padding: padding.into(),
            icon_size: self.icon_size(),
            font_size: self.font_size(),
        }
    }

    pub fn icon_size(&self) -> f32 {
        match self {
            Size::ExtraSmall => 20.0,
            Size::Small => 20.0,
            Size::Medium => 24.0,
            Size::Large => 32.0,
            Size::ExtraLarge => 40.0,
            Size::Custom {
                width: _,
                height: _,
                spacing: _,
                padding: _,
                icon_size,
                font_size: _,
            } => *icon_size,
        }
    }

    pub fn with_icon_size(self, icon_size: f32) -> Self {
        Self::Custom {
            width: self.width(),
            height: self.height(),
            spacing: self.spacing(),
            padding: self.padding(),
            icon_size,
            font_size: self.font_size(),
        }
    }

    // TODO: Implement the typography system and get the sizes from there
    pub fn font_size(&self) -> f32 {
        match self {
            Size::ExtraSmall | Size::Small => 14.0,
            Size::Medium => 16.0,
            Size::Large => 24.0,
            Size::ExtraLarge => 32.0,
            Size::Custom {
                width: _,
                height: _,
                spacing: _,
                padding: _,
                icon_size: _,
                font_size,
            } => *font_size,
        }
    }

    pub fn with_font_size(self, font_size: f32) -> Self {
        Self::Custom {
            width: self.width(),
            height: self.height(),
            spacing: self.spacing(),
            padding: self.padding(),
            icon_size: self.icon_size(),
            font_size,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum CornerStyle {
    #[default]
    Round,
    Square,
    Custom {
        resting: Radius,
        pressed: Radius,
    },
}

impl CornerStyle {
    fn resting(&self, size: &Size, selected: bool) -> Radius {
        if selected {
            self.pressed(size, false)
        } else {
            match self {
                CornerStyle::Round => Radius::new(f32::MAX),
                CornerStyle::Square => match size {
                    Size::ExtraSmall | Size::Small => Radius::new(12.0),
                    Size::Medium
                    | Size::Custom {
                        height: _,
                        padding: _,
                        width: _,
                        spacing: _,
                        icon_size: _,
                        font_size: _,
                    } => Radius::new(16.0),
                    Size::Large | Size::ExtraLarge => Radius::new(28.0),
                },
                CornerStyle::Custom {
                    resting,
                    pressed: _,
                } => *resting,
            }
        }
    }

    fn pressed(&self, size: &Size, selected: bool) -> Radius {
        if selected {
            self.resting(size, false)
        } else {
            match self {
                CornerStyle::Round | CornerStyle::Square => match size {
                    Size::ExtraSmall | Size::Small => Radius::new(8.0),
                    Size::Medium
                    | Size::Custom {
                        height: _,
                        padding: _,
                        width: _,
                        spacing: _,
                        icon_size: _,
                        font_size: _,
                    } => Radius::new(12.0),
                    Size::Large | Size::ExtraLarge => Radius::new(16.0),
                },
                CornerStyle::Custom {
                    resting: _,
                    pressed,
                } => *pressed,
            }
        }
    }
}

pub(crate) fn style(
    status: iced_widget::button::Status,
    selected: Option<bool>,
    button_size: Size,
    style: Style,
    corner_style: CornerStyle,
) -> iced_widget::button::Style {
    let is_disabled = status == iced_widget::button::Status::Disabled;
    let state_style = style.state_style(is_disabled, selected);
    let corner_radius = if status == iced_widget::button::Status::Pressed {
        corner_style.pressed(&button_size, selected.unwrap_or(false))
    } else {
        corner_style.resting(&button_size, selected.unwrap_or(false))
    };
    let border = iced::Border {
        radius: corner_radius,
        color: state_style.outline.color,
        width: state_style.outline.width,
    };
    let state_layer_color = match status {
        iced_widget::button::Status::Active | iced_widget::button::Status::Disabled => {
            style.state_layer.idle
        }
        iced_widget::button::Status::Hovered => style.state_layer.hovered,
        iced_widget::button::Status::Pressed => style.state_layer.pressed,
    };
    let container_color = match state_style.container {
        Some(color) => mix_colors(
            color,
            Color {
                a: 1.0,
                ..state_layer_color
            },
            state_layer_color.a,
        ),
        None => state_layer_color,
    };

    iced_widget::button::Style {
        background: Some(iced::Background::Color(container_color)),
        text_color: state_style.label,
        border,
        shadow: shadow(
            style.elevation.shadow_color,
            style.elevation.elevation(status),
        ),
        snap: true,
    }
}

#[derive(Clone)]
pub enum Content<'a> {
    Icon(text::Fragment<'a>),
    Label(text::Fragment<'a>),
    Full {
        icon: text::Fragment<'a>,
        label: text::Fragment<'a>,
    },
}

impl<'a> Content<'a> {
    #[must_use]
    pub fn label(self, label: impl text::IntoFragment<'a>) -> Self {
        match self {
            Content::Icon(icon) => Self::Full {
                icon,
                label: label.into_fragment(),
            },
            Content::Label(_) => Self::Label(label.into_fragment()),
            Content::Full { icon, label: _ } => Self::Full {
                icon,
                label: label.into_fragment(),
            },
        }
    }

    #[must_use]
    pub fn label_maybe(self, maybe_label: Option<impl text::IntoFragment<'a>>) -> Self {
        match maybe_label {
            Some(label) => self.label(label),
            None => self,
        }
    }

    #[must_use]
    pub fn icon(self, icon: impl text::IntoFragment<'a>) -> Self {
        match self {
            Content::Icon(_) => Self::Icon(icon.into_fragment()),
            Content::Label(label) => Self::Full {
                icon: icon.into_fragment(),
                label,
            },
            Content::Full { icon: _, label } => Self::Full {
                icon: icon.into_fragment(),
                label,
            },
        }
    }

    #[must_use]
    pub fn icon_maybe(self, maybe_icon: Option<char>) -> Self {
        match maybe_icon {
            Some(icon) => self.icon(icon),
            None => self,
        }
    }

    #[must_use]
    fn get_icon(self) -> Option<text::Fragment<'a>> {
        match self {
            Content::Icon(icon) => Some(icon),
            Content::Label(_) => None,
            Content::Full { icon, label: _ } => Some(icon),
        }
    }

    #[must_use]
    fn get_label(&self) -> Option<&str> {
        match self {
            Content::Icon(_) => None,
            Content::Label(label) => Some(label),
            Content::Full { icon: _, label } => Some(label),
        }
    }
}

pub struct Button<'a, Message>
where
    Message: Clone,
{
    style: Style,
    on_press: Option<OnPress<'a, Message>>,
    clip: bool,
    content: Content<'a>,
    label_font: Option<iced::Font>,
    icon_font: Option<iced::Font>,
    size: Size,
    corner_style: CornerStyle,
    elevation: Elevation,
    selected: Option<bool>,
}

impl<'a, Message> Button<'a, Message>
where
    Message: Clone,
{
    #[must_use]
    pub fn new(style: Style, content: Content<'a>) -> Self {
        Self {
            style,
            on_press: None,
            clip: false,
            content,
            label_font: None,
            icon_font: None,
            size: Size::default(),
            corner_style: CornerStyle::default(),
            elevation: Elevation::default(),
            selected: None,
        }
    }

    #[must_use]
    pub fn label_font(mut self, font: iced::Font) -> Self {
        self.label_font = Some(font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, maybe_font: Option<iced::Font>) -> Self {
        self.label_font = maybe_font;
        self
    }

    #[must_use]
    pub fn icon_font(mut self, font: iced::Font) -> Self {
        self.icon_font = Some(font);
        self
    }

    #[must_use]
    pub fn icon_font_maybe(mut self, maybe_font: Option<iced::Font>) -> Self {
        self.icon_font = maybe_font;
        self
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn corner_style(mut self, corner_style: CornerStyle) -> Self {
        self.corner_style = corner_style;
        self
    }

    #[must_use]
    pub fn elevation(mut self, elevation: impl Into<Elevation>) -> Self {
        self.elevation = elevation.into();
        self
    }

    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    #[must_use]
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    #[must_use]
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }

    #[must_use]
    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    #[must_use]
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }
}

impl<'a, Message> From<Button<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(button: Button<'a, Message>) -> Self {
        let label = button.content.get_label().map(|l| {
            text(l.to_string())
                .wrapping(text::Wrapping::None)
                .size(button.size.font_size())
                .font_maybe(button.label_font)
        });
        let icon = button
            .content
            .get_icon()
            .map(|i| icon(i, button.size.icon_size()).font_maybe(button.icon_font));

        let content = row![icon, label,]
            .align_y(Alignment::Center)
            .spacing(button.size.spacing());

        let button_widget = iced_widget::button(center(content))
            .width(button.size.width())
            .height(button.size.height())
            .padding(button.size.padding())
            .style(move |_, status| {
                style(
                    status,
                    button.selected,
                    button.size,
                    button.style,
                    button.corner_style,
                )
            });

        let button_widget = match button.on_press {
            Some(on_press) => match on_press {
                OnPress::Direct(on_press) => button_widget.on_press(on_press),
                OnPress::Closure(on_press) => button_widget.on_press_with(on_press),
            },
            None => button_widget,
        };

        button_widget.into()
    }
}
