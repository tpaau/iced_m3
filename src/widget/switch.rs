use std::time::Instant;

use iced::{
    Border, Color, Element, Length, Point, Radians, Rectangle, Size,
    advanced::{Overlay, Widget, layout::Node, mouse, overlay, renderer::Quad},
};
use iced_widget::core::{Svg, svg::Handle};

use crate::{
    animation::{
        Interpolable, midpoint_distance,
        motion::{SpringMotion, ValueMotion, fast_effects, fast_spatial},
    },
    style::StateLayer,
    theme::ColorScheme,
    widget::common_icons,
};

const TRACK_SIZE: Size<f32> = Size {
    width: 52.0,
    height: 32.0,
};
const TRACK_DISABLED_OUTLINE_WIDTH: f32 = 2.0;
const HANDLE_SIZE_NO_ICON: f32 = 16.0;
const HANDLE_SIZE_WITH_ICON: f32 = 24.0;
const HANDLE_SIZE_PRESSED: f32 = 28.0;
const STATE_LAYER_SIZE: f32 = 40.0;
const ICON_SIZE: f32 = 16.0;

const HANDLE_DISABLED_SELECTED_OPACITY: f32 = 1.0;
const HANDLE_DISABLED_UNSELECTED_OPACITY: f32 = 0.38;
const ICON_DISABLED_OPACITY: f32 = 0.38;
const TRACK_DISABLED_OPACITY: f32 = 0.12;
const OUTLINE_DISABLED_OPACITY: f32 = 0.38;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IconMode {
    Never,
    #[default]
    WhenSelected,
    Always,
}

#[derive(Clone, PartialEq)]
pub struct StateStyle {
    pub track_color: Color,
    pub handle_color: Color,
    pub icon_color: Color,
    pub outline_width: f32,
    pub outline_color: Color,
}

impl Interpolable for StateStyle {
    fn interpolate(self, other: Self, t: f32) -> Self {
        Self {
            track_color: self.track_color.interpolate(other.track_color, t),
            handle_color: self.handle_color.interpolate(other.handle_color, t),
            icon_color: self.icon_color.interpolate(other.icon_color, t),
            outline_width: self.outline_width.interpolate(other.outline_width, t),
            outline_color: self.outline_color.interpolate(other.outline_color, t),
        }
    }
}

pub struct Style {
    pub state_layer_selected: StateLayer,
    pub state_later_unselected: StateLayer,
    pub selected: StateStyle,
    pub selected_disabled: StateStyle,
    pub unselected: StateStyle,
    pub unselected_disabled: StateStyle,
}

impl Style {
    fn new(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            state_layer_selected: StateLayer::new(theme.primary()),
            state_later_unselected: StateLayer::new(theme.on_surface()),
            selected: StateStyle {
                track_color: theme.primary(),
                handle_color: theme.on_primary(),
                icon_color: theme.on_primary_container(),
                outline_width: 0.0,
                outline_color: theme.outline(),
            },
            selected_disabled: StateStyle {
                track_color: theme.on_surface().scale_alpha(TRACK_DISABLED_OPACITY),
                handle_color: theme
                    .surface()
                    .scale_alpha(HANDLE_DISABLED_SELECTED_OPACITY),
                icon_color: theme.on_surface().scale_alpha(ICON_DISABLED_OPACITY),
                outline_width: 0.0,
                outline_color: theme.on_surface().scale_alpha(OUTLINE_DISABLED_OPACITY),
            },
            unselected: StateStyle {
                track_color: theme.surface_container_highest(),
                handle_color: theme.outline(),
                icon_color: theme.surface_container_highest(),
                outline_width: TRACK_DISABLED_OUTLINE_WIDTH,
                outline_color: theme.outline(),
            },
            unselected_disabled: StateStyle {
                track_color: theme.on_surface().scale_alpha(TRACK_DISABLED_OPACITY),
                handle_color: theme
                    .on_surface()
                    .scale_alpha(HANDLE_DISABLED_UNSELECTED_OPACITY),
                icon_color: theme
                    .surface_container_highest()
                    .scale_alpha(ICON_DISABLED_OPACITY),
                outline_width: TRACK_DISABLED_OUTLINE_WIDTH,
                outline_color: theme.on_surface().scale_alpha(OUTLINE_DISABLED_OPACITY),
            },
        }
    }

    fn state_layer(&self, selected: bool) -> &StateLayer {
        match selected {
            true => &self.state_layer_selected,
            false => &self.state_later_unselected,
        }
    }

    fn state(&self, selected: bool, enabled: bool) -> &StateStyle {
        match (selected, enabled) {
            (true, true) => &self.selected,
            (true, false) => &self.selected_disabled,
            (false, true) => &self.unselected,
            (false, false) => &self.unselected_disabled,
        }
    }
}

