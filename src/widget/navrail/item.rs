pub const INDICATOR_SMALL_HEIGHT: f32 = 32.0;
pub const INDICATOR_SMALL_WIDTH: f32 = 56.0;
pub const INDICATOR_SMALL_ICON_LABEL_SPACE: f32 = 4.0;
pub const INDICATOR_SMALL_LABEL_SIZE: f32 = 12.0;
pub const INDICATOR_SMALL_LABEL_LINE_HEIGHT: f32 = 16.0;
pub const INDICATOR_SMALL_TOTAL_HEIGHT: f32 =
    INDICATOR_SMALL_HEIGHT + INDICATOR_SMALL_ICON_LABEL_SPACE + INDICATOR_SMALL_LABEL_LINE_HEIGHT;

pub const INDICATOR_LARGE_HEIGHT: f32 = 56.0;
pub const INDICATOR_LARGE_PADDING: f32 = 16.0;
pub const INDICATOR_LARGE_ICON_LABEL_SPACE: f32 = 8.0;
pub const INDICATOR_LARGE_LABEL_SIZE: f32 = 14.0;
pub const INDICATOR_LARGE_LABEL_LINE_HEIGHT: f32 = 20.0;

pub const INDICATOR_ICON_SIZE: f32 = 24.0;

use std::time::Instant;

use iced::{
    Border, Color, Element, Event, Font, Length, Pixels, Point, Rectangle, Size,
    advanced::{
        Widget,
        layout::Node,
        mouse,
        renderer::Quad,
        text::{self, Paragraph},
    },
    touch,
};
use iced_widget::core::Svg;

use crate::{
    animation::motion::{self, SpringMotion, ValueMotion, default_effects, fast_effects},
    style::StateLayer,
    theme::ColorScheme,
    widget::{Badge, OnPress, hybrid_icon::Icon},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateStyle {
    pub icon_color: Color,
    pub label_color: Color,
}

impl StateStyle {
    pub fn active(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            icon_color: theme.secondary(),
            label_color: theme.secondary(),
        }
    }

    pub fn inactive(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            icon_color: theme.on_surface_variant(),
            label_color: theme.on_surface_variant(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub container_color: Color,
    pub state_layer: StateLayer,
    pub active: StateStyle,
    pub inactive: StateStyle,
}

impl Style {
    pub fn new(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            container_color: theme.secondary_container(),
            state_layer: StateLayer::new(theme.on_secondary_container()),
            active: StateStyle::active(theme),
            inactive: StateStyle::inactive(theme),
        }
    }
}

pub struct Content<'a, Message>
where
    Message: Clone,
{
    pub icon_active: Icon<'a>,
    pub icon_inactive: Icon<'a>,
    pub badge: Option<Badge<'a>>,
    pub label: text::Fragment<'a>,
    pub on_press: OnPress<'a, Message>,
}

pub struct Item<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    expanded: bool,
    style: Style,
    content: Content<'a, Message>,
    label_font: Option<Font>,
    active: bool,

    // Cache
    label_paragraph: Option<Renderer::Paragraph>,
    icon_paragraph: Option<Renderer::Paragraph>,
}

impl<'a, Message, Renderer> Item<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    #[must_use]
    pub fn new(
        style: Style,
        content: Content<'a, Message>,
        expanded: bool,
        label_font: Option<Font>,
        active: bool,
    ) -> Self {
        Self {
            expanded,
            style,
            content,
            label_font,
            active,
            label_paragraph: None,
            icon_paragraph: None,
        }
    }
}

struct State {
    is_pressed: bool,
    is_hovered: bool,
    active_transition_spring: SpringMotion,
    state_layer_color_spring: ValueMotion<Color>,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Item<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        let now = Instant::now();
        let state = State {
            is_pressed: false,
            is_hovered: false,
            active_transition_spring: SpringMotion::new(
                default_effects(motion::Scheme::default()),
                self.active as usize as f32,
                now,
            ),
            state_layer_color_spring: ValueMotion::new(
                self.style.state_layer.idle,
                self.style.state_layer.idle,
                fast_effects(motion::Scheme::default()),
                now,
            ),
        };

