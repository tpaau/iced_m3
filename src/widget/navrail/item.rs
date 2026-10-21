pub const INDICATOR_SMALL_HEIGHT: f32 = 32.0;
pub const INDICATOR_SMALL_WIDTH: f32 = 56.0;
pub const ITEM_SMALL_ICON_LABEL_SPACE: f32 = 4.0;
pub const ITEM_SMALL_LABEL_SIZE: f32 = 12.0;
pub const ITEM_SMALL_LABEL_LINE_HEIGHT: f32 = 16.0;
pub const ITEM_SMALL_TOTAL_HEIGHT: f32 =
    INDICATOR_SMALL_HEIGHT + ITEM_SMALL_ICON_LABEL_SPACE + ITEM_SMALL_LABEL_LINE_HEIGHT;

pub const INDICATOR_LARGE_HEIGHT: f32 = 56.0;
pub const INDICATOR_LARGE_PADDING: f32 = 16.0;
pub const ITEM_LARGE_ICON_LABEL_SPACE: f32 = 8.0;
pub const ITEM_LARGE_LABEL_SIZE: f32 = 14.0;
pub const ITEM_LARGE_LABEL_LINE_HEIGHT: f32 = 20.0;

pub const ICON_SIZE: f32 = 24.0;

use std::time::Instant;

use iced::{
    Border, Element, Event, Font, Length, Pixels, Point, Rectangle, Size,
    advanced::{
        Widget,
        layout::Node,
        mouse,
        renderer::Quad,
        text::{self, Paragraph},
        widget::Tree,
    },
    touch,
};
use iced_widget::{core::Svg, space};

use crate::{
    animation::motion::{self, Spring, SpringValue, default_effects, fast_effects},
    widget::{self, hybrid_icon::Icon},
};

pub use crate::widget::internal::navigation_common::NavigationItemContent as Content;
pub use crate::widget::internal::navigation_common::NavigationItemStyle as Style;
pub use crate::widget::internal::navigation_common::{Colors, State};

