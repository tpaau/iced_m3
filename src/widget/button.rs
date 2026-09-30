use std::time::Instant;

use iced::{
    Color, Element, Event, Length, Padding, Pixels, Radians, Rectangle,
    advanced::{
        Clipboard, Layout, Shell, Widget,
        layout::{self, Node},
        mouse, renderer,
        text::Paragraph,
        widget::{Tree, tree},
    },
    border::Radius,
    padding, touch,
};
use iced_widget::text;

use crate::{
    animation::{
        Interpolable,
        motion::{self, Spring, SpringValue, fast_effects, fast_spatial},
    },
    style::{DISABLED_STATE_LAYER_OPACITY, Elevation, StateLayer, shadow},
    theme::{Accent, ColorScheme},
    widget::{self, OnPress, hybrid_icon::Icon},
};

mod constants {
    pub const BUTTON_HEIGHT_EXTRA_LARGE: f32 = 136.0;
    pub const BUTTON_HEIGHT_LARGE: f32 = 96.0;
    pub const BUTTON_HEIGHT_MEDIUM: f32 = 56.0;
    pub const BUTTON_HEIGHT_SMALL: f32 = 40.0;
    pub const BUTTON_HEIGHT_EXTRA_SMALL: f32 = 32.0;
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
const DISABLED_CONTENT_OPACITY: f32 = DISABLED_STATE_LAYER_OPACITY;

fn normalize_radius(radius: Radius, max: f32) -> Radius {
    Radius {
        top_left: radius.top_left.clamp(0.0, max),
        top_right: radius.top_right.clamp(0.0, max),
        bottom_right: radius.bottom_right.clamp(0.0, max),
        bottom_left: radius.bottom_left.clamp(0.0, max),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Interpolable for Outline {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Self {
            width: self.width.interpolate(other.width, t),
            color: self.color.interpolate(other.color, t),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateStyle {
    pub container: Color,
    pub label: Color,
    pub icon: Color,
    pub outline: Outline,
}

impl Interpolable for StateStyle {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Self {
            container: self.container.interpolate(other.container, t),
            label: self.label.interpolate(other.label, t),
            icon: self.icon.interpolate(other.icon, t),
            outline: self.outline.interpolate(other.outline, t),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevationStates {
    pub shadow_color: Color,
    pub idle: Elevation,
    pub disabled: Elevation,
    pub hover: Elevation,
    pub press: Elevation,
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
            container: theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY),
            label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            outline: Outline::default(),
        };
        Self {
            regular: StateStyle {
                container: theme.surface_container_low(),
                label: accent.color(theme),
                icon: accent.color(theme),
                outline: Outline::default(),
            },
            unselected: StateStyle {
                container: theme.surface_container_low(),
                label: accent.color(theme),
                icon: accent.color(theme),
                outline: Outline::default(),
            },
            selected: StateStyle {
                container: accent.color(theme),
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
            container: theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY),
            label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            outline: Outline::default(),
        };
        Self {
            regular: StateStyle {
                container: accent.color(theme),
                label: accent.on_color(theme),
                icon: accent.on_color(theme),
                outline: Outline::default(),
            },
            unselected: StateStyle {
                container: theme.surface_container(),
                label: theme.on_surface_variant(),
                icon: theme.on_surface_variant(),
                outline: Outline::default(),
            },
            selected: StateStyle {
                container: accent.color(theme),
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
            container: theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY),
            label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
            outline: Outline::default(),
        };
        Self {
            regular: StateStyle {
                container: accent.color_container(theme),
                label: accent.on_color_container(theme),
                icon: accent.on_color_container(theme),
                outline: Outline::default(),
            },
            unselected: StateStyle {
                container: accent.color_container(theme),
                label: accent.on_color_container(theme),
                icon: accent.on_color_container(theme),
                outline: Outline::default(),
            },
            selected: StateStyle {
                container: accent.color(theme),
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
                container: Color::TRANSPARENT,
                label: theme.on_surface_variant(),
                icon: theme.on_surface_variant(),
                outline: Outline {
                    width: 1.0,
                    color: theme.outline_variant(),
                },
            },
            unselected: StateStyle {
                container: Color::TRANSPARENT,
                label: theme.on_surface_variant(),
                icon: theme.on_surface_variant(),
                outline: Outline {
                    width: 1.0,
                    color: theme.on_surface_variant(),
                },
            },
            selected: StateStyle {
                container: theme.inverse_surface(),
                label: theme.inverse_on_surface(),
                icon: theme.inverse_on_surface(),
                outline: Outline {
                    width: 1.0,
                    color: theme.outline_variant(),
                },
            },
            disabled: StateStyle {
                container: Color::TRANSPARENT,
                label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                outline: Outline {
                    width: 1.0,
                    color: theme.outline_variant(),
                },
            },
            disabled_unselected: StateStyle {
                container: Color::TRANSPARENT,
                label: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                icon: theme.on_surface().scale_alpha(DISABLED_CONTENT_OPACITY),
                outline: Outline {
                    width: 1.0,
                    color: theme.outline_variant(),
                },
            },
            disabled_selected: StateStyle {
                container: theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY),
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
            container: Color::TRANSPARENT,
            label: accent.color(theme),
            icon: accent.color(theme),
            outline: Outline::default(),
        };
        let disabled = StateStyle {
            container: theme.on_surface().scale_alpha(DISABLED_CONTAINER_OPACITY),
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

    pub fn fab_vibrant(theme: &(impl ColorScheme + ?Sized), accent: Accent) -> Self {
        Self::filled(theme, accent)
    }

    pub fn fab_tonal(theme: &(impl ColorScheme + ?Sized), accent: Accent) -> Self {
        Self::tonal(theme, accent)
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CornerStyle {
    #[default]
    Rounded,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerRadius {
    pub style: CornerStyle,
    pub shape_morph: bool,
    pub rounded: Radius,
    pub square: Radius,
    pub pressed: Radius,
}

impl Default for CornerRadius {
    fn default() -> Self {
        Self::small()
    }
}

impl CornerRadius {
    pub fn extra_small() -> Self {
        Self {
            style: CornerStyle::default(),
            shape_morph: true,
            rounded: f32::MAX.into(),
            square: 12.0.into(),
            pressed: 8.0.into(),
        }
    }

    pub fn small() -> Self {
        Self {
            style: CornerStyle::default(),
            shape_morph: true,
            rounded: f32::MAX.into(),
            square: 12.0.into(),
            pressed: 8.0.into(),
        }
    }

    pub fn medium() -> Self {
        Self {
            style: CornerStyle::default(),
            shape_morph: true,
            rounded: f32::MAX.into(),
            square: 16.0.into(),
            pressed: 12.0.into(),
        }
    }

    pub fn large() -> Self {
        Self {
            style: CornerStyle::default(),
            shape_morph: true,
            rounded: f32::MAX.into(),
            square: 28.0.into(),
            pressed: 16.0.into(),
        }
    }

    pub fn extra_large() -> Self {
        Self {
            style: CornerStyle::default(),
            shape_morph: true,
            rounded: f32::MAX.into(),
            square: 28.0.into(),
            pressed: 16.0.into(),
        }
    }

    pub fn style(mut self, style: CornerStyle) -> Self {
        self.style = style;
        self
    }

    pub fn shape_morph(mut self, shape_morph: bool) -> Self {
        self.shape_morph = shape_morph;
        self
    }

    pub fn radius(&self, pressed: bool, selected: Option<bool>, max_radius: f32) -> Radius {
        if pressed && self.shape_morph {
            return normalize_radius(self.pressed, max_radius);
        }
        let selected = selected.unwrap_or_default();
        let radius = match (selected, self.style) {
            (true, CornerStyle::Rounded) | (false, CornerStyle::Square) => self.square,
            (true, CornerStyle::Square) | (false, CornerStyle::Rounded) => self.rounded,
        };
        normalize_radius(radius, max_radius)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: Length,
    pub height: Length,
    pub spacing: Pixels,
    pub padding: Padding,
    pub icon_size: f32,
    pub label_size: f32,
    pub corner_radius: CornerRadius,
}

impl Default for Size {
    fn default() -> Self {
        Self::small()
    }
}

impl Size {
    pub fn extra_small() -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Fixed(constants::BUTTON_HEIGHT_EXTRA_SMALL),
            spacing: Pixels(4.0),
            padding: padding::horizontal(12.0),
            icon_size: 20.0,
            label_size: 14.0,
            corner_radius: CornerRadius::extra_small(),
        }
    }

    pub fn small() -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Fixed(constants::BUTTON_HEIGHT_SMALL),
            spacing: Pixels(8.0),
            padding: padding::horizontal(16.0),
            icon_size: 20.0,
            label_size: 14.0,
            corner_radius: CornerRadius::small(),
        }
    }

    pub fn medium() -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Fixed(constants::BUTTON_HEIGHT_MEDIUM),
            spacing: Pixels(8.0),
            padding: padding::horizontal(24.0),
            icon_size: 24.0,
            label_size: 16.0,
            corner_radius: CornerRadius::medium(),
        }
    }

    pub fn large() -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Fixed(constants::BUTTON_HEIGHT_LARGE),
            spacing: Pixels(12.0),
            padding: padding::horizontal(48.0),
            icon_size: 32.0,
            label_size: 24.0,
            corner_radius: CornerRadius::large(),
        }
    }

    pub fn extra_large() -> Self {
        Self {
            width: Length::Shrink,
            height: Length::Fixed(constants::BUTTON_HEIGHT_EXTRA_LARGE),
            spacing: Pixels(16.0),
            padding: padding::horizontal(64.0),
            icon_size: 40.0,
            label_size: 32.0,
            corner_radius: CornerRadius::extra_large(),
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn spacing(mut self, spacing: Pixels) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn icon_size(mut self, icon_size: f32) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn label_size(mut self, label_size: f32) -> Self {
        self.label_size = label_size;
        self
    }

    pub fn corner_radius(mut self, corner_radius: CornerRadius) -> Self {
        self.corner_radius = corner_radius;
        self
    }
}

#[derive(Clone)]
pub enum Content<'a> {
    Icon(Icon<'a>),
    Label(text::Fragment<'a>),
    Full {
        icon: Icon<'a>,
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
    pub fn icon(self, icon: Icon<'a>) -> Self {
        match self {
            Content::Icon(_) => Self::Icon(icon),
            Content::Label(label) => Self::Full { icon: icon, label },
            Content::Full { icon: _, label } => Self::Full { icon: icon, label },
        }
    }

    #[must_use]
    pub fn icon_maybe(self, maybe_icon: Option<Icon<'a>>) -> Self {
        match maybe_icon {
            Some(icon) => self.icon(icon),
            None => self,
        }
    }

    #[must_use]
    fn get_icon(&self) -> Option<&Icon<'a>> {
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

pub struct Button<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    style: Style,
    on_press: Option<OnPress<'a, Message>>,
    /// Make the button appear enabled even when there is no message is being emitted on press.
    force_enabled: bool,
    clip: bool,
    content: Content<'a>,
    label_font: Option<iced::Font>,
    size: Size,
    elevation: Elevation,
    selected: Option<bool>,
    style_spring: Option<Spring>,
    state_layer_spring: Option<Spring>,
    corner_radius_spring: Option<Spring>,

    // Cache
    is_hovered: bool,
    style_change_checked: bool,
    icon_paragraph: Option<Renderer::Paragraph>,
    label_paragraph: Option<Renderer::Paragraph>,
}

impl<'a, Message, Renderer> Button<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    #[must_use]
    pub fn new(style: Style, content: Content<'a>) -> Self {
        Self {
            style,
            on_press: None,
            force_enabled: false,
            clip: false,
            content,
            label_font: None,
            size: Size::default(),
            elevation: Elevation::default(),
            selected: None,
            style_spring: None,
            state_layer_spring: None,
            corner_radius_spring: None,

            is_hovered: false,
            icon_paragraph: None,
            label_paragraph: None,
            style_change_checked: false,
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
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
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

    #[must_use]
    pub fn motion_scheme(mut self, scheme: motion::Scheme) -> Self {
        self.style_spring = Some(fast_effects(scheme));
        self.state_layer_spring = Some(fast_effects(scheme));
        self.corner_radius_spring = Some(fast_spatial(scheme));
        self
    }

    #[must_use]
    pub fn motion_scheme_maybe(self, scheme: Option<motion::Scheme>) -> Self {
        match scheme {
            Some(scheme) => self.motion_scheme(scheme),
            None => self,
        }
    }

    #[must_use]
    pub fn style_spring(mut self, spring: Spring) -> Self {
        self.style_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn style_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.style_spring = spring;
        self
    }

    #[must_use]
    pub fn state_layer_spring(mut self, spring: Spring) -> Self {
        self.state_layer_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn state_layer_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.state_layer_spring = spring;
        self
    }

    #[must_use]
    pub fn corner_radius_spring(mut self, spring: Spring) -> Self {
        self.corner_radius_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn corner_radius_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.corner_radius_spring = spring;
        self
    }

    fn get_style_spring(&self) -> Spring {
        self.style_spring
            .unwrap_or(fast_effects(motion::Scheme::default()))
    }

    fn get_state_layer_spring(&self) -> Spring {
        self.state_layer_spring
            .unwrap_or(fast_effects(motion::Scheme::default()))
    }

    fn get_corner_radius_spring(&self) -> Spring {
        self.corner_radius_spring
            .unwrap_or(fast_spatial(motion::Scheme::default()))
    }

    /// Make the button appear enabled even when there is no message is being emitted on press.
    ///
    /// This is currently only used for the FAB Menu widget as its opened state is managed internally.
    #[must_use]
    pub(crate) fn force_enabled(mut self, force_enabled: bool) -> Self {
        self.force_enabled = force_enabled;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct State {
    is_pressed: bool,
    style_spring: SpringValue<StateStyle>,
    state_layer_spring: SpringValue<Color>,
    corner_radius_spring: Option<SpringValue<Radius>>,
    last_style: Style,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Button<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a
        + iced::advanced::Renderer
        + iced::advanced::text::Renderer
        + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let style = self.style.state_style(
            self.on_press.is_none() && !self.force_enabled,
            self.selected,
        );
        let now = Instant::now();
        let state = State {
            is_pressed: false,
            style_spring: SpringValue::new(*style, self.get_style_spring(), now),
            state_layer_spring: SpringValue::new(
                self.style.state_layer.idle,
                self.get_state_layer_spring(),
                now,
            ),
            corner_radius_spring: None,
            last_style: self.style,
        };

        tree::State::new(state)
    }

    fn size(&self) -> iced::Size<Length> {
        iced::Size {
            width: self.size.width,
            height: self.size.height.into(),
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let padding = self.size.padding;
        let limits_shrink =
            iced::Size::new(padding.left + padding.right, padding.top + padding.bottom);
        let shrinked_limits = limits.shrink(limits_shrink);

        if let Some(Icon::Text { text, font }) = self.content.get_icon() {
            self.icon_paragraph = Some(
                <Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(
                    iced::advanced::text::Text {
                        content: &text,
                        bounds: shrinked_limits.max(),
                        size: Pixels(self.size.icon_size),
                        line_height: text::LineHeight::Absolute(Pixels(self.size.icon_size)),
                        font: font
                            .map(Renderer::Font::from)
                            .unwrap_or_else(|| renderer.default_font()),
                        shaping: text::Shaping::Advanced,
                        wrapping: text::Wrapping::None,
                        align_x: text::Alignment::Left,
                        align_y: iced::alignment::Vertical::Top,
                    },
                ),
            );
        }

        let contains_icon = matches!(&self.content, Content::Icon(_) | Content::Full { .. });
        let icon_node = contains_icon.then(|| {
            let icon_size = self.size.icon_size;
            Node::new(iced::Size::new(icon_size, icon_size))
        });

        let paragraph = self.content.get_label().map(|label| {
            <Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(
                iced::advanced::text::Text {
                    content: &label,
                    bounds: shrinked_limits.max(),
                    size: Pixels(self.size.label_size),
                    line_height: text::LineHeight::Relative(1.0),
                    font: self
                        .label_font
                        .map(Renderer::Font::from)
                        .unwrap_or_else(|| renderer.default_font()),
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                    align_x: text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                },
            )
        });

        let label_node = paragraph
            .as_ref()
            .map(|paragraph| Node::new(paragraph.min_bounds()));
        self.label_paragraph = paragraph;

        let spacing = if icon_node.is_some() && label_node.is_some() {
            self.size.spacing
        } else {
            Pixels::ZERO
        };

        let content_width = icon_node.as_ref().map_or(0.0, |node| node.size().width)
            + label_node.as_ref().map_or(0.0, |node| node.size().width)
            + spacing.0;
        let width = match self.size.width {
            Length::Fixed(width) => width,
            _ => content_width + padding.left + padding.right,
        };

        let content_height = icon_node
            .as_ref()
            .map_or(0.0, |node| node.size().height)
            .max(label_node.as_ref().map_or(0.0, |node| node.size().height));
        let height = match self.size.height {
            Length::Fixed(height) => height,
            _ => content_height + padding.top + padding.bottom,
        };
        let intrinsic_size = iced::Size::new(width, height);
        let size = limits.resolve(intrinsic_size.width, intrinsic_size.height, intrinsic_size);

        let state = tree.state.downcast_mut::<State>();
        if state.corner_radius_spring.is_none() {
            let corner_radius = self.size.corner_radius.radius(
                state.is_pressed && self.is_hovered,
                self.selected,
                size.width.min(size.height) / 2.0,
            );
            let now = Instant::now();
            state.corner_radius_spring = Some(SpringValue::new(
                corner_radius,
                self.get_corner_radius_spring(),
                now,
            ));
        }

        let content_area_width = size.width - padding.left - padding.right;
        let content_area_height = size.height - padding.top - padding.bottom;

        let group_x = padding.left + (content_area_width - content_width) / 2.0;
        let group_y = padding.top + (content_area_height - content_height) / 2.0;

        let mut children = Vec::with_capacity(2);
        let mut offset = 0.0;

        if let Some(icon) = icon_node {
            let y = group_y + (content_height - icon.size().height) / 2.0;
            offset += icon.size().width + spacing.0;
            children.push(icon.move_to(iced::Point::new(group_x, y)));
        }

        if let Some(label) = label_node {
            let y = group_y + (content_height - label.size().height) / 2.0;
            children.push(label.move_to(iced::Point::new(group_x + offset, y)));
        }

        layout::Node::with_children(size, children)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        self.is_hovered = cursor.is_over(bounds);
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() || self.force_enabled {
                    if self.is_hovered {
                        state.is_pressed = true;

                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if self.on_press.is_some() || self.force_enabled {
                    if state.is_pressed {
                        state.is_pressed = false;

                        if self.is_hovered
                            && let Some(on_press) = &self.on_press
                        {
                            shell.publish(on_press.resolve());
                        }

                        shell.capture_event();
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                state.is_pressed = false;
            }
            _ => {}
        }

        let spring = self.get_style_spring();
        if state.style_spring.spring != spring {
            state.style_spring.spring = spring;
        }
        let spring = self.get_state_layer_spring();
        if state.state_layer_spring.spring != spring {
            state.state_layer_spring.spring = spring;
        }
        let corner_radius_spring = state.corner_radius_spring.as_mut().unwrap();
        let spring = self.get_corner_radius_spring();
        if corner_radius_spring.spring != spring {
            corner_radius_spring.spring = spring;
        }

        let now = Instant::now();

        let state_layer = self.style.state_layer(self.selected);
        let state_layer_color = if self.on_press.is_none() && !self.force_enabled {
            state_layer.idle
        } else if cursor.is_over(layout.bounds()) {
            if state.is_pressed {
                state_layer.pressed
            } else {
                state_layer.hovered
            }
        } else {
            state_layer.idle
        };
        if state.state_layer_spring.to != state_layer_color {
            state.state_layer_spring.set_target(state_layer_color, now);
        }

        let style = *self.style.state_style(
            self.on_press.is_none() && !self.force_enabled,
            self.selected,
        );
        if !self.style_change_checked && state.last_style != self.style {
            state.style_spring.reset(style, now);
            self.style_change_checked = true;
            state.last_style = self.style;
        } else if state.style_spring.to != style {
            state.style_spring.set_target(style, now);
        }

        let corner_radius = self.size.corner_radius.radius(
            state.is_pressed && self.is_hovered,
            self.selected,
            bounds.width.min(bounds.height) / 2.0,
        );
        if corner_radius_spring.to != corner_radius {
            corner_radius_spring.set_target(corner_radius, now);
        }

        if !state.style_spring.is_at_rest()
            || !state.state_layer_spring.is_at_rest()
            || !corner_radius_spring.is_at_rest()
        {
            shell.request_redraw();
        }

        state.state_layer_spring.step(now);
        state.style_spring.step(now);
        corner_radius_spring.step(now);
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let mut children = layout.children();

        let elevation = if self.on_press.is_none() && !self.force_enabled {
            self.style.elevation.disabled
        } else if self.is_hovered {
            if state.is_pressed {
                self.style.elevation.press
            } else {
                self.style.elevation.hover
            }
        } else {
            self.style.elevation.idle
        };
        let style = state.style_spring.value();
        let corner_radius = state.corner_radius_spring.as_ref().unwrap().value();

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced::Border {
                    color: style.outline.color,
                    width: style.outline.width,
                    radius: corner_radius,
                },
                shadow: shadow(self.style.elevation.shadow_color, elevation),
                ..Default::default()
            },
            style.container,
        );

        if let Some(icon) = &self.icon_paragraph
            && let Some(layout) = children.next()
        {
            let bounds = layout.bounds();
            renderer.fill_paragraph(icon, bounds.position(), style.icon, bounds);
        } else if let Some(Icon::Svg(handle)) = self.content.get_icon()
            && let Some(layout) = children.next()
        {
            renderer.draw_svg(
                iced::advanced::svg::Svg {
                    handle: handle.clone(),
                    color: Some(style.icon),
                    rotation: Radians(0.0),
                    opacity: style.icon.a,
                },
                layout.bounds(),
                bounds,
            );
        }

        if let Some(label) = &self.label_paragraph
            && let Some(layout) = children.next()
        {
            renderer.fill_paragraph(label, layout.bounds().position(), style.label, bounds);
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced::Border::default().rounded(corner_radius),
                ..Default::default()
            },
            state.state_layer_spring.value(),
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over && (self.on_press.is_some() || self.force_enabled) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn size_hint(&self) -> iced::Size<Length> {
        <widget::button::Button<'_, Message, Renderer> as Widget<Message, Theme, Renderer>>::size(
            self,
        )
    }

    fn diff(&self, tree: &mut Tree) {
        tree.children.clear();
    }
}

impl<'a, Message, Theme, Renderer> From<Button<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn from(value: Button<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}
