use iced::{
    Border, Color, Element, Length, Padding, Point, Size,
    advanced::{Widget, layout::Node, mouse, widget::Tree},
};

use crate::{
    style::{DISABLED_STATE_LAYER_OPACITY, Elevation, StateLayer, shadow},
    theme::ColorScheme,
    widget::OnPress,
};

pub const MAX_CARD_BETWEEN_PADDING: f32 = 8.0;
mod constants {
    pub const DEFAULT_CORNER_RADIUS: f32 = 12.0;
    pub const DEFAULT_PADDING: f32 = 16.0;
    pub const DEFAULT_OUTLINE_WIDTH: f32 = 1.0;
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

#[derive(Default)]
struct State {
    is_hovered: bool,
    is_pressed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub container_color: Color,
    pub border: Border,
    pub state_layer: StateLayer,
    pub shadow_color: Color,
    pub elevation: Elevation,
    pub container_color_disabled: Color,
}

impl Style {
    #[must_use]
    pub fn elevated(theme: &(impl ColorScheme + ?Sized)) -> Self {
        let container_color = theme.surface_container_low();
        Self {
            container_color,
            border: Border::default().rounded(constants::DEFAULT_CORNER_RADIUS),
            state_layer: StateLayer::new(theme.on_surface()),
            shadow_color: theme.shadow(),
            elevation: Elevation::Level1,
            container_color_disabled: container_color.scale_alpha(DISABLED_STATE_LAYER_OPACITY),
        }
    }

    #[must_use]
    pub fn filled(theme: &(impl ColorScheme + ?Sized)) -> Self {
        let container_color = theme.surface_container_highest();
        Self {
            container_color,
            border: Border::default().rounded(constants::DEFAULT_CORNER_RADIUS),
            state_layer: StateLayer::new(theme.on_surface()),
            shadow_color: theme.shadow(),
            elevation: Elevation::default(),
            container_color_disabled: container_color.scale_alpha(DISABLED_STATE_LAYER_OPACITY),
        }
    }

    #[must_use]
    pub fn outlined(theme: &(impl ColorScheme + ?Sized)) -> Self {
        let container_color = theme.surface();
        Self {
            container_color,
            border: Border::default()
                .rounded(constants::DEFAULT_CORNER_RADIUS)
                .width(constants::DEFAULT_OUTLINE_WIDTH)
                .color(theme.outline_variant()),
            state_layer: StateLayer::new(theme.on_surface()),
            shadow_color: theme.shadow(),
            elevation: Elevation::default(),
            container_color_disabled: container_color.scale_alpha(DISABLED_STATE_LAYER_OPACITY),
        }
    }
}

// TODO: Drag and drop
#[derive(Default)]
pub enum Interaction<'a, Message>
where
    Message: Clone,
{
    #[default]
    NonInteractive,
    Press(OnPress<'a, Message>),
    Disabled,
}

pub struct Card<'a, Message, Theme, Renderer>
where
    Message: Clone,
{
    content: Element<'a, Message, Theme, Renderer>,
    interaction: Interaction<'a, Message>,
    padding: Padding,
    style: Style,
    width: Length,
    height: Length,
}

impl<'a, Message, Theme, Renderer> Card<'a, Message, Theme, Renderer>
where
    Message: Clone,
{
    #[must_use]
    pub fn new(style: Style, content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            interaction: Interaction::default(),
            padding: constants::DEFAULT_PADDING.into(),
            style,
            width: Length::Shrink,
            height: Length::Shrink,
        }
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

    #[must_use]
    pub fn interaction(mut self, interaction: Interaction<'a, Message>) -> Self {
        self.interaction = interaction;
        self
    }

    #[must_use]
    pub fn interaction_maybe(self, maybe_interaction: Option<Interaction<'a, Message>>) -> Self {
        match maybe_interaction {
            Some(interaction) => self.interaction(interaction),
            None => self,
        }
    }

    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    #[must_use]
    pub fn width_maybe(self, maybe_width: Option<impl Into<Length>>) -> Self {
        match maybe_width {
            Some(width) => self.width(width),
            None => self,
        }
    }

    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    #[must_use]
    pub fn height_maybe(self, maybe_height: Option<impl Into<Length>>) -> Self {
        match maybe_height {
            Some(height) => self.height(height),
            None => self,
        }
    }

    #[must_use]
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    #[must_use]
    pub fn padding_maybe(self, maybe_padding: Option<Padding>) -> Self {
        match maybe_padding {
            Some(padding) => self.padding(padding),
            None => self,
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Card<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(State::default())
    }

    fn size(&self) -> iced::Size<Length> {
        iced::Size {
            width: self.width,
            height: self.height,
        }
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
        let wrapper_limits = limits.width(self.width).height(self.height);
        let content_limits = wrapper_limits.shrink(self.padding);

        let content =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &content_limits);

        let content_size = content.size();

        let desired_size = Size::new(
            content_size.width + self.padding.left + self.padding.right,
            content_size.height + self.padding.top + self.padding.bottom,
        );

        let size = wrapper_limits.resolve(self.width, self.height, desired_size);

        Node::with_children(
            size,
            vec![content.move_to(Point::new(self.padding.left, self.padding.top))],
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            iced::advanced::renderer::Quad {
                bounds: layout.bounds(),
                border: self.style.border,
                shadow: shadow(self.style.shadow_color, self.style.elevation),
                ..Default::default()
            },
            if matches!(self.interaction, Interaction::Disabled) {
                self.style.container_color_disabled
            } else {
                self.style.container_color
            },
        );

        let state_layer_color = match self.interaction {
            Interaction::Press(_) => {
                let state = tree.state.downcast_ref::<State>();
                if state.is_pressed {
                    self.style.state_layer.pressed
                } else if state.is_hovered {
                    self.style.state_layer.hovered
                } else {
                    self.style.state_layer.idle
                }
            }
            _ => self.style.state_layer.idle,
        };

        if state_layer_color != Color::TRANSPARENT {
            renderer.fill_quad(
                iced::advanced::renderer::Quad {
                    bounds: layout.bounds(),
                    border: self.style.border.width(0.0),
                    ..Default::default()
                },
                state_layer_color,
            );
        }

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let interaction = self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        );

        if interaction != mouse::Interaction::None {
            return interaction;
        }

        if cursor.is_over(layout.bounds()) && matches!(self.interaction, Interaction::Press(_)) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if shell.is_event_captured() || !matches!(self.interaction, Interaction::Press(_)) {
            return;
        }

        let state = tree.state.downcast_mut::<State>();
        let is_over = cursor.is_over(layout.bounds());

        match event {
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if is_over => {
                shell.capture_event();
                state.is_pressed = true;
                shell.invalidate_layout();
                shell.request_redraw();
            }

            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if state.is_pressed =>
            {
                shell.capture_event();
                if is_over {
                    if let Interaction::Press(on_press) = &self.interaction {
                        shell.publish(on_press.resolve());
                    }
                }
                state.is_pressed = false;
                shell.invalidate_layout();
                shell.request_redraw();
            }

            _ => {}
        }

        if state.is_hovered != is_over {
            state.is_hovered = is_over;

            if !is_over {
                state.is_pressed = false;
            }

            shell.invalidate_layout();
            shell.request_redraw();
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Card<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::Renderer,
{
    fn from(value: Card<'a, Message, Theme, Renderer>) -> Self {
        Element::new(value)
    }
}