        iced::advanced::widget::tree::State::new(state)
    }

    fn size(&self) -> Size<Length> {
        match self.expanded {
            true => Size {
                width: Length::Shrink,
                height: Length::Fixed(INDICATOR_LARGE_HEIGHT),
            },
            false => Size {
                width: Length::Fixed(INDICATOR_SMALL_WIDTH),
                height: Length::Fixed(INDICATOR_SMALL_TOTAL_HEIGHT),
            },
        }
    }

    fn layout(
        &mut self,
        _tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        _limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let icon = match self.active {
            true => &self.content.icon_active,
            false => &self.content.icon_inactive,
        };
        if let Icon::Text { text, font } = icon {
            let icon_paragraph = <Renderer as text::Renderer>::Paragraph::with_text(text::Text {
                content: &text,
                bounds: Size::INFINITE,
                size: Pixels(INDICATOR_ICON_SIZE),
                line_height: text::LineHeight::Absolute(Pixels(INDICATOR_ICON_SIZE)),
                font: font
                    .map(Renderer::Font::from)
                    .unwrap_or_else(|| renderer.default_font()),
                shaping: text::Shaping::Advanced,
                wrapping: text::Wrapping::None,
                align_x: text::Alignment::Left,
                align_y: iced::alignment::Vertical::Center,
            });
            self.icon_paragraph = Some(icon_paragraph);
        }

        let icon_bounds = Size::new(INDICATOR_ICON_SIZE, INDICATOR_ICON_SIZE);

        match self.expanded {
            true => {
                let label_paragraph =
                    <Renderer as text::Renderer>::Paragraph::with_text(text::Text {
                        content: &self.content.label,
                        bounds: Size::INFINITE,
                        size: Pixels(INDICATOR_LARGE_LABEL_SIZE),
                        line_height: text::LineHeight::Absolute(Pixels(
                            INDICATOR_LARGE_LABEL_LINE_HEIGHT,
                        )),
                        font: self
                            .label_font
                            .map(Renderer::Font::from)
                            .unwrap_or_else(|| renderer.default_font()),
                        shaping: text::Shaping::Advanced,
                        wrapping: text::Wrapping::None,
                        align_x: text::Alignment::Left,
                        align_y: iced::alignment::Vertical::Center,
                    });

                let label_bounds = label_paragraph.min_bounds();
                self.label_paragraph = Some(label_paragraph);

                let icon_bounds = Size::new(INDICATOR_ICON_SIZE, INDICATOR_ICON_SIZE);

                let indicator_bounds = Size::new(
                    label_bounds.width
                        + icon_bounds.width
                        + 2.0 * INDICATOR_LARGE_PADDING
                        + INDICATOR_LARGE_ICON_LABEL_SPACE,
                    INDICATOR_LARGE_HEIGHT,
                );

                let icon_node = Node::new(icon_bounds).move_to(Point::new(
                    INDICATOR_LARGE_PADDING,
                    (indicator_bounds.height - INDICATOR_ICON_SIZE) / 2.0,
                ));

                let label_node = Node::new(label_bounds).move_to(Point::new(
                    INDICATOR_LARGE_PADDING
                        + INDICATOR_ICON_SIZE
                        + INDICATOR_LARGE_ICON_LABEL_SPACE,
                    (indicator_bounds.height - label_bounds.height) / 2.0,
                ));

                Node::with_children(indicator_bounds, vec![icon_node, label_node])
            }
            false => {
                let label_paragraph =
                    <Renderer as text::Renderer>::Paragraph::with_text(text::Text {
                        content: &self.content.label,
                        bounds: Size::INFINITE,
                        size: Pixels(INDICATOR_SMALL_LABEL_SIZE),
                        line_height: text::LineHeight::Absolute(Pixels(
                            INDICATOR_SMALL_LABEL_LINE_HEIGHT,
                        )),
                        font: self
                            .label_font
                            .map(Renderer::Font::from)
                            .unwrap_or_else(|| renderer.default_font()),
                        shaping: text::Shaping::Advanced,
                        wrapping: text::Wrapping::None,
                        align_x: text::Alignment::Left,
                        align_y: iced::alignment::Vertical::Center,
                    });

                let label_bounds = label_paragraph.min_bounds();
                self.label_paragraph = Some(label_paragraph);

                let indicator_bounds = Size::new(INDICATOR_SMALL_WIDTH, INDICATOR_SMALL_HEIGHT);

                let parent_bounds = Size::new(INDICATOR_SMALL_WIDTH, INDICATOR_SMALL_TOTAL_HEIGHT);

                let indicator_node = Node::new(indicator_bounds);

                let icon_node = Node::new(icon_bounds).move_to(Point::new(
                    (indicator_bounds.width - icon_bounds.width) / 2.0,
                    (indicator_bounds.height - icon_bounds.height) / 2.0,
                ));

                let label_node = Node::new(indicator_bounds).move_to(Point::new(
                    (indicator_bounds.width - label_bounds.width) / 2.0,
                    parent_bounds.height - label_bounds.height,
                ));

                Node::with_children(parent_bounds, vec![indicator_node, icon_node, label_node])
            }
        }
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let (indicator_bounds, icon_bounds, label_bounds) = match self.expanded {
            true => (
                layout.bounds(),
                layout.child(0).bounds(),
                layout.child(1).bounds(),
            ),
            false => (
                layout.child(0).bounds(),
                layout.child(1).bounds(),
                layout.child(2).bounds(),
            ),
        };
        let style = match self.active {
            true => self.style.active,
            false => self.style.inactive,
        };

        let state = tree.state.downcast_ref::<State>();
        let width = state.active_transition_spring.position * indicator_bounds.width;
        if width > 0.0 {
            let bounds = Rectangle {
                x: indicator_bounds.x + (indicator_bounds.width - width) / 2.0,
                y: indicator_bounds.y,
                width,
                height: indicator_bounds.height,
            };
            renderer.fill_quad(
                Quad {
                    bounds,
                    border: Border::default().rounded(f32::MAX),
                    ..Default::default()
                },
                self.style
                    .container_color
                    .scale_alpha(state.active_transition_spring.position),
            );
        }

        if let Some(icon) = &self.icon_paragraph {
            renderer.fill_paragraph(icon, icon_bounds.position(), style.icon_color, icon_bounds);
        } else {
            let icon = match self.active {
                true => &self.content.icon_active,
                false => &self.content.icon_inactive,
            };
            if let Icon::Svg(handle) = icon.clone() {
                renderer.draw_svg(
                    Svg::new(handle).color(style.icon_color),
                    icon_bounds,
                    icon_bounds,
                );
            }
        }

        if let Some(label) = &self.label_paragraph {
            renderer.fill_paragraph(
                label,
                label_bounds.position(),
                style.label_color,
                label_bounds,
            );
        }

        renderer.fill_quad(
            Quad {
                bounds: indicator_bounds,
                border: Border::default().rounded(f32::MAX),
                ..Default::default()
            },
            state.state_layer_color_spring.value(),
        );
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let indicator_bounds = match self.expanded {
            true => layout.bounds(),
            false => layout.child(0).bounds(),
        };
        let is_hovered = cursor.is_over(indicator_bounds);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if is_hovered {
                    state.is_pressed = true;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                state.is_pressed = false;
                if is_hovered {
                    shell.publish(self.content.on_press.resolve());
                }
                shell.request_redraw();
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                state.is_pressed = false;
                shell.request_redraw();
            }
            _ => {}
        }

        if state.is_hovered != is_hovered {
            state.is_hovered = is_hovered;
            shell.request_redraw();
        }

        let now = Instant::now();
        let state_layer_color = match is_hovered {
            true => match state.is_pressed {
                true => self.style.state_layer.pressed,
                false => self.style.state_layer.hovered,
            },
            false => self.style.state_layer.idle,
        };
        if state.state_layer_color_spring.to != state_layer_color {
            state
                .state_layer_color_spring
                .set_target(state_layer_color, now);
        }
        let target = self.active as usize as f32;
        if state.active_transition_spring.target != target {
            state.active_transition_spring.target = target;
        }
        if !state.active_transition_spring.is_at_rest()
            || !state.state_layer_color_spring.is_at_rest()
        {
            shell.request_redraw();
        }
        state.active_transition_spring.step(now);
        state.state_layer_color_spring.step(now);
    }

    fn mouse_interaction(
        &self,
        _tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        let indicator_bounds = match self.expanded {
            true => layout.bounds(),
            false => layout.child(0).bounds(),
        };

        match cursor.is_over(indicator_bounds) {
            true => iced::advanced::mouse::Interaction::Pointer,
            false => iced::advanced::mouse::Interaction::None,
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Item<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn from(value: Item<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}
