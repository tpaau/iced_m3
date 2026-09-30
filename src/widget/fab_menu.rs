use std::time::Instant;

use iced::{
    Border, Color, Element, Event, Font, Length, Point, Radians, Rectangle, Size,
    advanced::{
        Widget,
        layout::{Limits, Node},
        mouse, overlay,
        renderer::Quad,
        svg::{self},
        text,
        widget::{Tree, tree},
    },
    border::Radius,
    touch,
};

use crate::{
    animation::{
        Interpolable,
        motion::{self, Spring, SpringValue, fast_effects, fast_spatial},
    },
    style::StateLayer,
    theme::{Accent, ColorScheme},
    widget::{
        self, OnPress,
        button::{self, CornerStyle},
        common_icons,
        fab::{self},
        hybrid_icon::Icon,
    },
};

pub use crate::widget::advanced::drop_down_menu::Placement;

const BUTTON_SIZE: fab::Size = fab::Size::Regular;
const BUTTON_TRIGGER_BETWEEN_SPACE: f32 = 8.0;
const BUTTON_SPACING: f32 = 4.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateStyle {
    pub container_color: Color,
    pub icon_color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub expanded: StateStyle,
    pub collapsed: StateStyle,
    pub state_layer_expanded: StateLayer,
    pub state_layer_collapsed: StateLayer,
    pub item_style: button::Style,
    pub accent: Accent,
}

impl Style {
    pub fn new(theme: &(impl ColorScheme + ?Sized), accent: Accent) -> Self {
        Self {
            expanded: StateStyle {
                container_color: accent.color(theme),
                icon_color: accent.on_color(theme),
            },
            collapsed: StateStyle {
                container_color: accent.color_container(theme),
                icon_color: accent.on_color_container(theme),
            },
            state_layer_expanded: StateLayer::new(accent.on_color(theme)),
            state_layer_collapsed: StateLayer::new(accent.on_color_container(theme)),
            item_style: button::Style::fab_tonal(theme, accent),
            accent,
        }
    }
}

impl Interpolable for StateStyle {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Self {
            container_color: self.container_color.interpolate(other.container_color, t),
            icon_color: self.icon_color.interpolate(other.icon_color, t),
        }
    }
}

pub struct Item<'a, Message>
where
    Message: Clone,
{
    pub on_press: OnPress<'a, Message>,
    pub label: text::Fragment<'a>,
    pub icon: Icon<'a>,
}

pub struct FABMenu<'a, Message, Theme, Renderer>
where
    Message: Clone,
{
    style: Style,
    size: Option<fab::Size>,
    spatial_spring: Option<Spring>,
    effects_spring: Option<Spring>,
    items: Vec<Element<'a, Message, Theme, Renderer>>,

    // Cache
    style_checked: bool,
}

impl<'a, Message, Theme, Renderer> FABMenu<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    #[must_use]
    pub fn new<I>(style: Style, items: I, label_font: Option<Font>) -> Self
    where
        I: IntoIterator<Item = Item<'a, Message>>,
    {
        let items = items
            .into_iter()
            .map(|i| {
                widget::fab(
                    style.item_style,
                    widget::fab::Content::Extended {
                        icon: i.icon,
                        label: i.label,
                    },
                    i.on_press,
                )
                .corner_style(CornerStyle::Rounded)
                .label_font_maybe(label_font)
                .size(BUTTON_SIZE)
                .into()
            })
            .collect();

        Self {
            size: None,
            items,
            style,
            spatial_spring: None,
            effects_spring: None,
            style_checked: false,
        }
    }

    #[must_use]
    pub fn size(mut self, size: fab::Size) -> Self {
        self.size = Some(size);
        self
    }

    #[must_use]
    pub fn size_maybe(mut self, size: Option<fab::Size>) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn motion_scheme(mut self, scheme: motion::Scheme) -> Self {
        self.effects_spring = Some(fast_effects(scheme));
        self.spatial_spring = Some(fast_spatial(scheme));
        self
    }

    #[must_use]
    pub fn spatial_spring(mut self, spring: Spring) -> Self {
        self.spatial_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn spatial_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.spatial_spring = spring;
        self
    }

    #[must_use]
    pub fn effects_spring(mut self, spring: Spring) -> Self {
        self.effects_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn effects_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.effects_spring = spring;
        self
    }
}

