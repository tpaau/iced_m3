use iced::{
    Border, Color, Element, Length, Padding, Point, Size,
    advanced::{Widget, layout::Node, mouse, widget::Tree},
    touch,
};

use crate::{
    style::{DISABLED_STATE_LAYER_OPACITY, Elevation, StateLayer, shadow},
    theme::ColorScheme,
    widget::{Action, ripple::Ripple},
};

pub const MAX_CARD_BETWEEN_PADDING: f32 = 8.0;
pub(crate) mod constants {
    pub const DEFAULT_CORNER_RADIUS: f32 = 12.0;
    pub const DEFAULT_PADDING: f32 = 16.0;
    pub const DEFAULT_OUTLINE_WIDTH: f32 = 1.0;
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

#[derive(Default)]
struct State {
    is_pressed: bool,
    ripple: Ripple,
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
    Press(Action<'a, Message>),
    Disabled,
}

// TODO: Basic animation
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
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    #[must_use]
    pub fn padding_maybe(self, maybe_padding: Option<impl Into<Padding>>) -> Self {
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
    Renderer: iced_widget::renderer::geometry::Renderer,
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

        let enabled = matches!(self.interaction, Interaction::Press(_));
        tree.state.downcast_ref::<State>().ripple.draw(
            renderer,
            layout.bounds(),
            self.style.border.radius,
            self.style.state_layer,
            enabled,
        );

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

    // TODO: Better press tracking
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

        let is_interactive = matches!(self.interaction, Interaction::Press(_));
        let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<State>();
        state
            .ripple
            .handle(event, bounds, cursor, is_interactive, shell);

        if shell.is_event_captured() || !is_interactive {
            return;
        }

        let is_over = cursor.is_over(bounds);

        match event {
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if is_over => {
                shell.capture_event();
                state.is_pressed = true;
                shell.request_redraw();
            }
            iced::Event::Touch(touch::Event::FingerPressed { position, .. })
                if bounds.contains(*position) =>
            {
                shell.capture_event();
                state.is_pressed = true;
                shell.request_redraw();
            }

            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if state.is_pressed =>
            {
                shell.capture_event();
                if is_over && let Interaction::Press(on_press) = &self.interaction {
                    shell.publish(on_press.resolve());
                }
                state.is_pressed = false;
                shell.request_redraw();
            }
            iced::Event::Touch(touch::Event::FingerLifted { position, .. }) if state.is_pressed => {
                shell.capture_event();
                if bounds.contains(*position)
                    && let Interaction::Press(on_press) = &self.interaction
                {
                    shell.publish(on_press.resolve());
                }
                state.is_pressed = false;
                shell.request_redraw();
            }
            iced::Event::Touch(touch::Event::FingerLost { .. }) => {
                state.is_pressed = false;
            }

            _ => {}
        }

        if !is_over && !matches!(event, iced::Event::Touch(_)) {
            state.is_pressed = false;
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Card<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced_widget::renderer::geometry::Renderer,
{
    fn from(value: Card<'a, Message, Theme, Renderer>) -> Self {
        Element::new(value)
    }
}