pub struct Switch<Message>
where
    Message: Clone,
{
    style: Style,
    icon_mode: IconMode,
    selected: bool,
    on_toggle: Option<Message>,
    expressive_animation: bool,
}

impl<Message> Switch<Message>
where
    Message: Clone,
{
    #[must_use]
    pub fn new(theme: &(impl ColorScheme + ?Sized), selected: bool) -> Self {
        Self {
            style: Style::new(theme),
            icon_mode: IconMode::default(),
            selected,
            on_toggle: None,
            expressive_animation: false,
        }
    }

    #[must_use]
    pub fn on_toggle(mut self, on_toggle: Message) -> Self {
        self.on_toggle = Some(on_toggle);
        self
    }

    #[must_use]
    pub fn on_toggle_maybe(mut self, on_toggle: Option<Message>) -> Self {
        self.on_toggle = on_toggle;
        self
    }

    #[must_use]
    pub fn icon_mode(mut self, icon_mode: IconMode) -> Self {
        self.icon_mode = icon_mode;
        self
    }

    #[must_use]
    pub fn icon_mode_maybe(self, icon_mode: Option<IconMode>) -> Self {
        match icon_mode {
            Some(mode) => self.icon_mode(mode),
            None => self,
        }
    }

    #[must_use]
    pub fn expressive_animation(mut self, expressive: bool) -> Self {
        self.expressive_animation = expressive;
        self
    }

    fn icon_rotation(&self) -> Radians {
        if self.selected {
            Radians(0.0)
        } else {
            // The close icon is symmetrical so it doesn't matter it's not a 360 rotation
            -1.0 * Radians::PI
        }
    }
}

#[derive(PartialEq)]
enum Status {
    Selected,
    SelectedDisabled,
    Unselected,
    UnselectedDisabled,
}

impl Status {
    fn new(selected: bool, enabled: bool) -> Self {
        match (selected, enabled) {
            (true, true) => Self::Selected,
            (true, false) => Self::SelectedDisabled,
            (false, true) => Self::Unselected,
            (false, false) => Self::UnselectedDisabled,
        }
    }
}

struct State {
    is_hovered: bool,
    is_pressed: bool,
    check_icon: Handle,
    close_icon: Handle,
    last_expressive: bool,
    last_status: Status,
    state_layer_color: ValueMotion<Color>,
    icon_rotation_spring: ValueMotion<Radians>,
    icon_fade_spring: SpringMotion,
    style_spring: ValueMotion<StateStyle>,
    handle_position_spring: SpringMotion,
    handle_size_spring: SpringMotion,
}

impl State {
    fn new(
        state_style: StateStyle,
        last_status: Status,
        expressive: bool,
        icon_rotation: Radians,
        state_layer_color: Color,
    ) -> Self {
        let now = Instant::now();
        Self {
            is_hovered: false,
            is_pressed: false,
            check_icon: Handle::from_memory(common_icons::CHECK),
            close_icon: Handle::from_memory(common_icons::CLOSE),
            last_expressive: expressive,
            last_status,
            state_layer_color: ValueMotion::new(
                state_layer_color,
                state_layer_color,
                fast_effects(expressive),
                now,
            ),
            icon_rotation_spring: ValueMotion::new(
                icon_rotation,
                icon_rotation,
                fast_spatial(expressive),
                now,
            ),
            icon_fade_spring: SpringMotion::new(fast_effects(expressive), 0.0, now),
            style_spring: ValueMotion::new(
                state_style.clone(),
                state_style,
                fast_effects(expressive),
                now,
            ),
            handle_position_spring: SpringMotion::new(fast_spatial(expressive), 0.0, now),
            handle_size_spring: SpringMotion::new(fast_spatial(expressive), 0.0, now),
        }
    }