struct State {
    is_expanded: bool,
    is_pressed: bool,
    style_spring: SpringValue<StateStyle>,
    icon_rotation_spring: SpringValue<Radians>,
    corner_radius_spring: SpringValue<Radius>,
    icon_handle: svg::Handle,
    last_style: Style,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for FABMenu<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::Renderer + iced::advanced::svg::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let now = Instant::now();
        let spatial_spring = self
            .spatial_spring
            .unwrap_or(fast_effects(motion::Scheme::default()));
        let effects_spring = self
            .effects_spring
            .unwrap_or(fast_effects(motion::Scheme::default()));
        let state = State {
            is_expanded: false,
            is_pressed: false,
            style_spring: SpringValue::new(self.style.collapsed, effects_spring, now),
            icon_rotation_spring: SpringValue::new(Radians(0.0), spatial_spring, now),
            corner_radius_spring: SpringValue::new(
                self.size.unwrap_or_default().rounding(),
                spatial_spring,
                now,
            ),
            icon_handle: svg::Handle::from_memory(common_icons::ADD),
            last_style: self.style,
        };

        tree::State::new(state)
    }

    fn size(&self) -> Size<Length> {
        let size = self.size.unwrap_or_default().container_height();

        Size {
            width: Length::Fixed(size),
            height: Length::Fixed(size),
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.items.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.items);
    }

    fn layout(
        &mut self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let size = self.size.unwrap_or_default();
        let container_size = size.container_height();
        let intrinsic_size = Size::new(container_size, container_size);
        let container_bounds =
            limits.resolve(intrinsic_size.width, intrinsic_size.height, intrinsic_size);

        let icon_size = size.icon_size();
        let icon_bounds = Size::new(icon_size, icon_size);
        let icon_node = Node::new(icon_bounds).move_to(Point::new(
            (container_bounds.width - icon_bounds.width) / 2.0,
            (container_bounds.height - icon_bounds.height) / 2.0,
        ));

        Node::with_children(container_bounds, vec![icon_node])
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let style = state.style_spring.value();
        let corner_radius = state.corner_radius_spring.value();
        let icon_rotation = state.icon_rotation_spring.value();

        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border::default().rounded(corner_radius),
                ..Default::default()
            },
            style.container_color,
        );

        let bounds = layout.child(0).bounds();
        renderer.draw_svg(
            svg::Svg::new(state.icon_handle.clone())
                .rotation(icon_rotation)
                .color(style.icon_color),
            bounds,
            bounds,
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
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    state.is_pressed = true;
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if state.is_pressed {
                    state.is_pressed = false;

                    if cursor.is_over(layout.bounds()) {
                        state.is_expanded = !state.is_expanded;
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                state.is_pressed = false;
            }
            _ => {}
        }

        if let Some(spring) = self.spatial_spring {
            if state.icon_rotation_spring.spring != spring {
                state.icon_rotation_spring.spring = spring;
            }
            if state.corner_radius_spring.spring != spring {
                state.corner_radius_spring.spring = spring;
            }
        }
        if let Some(spring) = self.effects_spring
            && state.style_spring.spring != spring
        {
            state.style_spring.spring = spring;
        }

        let now = Instant::now();
        let (style, corner_radius, icon_rotation) = match state.is_expanded {
            true => {
                let bounds = layout.bounds();
                let radius = bounds.width.min(bounds.height) / 2.0;
                (self.style.expanded, radius.into(), Radians::PI / 4.0)
            }
            false => (
                self.style.collapsed,
                self.size.unwrap_or_default().rounding(),
                Radians(0.0),
            ),
        };
        if !self.style_checked && state.last_style != self.style {
            state.style_spring.reset(style, now);
            state.last_style = self.style;
            self.style_checked = true;
        } else if state.style_spring.to != style {
            state.style_spring.set_target(style, now);
        }
        if state.corner_radius_spring.to != corner_radius {
            state.corner_radius_spring.set_target(corner_radius, now);
        }
        if state.icon_rotation_spring.to != icon_rotation {
            state.icon_rotation_spring.set_target(icon_rotation, now);
        }

        if !state.style_spring.is_at_rest()
            || !state.corner_radius_spring.is_at_rest()
            || !state.icon_rotation_spring.is_at_rest()
        {
            shell.request_redraw();
        }

        state.style_spring.step(now);
        state.corner_radius_spring.step(now);
        state.icon_rotation_spring.step(now);
    }

    fn mouse_interaction(
        &self,
        _tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'b>,
        _renderer: &Renderer,
        _viewport: &Rectangle,
        _translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_ref::<State>();
        state
            .is_expanded
            .then_some(overlay::Element::new(Box::new(Overlay {
                base_bounds: layout.bounds(),
                tree,
                items: &mut self.items,
            })))
    }
}

