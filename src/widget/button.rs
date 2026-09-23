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
    padding, touch, window,
};
use iced_widget::text;

use crate::{
    style::{DISABLED_STATE_LAYER_OPACITY, Elevation, StateLayer, shadow},
    theme::{Accent, ColorScheme},
    widget::{self, OnPress, hybrid_icon::Icon},
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
    pub container: Color,
    pub label: Color,
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
    fn elevation(&self, status: Status) -> Elevation {
        match status {
            Status::Idle => self.idle,
            Status::Hovered => self.hover,
            Status::Pressed => self.press,
            Status::Disabled => self.disabled,
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
        label_size: f32,
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
                label_size: _,
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
            label_size: self.label_size(),
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
                label_size: _,
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
            label_size: self.label_size(),
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
                label_size: _,
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
            label_size: self.label_size(),
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
                label_size: _,
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
            label_size: self.label_size(),
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
                label_size: _,
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
            label_size: self.label_size(),
        }
    }

    pub fn label_size(&self) -> f32 {
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
                label_size,
            } => *label_size,
        }
    }

    pub fn with_label_size(self, label_size: f32) -> Self {
        Self::Custom {
            width: self.width(),
            height: self.height(),
            spacing: self.spacing(),
            padding: self.padding(),
            icon_size: self.icon_size(),
            label_size,
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
                        label_size: _,
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
                        label_size: _,
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
    clip: bool,
    content: Content<'a>,
    label_font: Option<iced::Font>,
    size: Size,
    corner_style: CornerStyle,
    elevation: Elevation,
    selected: Option<bool>,
    status: Option<Status>,
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
            clip: false,
            content,
            label_font: None,
            size: Size::default(),
            corner_style: CornerStyle::default(),
            elevation: Elevation::default(),
            selected: None,
            status: Some(Status::default()),
            icon_paragraph: None,
            label_paragraph: None,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    #[default]
    Idle,
    Hovered,
    Pressed,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
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
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        Vec::new()
    }

    fn size(&self) -> iced::Size<Length> {
        iced::Size {
            width: self.size.width(),
            height: self.size.height(),
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let padding = self.size.padding();
        let limits_shrink =
            iced::Size::new(padding.left + padding.right, padding.top + padding.bottom);
        let shrinked_limits = limits.shrink(limits_shrink);

        if let Some(Icon::Text { text, font }) = self.content.get_icon() {
            self.icon_paragraph = Some(
                <Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(
                    iced::advanced::text::Text {
                        content: &text,
                        bounds: shrinked_limits.max(),
                        size: Pixels(self.size.icon_size()),
                        line_height: text::LineHeight::Absolute(Pixels(self.size.icon_size())),
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
            let icon_size = self.size.icon_size();
            Node::new(iced::Size::new(icon_size, icon_size))
        });

        let paragraph = self.content.get_label().map(|label| {
            <Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(
                iced::advanced::text::Text {
                    content: &label,
                    bounds: shrinked_limits.max(),
                    size: Pixels(self.size.label_size()),
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
            self.size.spacing()
        } else {
            0.0
        };

        let content_width = icon_node.as_ref().map_or(0.0, |node| node.size().width)
            + label_node.as_ref().map_or(0.0, |node| node.size().width)
            + spacing;
        let width = match self.size.width() {
            Length::Fixed(width) => width,
            _ => content_width + padding.left + padding.right,
        };

        let content_height = icon_node
            .as_ref()
            .map_or(0.0, |node| node.size().height)
            .max(label_node.as_ref().map_or(0.0, |node| node.size().height));
        let height = match self.size.height() {
            Length::Fixed(height) => height,
            _ => content_height + padding.top + padding.bottom,
        };

        let intrinsic_size = iced::Size::new(width, height);
        let size = limits.resolve(intrinsic_size.width, intrinsic_size.height, intrinsic_size);

        let mut children = Vec::with_capacity(2);
        let mut offset = 0.0;
        let x = (width - content_width) / 2.0;

        if let Some(icon) = icon_node {
            offset += icon.size().width + spacing;
            let y = padding.top + (height - icon.size().height) / 2.0;
            children.push(icon.move_to(iced::Point::new(x, y)));
        }

        if let Some(label) = label_node {
            let y = padding.top + (height - label.size().height) / 2.0;
            children.push(label.move_to(iced::Point::new(x + offset, y)));
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
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        let state = tree.state.downcast_mut::<State>();

                        state.is_pressed = true;

                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = &self.on_press {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press.resolve());
                        }

                        shell.capture_event();
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            _ => {}
        }

        let current_status = if self.on_press.is_none() {
            Status::Disabled
        } else if cursor.is_over(layout.bounds()) {
            let state = tree.state.downcast_ref::<State>();

            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Idle
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let status = self.status.unwrap();
        let bounds = layout.bounds();
        let mut children = layout.children();

        let elevation = self.style.elevation.elevation(status);
        let style = self
            .style
            .state_style(status == Status::Disabled, self.selected);
        let corner_radius = if status == Status::Pressed {
            self.corner_style
                .pressed(&self.size, self.selected.unwrap_or_default())
        } else {
            self.corner_style
                .resting(&self.size, self.selected.unwrap_or_default())
        };

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

        let state_layer = self.style.state_layer(self.selected);
        let state_layer = match status {
            Status::Idle => Some(state_layer.idle),
            Status::Hovered => Some(state_layer.hovered),
            Status::Pressed => Some(state_layer.pressed),
            Status::Disabled => None,
        };

        state_layer.map(|color| {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: iced::Border::default().rounded(corner_radius),
                    ..Default::default()
                },
                color,
            );
        });
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

        if is_mouse_over && self.on_press.is_some() {
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