pub struct Item<'a, Message, Theme, Renderer>
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
    active_transition_spring: Option<Spring>,
    color_spring: Option<Spring>,

    // Cache
    label_paragraph: Option<Renderer::Paragraph>,
    icon_paragraph: Option<Renderer::Paragraph>,
    badge: Option<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Item<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
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
        let badge = content.badge.as_ref().map(|badge| {
            widget::badge(
                style.badge_style,
                space().width(ICON_SIZE).height(ICON_SIZE),
            )
            .label_maybe(badge.label.clone())
            .into()
        });

        Self {
            expanded,
            style,
            content,
            label_font,
            active,
            active_transition_spring: None,
            color_spring: None,
            label_paragraph: None,
            icon_paragraph: None,
            badge,
        }
    }

    pub fn motion_scheme_maybe(mut self, scheme: Option<motion::Scheme>) -> Self {
        if let Some(scheme) = scheme {
            self.color_spring = Some(fast_effects(scheme));
            self.active_transition_spring = Some(default_effects(scheme));
        }
        self
    }

    #[must_use]
    pub fn active_transition_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.active_transition_spring = spring;
        self
    }

    #[must_use]
    pub fn color_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.color_spring = spring;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Item<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn size(&self) -> Size<Length> {
        match self.expanded {
            true => Size {
                width: Length::Shrink,
                height: Length::Fixed(INDICATOR_LARGE_HEIGHT),
            },
            false => Size {
                width: Length::Fixed(INDICATOR_SMALL_WIDTH),
                height: Length::Fixed(ITEM_SMALL_TOTAL_HEIGHT),
            },
        }
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        let now = Instant::now();

        let content_color = match self.active {
            true => self.style.content_active,
            false => self.style.content_inactive,
        };
        let colors = Colors {
            state_layer: self.style.state_layer.idle,
            content: content_color,
        };
        let state = State {
            is_pressed: false,
            is_hovered: false,
            active_transition_spring: SpringValue::new(
                self.active as usize as f32,
                self.active_transition_spring
                    .unwrap_or(default_effects(motion::Scheme::default())),
                now,
            ),
            color_spring: SpringValue::new(
                colors,
                self.color_spring
                    .unwrap_or(fast_effects(motion::Scheme::default())),
                now,
            ),
        };

        iced::advanced::widget::tree::State::new(state)
    }

    fn children(&self) -> Vec<Tree> {
        match &self.badge {
            Some(badge) => vec![Tree::new(badge.as_widget())],
            None => Vec::new(),
        }
    }

    fn diff(&self, tree: &mut Tree) {
        let children = match &self.badge {
            Some(badge) => vec![badge.as_widget()],
            None => Vec::new(),
        };

        tree.diff_children(&children);
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let icon = match self.active {
            true => &self.content.icon_active,
            false => &self.content.icon_inactive,
        };
        if let Icon::Text { text, font } = icon {
            let icon_paragraph = <Renderer as text::Renderer>::Paragraph::with_text(text::Text {
                content: &text,
                bounds: Size::INFINITE,
                size: Pixels(ICON_SIZE),
                line_height: text::LineHeight::Absolute(Pixels(ICON_SIZE)),
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

        let icon_bounds = Size::new(ICON_SIZE, ICON_SIZE);
        let icon_node = Node::new(icon_bounds);

        let badge_node = self.badge.as_mut().map(|badge| {
            badge
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, limits)
        });

        match self.expanded {
            true => {
                let label_paragraph =
                    <Renderer as text::Renderer>::Paragraph::with_text(text::Text {
                        content: &self.content.label,
                        bounds: Size::INFINITE,
                        size: Pixels(ITEM_LARGE_LABEL_SIZE),
                        line_height: text::LineHeight::Absolute(Pixels(
                            ITEM_LARGE_LABEL_LINE_HEIGHT,
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

                let indicator_bounds = Size::new(
                    label_bounds.width
                        + icon_bounds.width
                        + 2.0 * INDICATOR_LARGE_PADDING
                        + ITEM_LARGE_ICON_LABEL_SPACE,
                    INDICATOR_LARGE_HEIGHT,
                );

                let icon_position = Point::new(
                    INDICATOR_LARGE_PADDING,
                    (indicator_bounds.height - ICON_SIZE) / 2.0,
                );
                let icon_node = icon_node.move_to(icon_position);

                let label_node = Node::new(label_bounds).move_to(Point::new(
                    INDICATOR_LARGE_PADDING + ICON_SIZE + ITEM_LARGE_ICON_LABEL_SPACE,
                    (indicator_bounds.height - label_bounds.height) / 2.0,
                ));

                let mut children = vec![icon_node, label_node];
                badge_node.map(|node| {
                    let position = Point::new(
                        icon_position.x,
                        icon_position.y + icon_bounds.height - node.bounds().height,
                    );
                    children.push(node.move_to(position));
                });

                Node::with_children(indicator_bounds, children)
            }
            false => {
                let label_paragraph =
                    <Renderer as text::Renderer>::Paragraph::with_text(text::Text {
                        content: &self.content.label,
                        bounds: Size::INFINITE,
                        size: Pixels(ITEM_SMALL_LABEL_SIZE),
                        line_height: text::LineHeight::Absolute(Pixels(
                            ITEM_SMALL_LABEL_LINE_HEIGHT,
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

                let parent_bounds = Size::new(INDICATOR_SMALL_WIDTH, ITEM_SMALL_TOTAL_HEIGHT);

                let indicator_node = Node::new(indicator_bounds);

                let icon_position = Point::new(
                    (indicator_bounds.width - icon_bounds.width) / 2.0,
                    (indicator_bounds.height - icon_bounds.height) / 2.0,
                );
                let icon_node = icon_node.move_to(icon_position);

                let label_node = Node::new(indicator_bounds).move_to(Point::new(
                    (indicator_bounds.width - label_bounds.width) / 2.0,
                    parent_bounds.height - label_bounds.height,
                ));

                let mut children = vec![indicator_node, icon_node, label_node];
                badge_node.map(|node| {
                    let position = Point::new(
                        icon_position.x,
                        icon_position.y + icon_bounds.height - node.bounds().height,
                    );
                    children.push(node.move_to(position));
                });

                Node::with_children(parent_bounds, children)
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let (indicator_bounds, icon_bounds, label_bounds, badge_node) = match self.expanded {
            true => (
                layout.bounds(),
                layout.child(0).bounds(),
                layout.child(1).bounds(),
                self.badge.as_ref().map(|_| layout.child(2)),
            ),
            false => (
                layout.child(0).bounds(),
                layout.child(1).bounds(),
                layout.child(2).bounds(),
                self.badge.as_ref().map(|_| layout.child(3)),
            ),
        };

        let state = tree.state.downcast_ref::<State>();
        let width = state.active_transition_spring.value() * indicator_bounds.width;
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
                    .scale_alpha(state.active_transition_spring.value()),
            );
        }

        let content_color = state.color_spring.value().content;
        if let Some(icon) = &self.icon_paragraph {
            renderer.fill_paragraph(icon, icon_bounds.position(), content_color, icon_bounds);
        } else {
            let icon = match self.active {
                true => &self.content.icon_active,
                false => &self.content.icon_inactive,
            };
            if let Icon::Svg(handle) = icon.clone() {
                renderer.draw_svg(
                    Svg::new(handle).color(content_color),
                    icon_bounds,
                    icon_bounds,
                );
            }
        }

        if let Some(label) = &self.label_paragraph {
            renderer.fill_paragraph(label, label_bounds.position(), content_color, label_bounds);
        }

        if let Some(badge) = &self.badge {
            badge.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                badge_node.unwrap(),
                cursor,
                viewport,
            );
        }

        renderer.fill_quad(
            Quad {
                bounds: indicator_bounds,
                border: Border::default().rounded(f32::MAX),
                ..Default::default()
            },
            state.color_spring.value().state_layer,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
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
                if is_hovered && state.is_pressed {
                    shell.publish(self.content.on_press.clone());
                }
                state.is_pressed = false;
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

        if let Some(spring) = self.active_transition_spring
            && state.active_transition_spring.spring.model != spring
        {
            state.active_transition_spring.spring.model = spring;
        }
        if let Some(spring) = self.color_spring
            && state.color_spring.spring.model != spring
        {
            state.color_spring.spring.model = spring;
        }

        let now = Instant::now();
        let state_layer_color = match is_hovered {
            true => match state.is_pressed {
                true => self.style.state_layer.pressed,
                false => self.style.state_layer.hovered,
            },
            false => self.style.state_layer.idle,
        };
        let content_color = match self.active {
            true => self.style.content_active,
            false => self.style.content_inactive,
        };
        let colors = Colors {
            state_layer: state_layer_color,
            content: content_color,
        };
        state.color_spring.set_target(colors, now);
        state
            .active_transition_spring
            .set_target(self.active as usize as f32, now);

        if !state.active_transition_spring.is_at_rest() || !state.color_spring.is_at_rest() {
            shell.request_redraw();
        }
        state.active_transition_spring.step(now);
        state.color_spring.step(now);
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
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

impl<'a, Message, Theme, Renderer> From<Item<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn from(value: Item<'a, Message, Theme, Renderer>) -> Self {
        Element::new(value)
    }
}