    fn set_expressive(&mut self, expressive: bool) {
        self.last_expressive = expressive;

        self.state_layer_color.spring.spring = fast_effects(expressive);
        self.icon_rotation_spring.spring.spring = fast_spatial(expressive);
        self.icon_fade_spring.spring = fast_effects(expressive);
        self.style_spring.spring.spring = fast_effects(expressive);
        self.handle_position_spring.spring = fast_spatial(expressive);
        self.handle_size_spring.spring = fast_spatial(expressive);
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Switch<Message>
where
    Message: Clone,
    Renderer: iced::advanced::Renderer + iced::advanced::svg::Renderer,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(State::new(
            self.style
                .state(self.selected, self.on_toggle.is_some())
                .clone(),
            Status::new(self.selected, self.on_toggle.is_some()),
            self.expressive_animation,
            self.icon_rotation(),
            self.style
                .state_layer(self.selected)
                .hovered
                .scale_alpha(0.0),
        ))
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(TRACK_SIZE.width),
            height: Length::Fixed(TRACK_SIZE.width),
        }
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        _limits: &iced::advanced::layout::Limits,
    ) -> Node {
        let state = tree.state.downcast_mut::<State>();
        let handle_size = if state.is_pressed && self.on_toggle.is_some() {
            HANDLE_SIZE_PRESSED
        } else if self.icon_mode == IconMode::Always || self.selected {
            HANDLE_SIZE_WITH_ICON
        } else {
            HANDLE_SIZE_NO_ICON
        };

        state.handle_size_spring.target = handle_size;
        let handle_size = state.handle_size_spring.position;

        state.handle_position_spring.target = if self.selected {
            TRACK_SIZE.width - TRACK_SIZE.height / 2.0
        } else {
            TRACK_SIZE.height / 2.0
        };

        let handle_center = state.handle_position_spring.position;
        let handle_x = handle_center - handle_size / 2.0;
        let handle_y = (TRACK_SIZE.height - handle_size) / 2.0;

        let icon_x = handle_x + (handle_size - ICON_SIZE) / 2.0;
        let icon_y = handle_y + (handle_size - ICON_SIZE) / 2.0;

        let icon_node = Node::new(Size {
            width: ICON_SIZE,
            height: ICON_SIZE,
        })
        .move_to(Point::new(icon_x, icon_y));

        let handle_node = Node::new(Size {
            width: handle_size,
            height: handle_size,
        })
        .move_to(Point::new(handle_x, handle_y));

        Node::with_children(TRACK_SIZE, vec![handle_node, icon_node])
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let style = state.style_spring.value();
        let track_bounds = layout.bounds();
        let handle_bounds = layout.children().nth(0).unwrap().bounds();
        let icon_bounds = layout.children().nth(1).unwrap().bounds();

        renderer.fill_quad(
            Quad {
                bounds: track_bounds,
                border: Border::default()
                    .rounded(f32::MAX)
                    .color(style.outline_color)
                    .width(style.outline_width),
                ..Default::default()
            },
            style.track_color,
        );

        renderer.fill_quad(
            Quad {
                bounds: handle_bounds,
                border: Border::default().rounded(f32::MAX),
                ..Default::default()
            },
            style.handle_color,
        );

        let state = tree.state.downcast_ref::<State>();
        let icon = Svg::new(if state.icon_fade_spring.position < 0.5 {
            state.close_icon.clone()
        } else {
            state.check_icon.clone()
        })
        .rotation(state.icon_rotation_spring.value());

        let color = Color {
            r: style.icon_color.r,
            g: style.icon_color.g,
            b: style.icon_color.b,
            a: 1.0,
        };
        let opacity = if self.icon_mode == IconMode::Always {
            midpoint_distance(state.icon_fade_spring.position)
        } else if self.icon_mode == IconMode::WhenSelected {
            state.icon_fade_spring.position
        } else {
            0.0
        } * style.icon_color.a;
        (opacity > 0.0).then(|| {
            renderer.draw_svg(icon.color(color).opacity(opacity), icon_bounds, icon_bounds)
        });
    }

    fn mouse_interaction(
        &self,
        _tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over && self.on_toggle.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        if self.expressive_animation != state.last_expressive {
            state.set_expressive(self.expressive_animation);
        }
        let status = Status::new(self.selected, self.on_toggle.is_some());
        let now = Instant::now();
        if status != state.last_status {
            state.style_spring.set_target(
                self.style
                    .state(self.selected, self.on_toggle.is_some())
                    .clone(),
                now,
            );
            state.last_status = status;
        }
        let radians = self.icon_rotation();
        if state.icon_rotation_spring.to != radians {
            state.icon_rotation_spring.set_target(radians, now);
        }
        state.icon_fade_spring.target = if self.selected { 1.0 } else { 0.0 };

        let state_layer_color = match state.is_hovered {
            true => match state.is_pressed {
                true => self.style.state_layer(self.selected).pressed,
                false => self.style.state_layer(self.selected).hovered,
            },
            false => self
                .style
                .state_layer(self.selected)
                .hovered
                .scale_alpha(0.0),
        };
        state.state_layer_color.set_target(state_layer_color, now);
        if !state.handle_position_spring.is_at_rest()
            || !state.handle_size_spring.is_at_rest()
            || !state.style_spring.is_at_rest()
            || !state.icon_fade_spring.is_at_rest()
            || !state.icon_rotation_spring.is_at_rest()
            || !state.state_layer_color.is_at_rest()
        {
            shell.invalidate_layout();
            shell.request_redraw();
        }

        // Those DO need to be updated every frame, otherwise wacky things happen
        state.state_layer_color.step(now);
        state.icon_rotation_spring.step(now);
        state.icon_fade_spring.step(now);
        state.handle_size_spring.step(now);
        state.handle_position_spring.step(now);
        state.style_spring.step(now);

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
                    if let Some(on_toggle) = self.on_toggle.clone() {
                        shell.publish(on_toggle);
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

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'b>,
        _renderer: &Renderer,
        _viewport: &iced::Rectangle,
        _translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_ref::<State>();
        let color = state.state_layer_color.value();
        (self.on_toggle.is_some() && color.a > 0.0).then_some({
            let handle_bounds = layout.children().nth(0).unwrap().bounds();
            let bounds = Rectangle {
                x: handle_bounds.x + (handle_bounds.width - STATE_LAYER_SIZE) / 2.0,
                y: handle_bounds.y + (handle_bounds.height - STATE_LAYER_SIZE) / 2.0,
                width: STATE_LAYER_SIZE,
                height: STATE_LAYER_SIZE,
            };

            let overlay = StateLayerOverlay { color, bounds };
            overlay::Element::new(Box::new(overlay))
        })
    }
}

struct StateLayerOverlay {
    color: Color,
    bounds: Rectangle,
}

impl<Message, Theme, Renderer> Overlay<Message, Theme, Renderer> for StateLayerOverlay
where
    Renderer: iced::advanced::Renderer,
{
    fn layout(&mut self, _renderer: &Renderer, _bounds: Size) -> iced::advanced::layout::Node {
        Node::new(Size {
            width: self.bounds.width,
            height: self.bounds.height,
        })
        .move_to(Point::new(self.bounds.x, self.bounds.y))
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: mouse::Cursor,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border::default().rounded(f32::MAX),
                ..Default::default()
            },
            self.color,
        );
    }
}

impl<'a, Message> From<Switch<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(value: Switch<Message>) -> Self {
        Element::new(value)
    }
}