impl<'a, Message, Theme, Renderer> From<FABMenu<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::svg::Renderer,
{
    fn from(menu: FABMenu<'a, Message, Theme, Renderer>) -> Self {
        Element::new(menu)
    }
}

struct Overlay<'a, 'b, Message, Theme, Renderer> {
    base_bounds: Rectangle,
    tree: &'b mut Tree,
    items: &'b mut Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, 'b, Message, Theme, Renderer> iced::advanced::Overlay<Message, Theme, Renderer>
    for Overlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: iced::Size) -> iced::advanced::layout::Node {
        let item_height = BUTTON_SIZE.container_height();
        let children: Vec<_> = self
            .items
            .iter_mut()
            .enumerate()
            .map(|(i, item)| {
                let item = item.as_widget_mut().layout(
                    &mut self.tree.children[i],
                    renderer,
                    &Limits::new(Size::ZERO, bounds),
                );

                let y = -item_height * (i + 1) as f32
                    - BUTTON_TRIGGER_BETWEEN_SPACE
                    - BUTTON_SPACING * i as f32;
                let x = self.base_bounds.width - item.size().width;

                item.move_to(Point::new(x, y))
            })
            .collect();

        Node::with_children(self.base_bounds.size(), children).move_to(self.base_bounds.position())
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
    ) {
        let viewport = iced::Rectangle::new(iced::Point::ORIGIN, layout.bounds().size());
        for (i, item) in self.items.iter().enumerate() {
            item.as_widget().draw(
                &self.tree.children[i],
                renderer,
                theme,
                style,
                layout.child(i),
                cursor,
                &viewport,
            );
        }
    }

    fn update(
        &mut self,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
    ) {
        let viewport = iced::Rectangle::new(iced::Point::ORIGIN, layout.bounds().size());
        for (i, item) in self.items.iter_mut().enumerate() {
            item.as_widget_mut().update(
                &mut self.tree.children[i],
                event,
                layout.child(i),
                cursor,
                renderer,
                clipboard,
                shell,
                &viewport,
            );
        }

        let should_close = match event {
            Event::Mouse(iced::mouse::Event::ButtonPressed(_)) => {
                !shell.is_event_captured() && !cursor.is_over(self.base_bounds)
            }
            Event::Mouse(iced::mouse::Event::ButtonReleased(_)) => {
                !cursor.is_over(self.base_bounds)
            }
            _ => false,
        };

        if should_close {
            let state = self.tree.state.downcast_mut::<State>();
            state.is_expanded = false;
            shell.invalidate_widgets();
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        for (i, _) in self.items.iter().enumerate() {
            if cursor.is_over(layout.child(i).bounds()) {
                return iced::mouse::Interaction::Pointer;
            }
        }

        iced::mouse::Interaction::None
    }
}
