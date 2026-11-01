use std::time::Instant;

use iced::{
    Border, Color, Element, Length, Point, Radians, Rectangle, Size,
    advanced::{
        Overlay, Widget,
        layout::{self, Node},
        mouse, overlay,
        renderer::Quad,
    },
};
use iced_widget::core::{Svg, svg::Handle};

use crate::{
    animation::{
        Interpolable, midpoint_distance,
        motion::{self, Spring, SpringValue, fast_effects, fast_spatial},
    },
    style::StateLayer,
    theme::ColorScheme,
    widget::common_icons,
};

const TRACK_SIZE: Size<f32> = Size {
    width: 52.0,
    height: 32.0,
};

const HANDLE_CENTER_MIN: f32 = TRACK_SIZE.height / 2.0;
const HANDLE_CENTER_MAX: f32 = TRACK_SIZE.width - TRACK_SIZE.height / 2.0;

/// Returns `true` when the layout squeezes the switch below `TRACK_SIZE`
fn is_squeezed(bounds: Rectangle) -> bool {
    bounds.width < TRACK_SIZE.width || bounds.height < TRACK_SIZE.height
}

const TRACK_DISABLED_OUTLINE_WIDTH: f32 = 2.0;
const HANDLE_SIZE_NO_ICON: f32 = 16.0;
const HANDLE_SIZE_WITH_ICON: f32 = 24.0;
const HANDLE_SIZE_PRESSED: f32 = 28.0;
const STATE_LAYER_SIZE: f32 = 40.0;
const ICON_SIZE: f32 = 16.0;
const DRAG_START_THRESHOLD: f32 = 2.0;

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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    effects_spring: Option<Spring>,
    spatial_spring: Option<Spring>,

    // Cache
    style_checked: bool,
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
            effects_spring: None,
            spatial_spring: None,
            style_checked: false,
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
    pub fn motion_scheme(mut self, scheme: motion::Scheme) -> Self {
        self.effects_spring = Some(fast_effects(scheme));
        self.spatial_spring = Some(fast_spatial(scheme));
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

    fn icon_rotation_for(&self, selected: bool) -> Radians {
        if selected {
            Radians(0.0)
        } else {
            // The close icon is symmetrical so it doesn't matter it's not a 360 rotation
            -1.0 * Radians::PI
        }
    }

    fn icon_rotation(&self) -> Radians {
        self.icon_rotation_for(self.selected)
    }

    fn handle_size(&self, is_pressed: bool) -> f32 {
        if is_pressed && self.on_toggle.is_some() {
            HANDLE_SIZE_PRESSED
        } else if self.icon_mode == IconMode::Always || self.selected {
            HANDLE_SIZE_WITH_ICON
        } else {
            HANDLE_SIZE_NO_ICON
        }
    }
}

struct State {
    is_hovered: bool,
    is_dragging: bool,
    press_x: Option<f32>,
    drag_position: Option<f32>,
    awaiting_toggle: Option<bool>,
    check_icon: Handle,
    close_icon: Handle,
    last_effects_spring: Spring,
    last_spatial_spring: Spring,
    last_style: Style,
    state_layer_color: SpringValue<Color>,
    icon_rotation_spring: SpringValue<Radians>,
    icon_opacity_spring: SpringValue<f32>,
    style_spring: SpringValue<StateStyle>,
    handle_position_spring: SpringValue<f32>,
    handle_size_spring: SpringValue<f32>,
}

impl State {
    fn set_spatial_spring(&mut self, spatial: Spring) {
        self.icon_rotation_spring.spring.model = spatial;
        self.handle_position_spring.spring.model = spatial;
        self.handle_size_spring.spring.model = spatial;
    }

    fn set_effects_spring(&mut self, effects: Spring) {
        self.state_layer_color.spring.model = effects;
        self.icon_opacity_spring.spring.model = effects;
        self.style_spring.spring.model = effects;
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
        let icon_rotation = self.icon_rotation();
        let state_layer_color = self
            .style
            .state_layer(self.selected)
            .hovered
            .scale_alpha(0.0);
        let style = self
            .style
            .state(self.selected, self.on_toggle.is_some())
            .clone();
        let now = Instant::now();
        let effects_spring = self
            .effects_spring
            .unwrap_or(fast_effects(motion::Scheme::default()));
        let spatial_spring = self
            .spatial_spring
            .unwrap_or(fast_spatial(motion::Scheme::default()));
        let handle_position = if self.selected {
            HANDLE_CENTER_MAX
        } else {
            HANDLE_CENTER_MIN
        };
        let handle_size = self.handle_size(false);
        let icon_opacity = if self.icon_mode == IconMode::Always {
            0.5
        } else if self.icon_mode == IconMode::WhenSelected {
            1.0
        } else {
            0.0
        } * style.icon_color.a;

        let state = State {
            is_hovered: false,
            is_dragging: false,
            press_x: None,
            drag_position: None,
            awaiting_toggle: None,
            check_icon: Handle::from_memory(common_icons::CHECK),
            close_icon: Handle::from_memory(common_icons::CLOSE),
            last_effects_spring: effects_spring,
            last_spatial_spring: spatial_spring,
            state_layer_color: SpringValue::new(state_layer_color, effects_spring, now),
            icon_rotation_spring: SpringValue::new(icon_rotation, spatial_spring, now),
            icon_opacity_spring: SpringValue::new(icon_opacity, effects_spring, now),
            style_spring: SpringValue::new(style, effects_spring, now),
            handle_position_spring: SpringValue::new(handle_position, spatial_spring, now),
            handle_size_spring: SpringValue::new(handle_size, spatial_spring, now),
            last_style: self.style,
        };

        iced::advanced::widget::tree::State::new(state)
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(TRACK_SIZE.width),
            height: Length::Fixed(TRACK_SIZE.height),
        }
    }

    fn layout(
        &mut self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
        layout::atomic(limits, TRACK_SIZE.width, TRACK_SIZE.height)
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
        let bounds = layout.bounds();

        let draw_switch = |renderer: &mut Renderer| {
            renderer.fill_quad(
                Quad {
                    bounds,
                    border: Border::default()
                        .rounded(f32::MAX)
                        .color(style.outline_color)
                        .width(style.outline_width),
                    ..Default::default()
                },
                style.track_color,
            );

            let handle_size = state.handle_size_spring.value();
            let handle_center = state.handle_position_spring.value();
            let handle_bounds = Rectangle {
                x: bounds.x + handle_center - handle_size / 2.0,
                y: bounds.y + (bounds.height - handle_size) / 2.0,
                width: handle_size,
                height: handle_size,
            };

            renderer.fill_quad(
                Quad {
                    bounds: handle_bounds,
                    border: Border::default().rounded(f32::MAX),
                    ..Default::default()
                },
                style.handle_color,
            );

            let opacity = if self.icon_mode == IconMode::Always {
                midpoint_distance(state.icon_opacity_spring.value())
            } else if self.icon_mode == IconMode::WhenSelected {
                state.icon_opacity_spring.value()
            } else {
                0.0
            } * style.icon_color.a;
            (opacity > 0.0).then(|| {
                let icon = Svg::new(if state.icon_opacity_spring.value() < 0.5 {
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
                let icon_bounds = Rectangle {
                    x: handle_bounds.x + (handle_size - ICON_SIZE) / 2.0,
                    y: handle_bounds.y + (handle_size - ICON_SIZE) / 2.0,
                    width: ICON_SIZE,
                    height: ICON_SIZE,
                };
                renderer.draw_svg(icon.color(color).opacity(opacity), icon_bounds, icon_bounds)
            });
        };

        // When the layout squeezes the switch below `TRACK_SIZE`, clip the
        // handle to the track instead of letting it stick out
        if is_squeezed(bounds) {
            renderer.with_layer(bounds, draw_switch);
        } else {
            draw_switch(renderer);
        }
    }

    fn mouse_interaction(
        &self,
        tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.on_toggle.is_none() {
            return mouse::Interaction::default();
        }

        let state = tree.state.downcast_ref::<State>();
        if state.press_x.is_some() {
            mouse::Interaction::Grabbing
        } else if cursor.is_over(layout.bounds()) {
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
        let bounds = layout.bounds();
        let is_over = cursor.is_over(bounds);
        let handle_size = self.handle_size(state.press_x.is_some());
        let handle_half = handle_size / 2.0;
        // The same margin the handle keeps at the nominal size: half of the
        // track height the handle doesn't occupy, scaled to the actual size
        let margin = ((bounds.height - handle_size) / 2.0).max(0.0);
        let handle_center_min = (handle_half + margin).min(bounds.width / 2.0);
        let handle_center_max = (bounds.width - handle_half - margin).max(handle_center_min);
        // A track squeezed to a single point must not produce a NaN progress
        let handle_travel = (handle_center_max - handle_center_min).max(f32::EPSILON);
        let track_midpoint = bounds.width / 2.0;
        let rest_position = |selected: bool| {
            if selected {
                handle_center_max
            } else {
                handle_center_min
            }
        };

        let now = Instant::now();

        if let Some(effects_spring) = self.effects_spring
            && effects_spring != state.last_effects_spring
        {
            state.set_effects_spring(effects_spring);
        }
        if let Some(spatial_spring) = self.spatial_spring
            && spatial_spring != state.last_spatial_spring
        {
            state.set_spatial_spring(spatial_spring);
        }

        let drag_progress = state.drag_position.map(|drag_position| {
            ((drag_position - handle_center_min) / handle_travel).clamp(0.0, 1.0)
        });
        let visual_selected = match drag_progress {
            Some(progress) => progress >= 0.5,
            None => self.selected,
        };

        let style = match drag_progress {
            Some(progress) => self
                .style
                .state(false, self.on_toggle.is_some())
                .interpolate(*self.style.state(true, self.on_toggle.is_some()), progress),
            None => *self.style.state(visual_selected, self.on_toggle.is_some()),
        };
        if !self.style_checked && self.style != state.last_style {
            state.style_spring.reset(style, now);
            state.last_style = self.style;
            self.style_checked = true;
            shell.request_redraw();
        } else {
            state.style_spring.set_target(style.clone(), now);
        }

        state
            .handle_size_spring
            .set_target(self.handle_size(state.press_x.is_some()), now);
        if let Some(drag_position) = state.drag_position {
            state.handle_position_spring.reset(drag_position, now);
        } else {
            state
                .handle_position_spring
                .set_target(rest_position(self.selected), now);
        }
        let icon_rotation_target = match drag_progress {
            Some(progress) => self
                .icon_rotation_for(false)
                .interpolate(self.icon_rotation_for(true), progress),
            None => self.icon_rotation_for(visual_selected),
        };
        state
            .icon_rotation_spring
            .set_target(icon_rotation_target, now);
        state.icon_opacity_spring.set_target(
            drag_progress.unwrap_or(visual_selected as usize as f32),
            now,
        );

        let state_layer = match drag_progress {
            Some(progress) => (*self.style.state_layer(false))
                .interpolate(*self.style.state_layer(true), progress),
            None => *self.style.state_layer(visual_selected),
        };
        let state_layer_color = match state.is_hovered {
            true => match state.press_x.is_some() {
                true => state_layer.pressed,
                false => state_layer.hovered,
            },
            false => state_layer.hovered.scale_alpha(0.0),
        };
        state.state_layer_color.set_target(state_layer_color, now);
        if !state.handle_position_spring.is_at_rest()
            || !state.handle_size_spring.is_at_rest()
            || !state.style_spring.is_at_rest()
            || !state.icon_opacity_spring.is_at_rest()
            || !state.icon_rotation_spring.is_at_rest()
            || !state.state_layer_color.is_at_rest()
        {
            shell.request_redraw();
        }

        // Those DO need to be updated every frame, otherwise wacky things happen
        state.state_layer_color.step(now);
        state.icon_rotation_spring.step(now);
        state.icon_opacity_spring.step(now);
        state.handle_size_spring.step(now);
        state.handle_position_spring.step(now);
        state.style_spring.step(now);

        if state
            .awaiting_toggle
            .is_some_and(|selected_at_release| selected_at_release != self.selected)
        {
            state.awaiting_toggle = None;
            state.drag_position = None;
        }

        match event {
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if is_over => {
                let press_x = cursor.position().map_or(0.0, |point| point.x);
                shell.capture_event();
                state.press_x = Some(press_x);
                state.is_dragging = false;
                state.awaiting_toggle = None;
                state.drag_position = None;

                if self.on_toggle.is_some() {
                    let drag_position =
                        (press_x - bounds.x).clamp(handle_center_min, handle_center_max);

                    state.drag_position = Some(drag_position);
                    state.handle_position_spring.reset(drag_position, now);
                    shell.request_redraw();
                }
            }

            iced::Event::Mouse(mouse::Event::CursorMoved { .. }) if state.press_x.is_some() => {
                if let Some(position) = cursor.position() {
                    if !state.is_dragging
                        && (position.x - state.press_x.unwrap_or(0.0)).abs() > DRAG_START_THRESHOLD
                    {
                        state.is_dragging = true;
                    }

                    if self.on_toggle.is_some() {
                        let drag_position =
                            (position.x - bounds.x).clamp(handle_center_min, handle_center_max);

                        state.drag_position = Some(drag_position);
                        state.handle_position_spring.reset(drag_position, now);
                        shell.capture_event();
                        shell.request_redraw();
                    }
                }
            }

            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if state.press_x.is_some() =>
            {
                shell.capture_event();
                let toggled = if self.on_toggle.is_some() {
                    match state.is_dragging {
                        true => {
                            let drag_position =
                                state.drag_position.unwrap_or(rest_position(self.selected));
                            let target_selected = drag_position >= track_midpoint;
                            target_selected != self.selected
                        }
                        // A tap always toggles regardless of where the thumb was
                        // moved by the press
                        false => is_over,
                    }
                } else {
                    false
                };

                if toggled {
                    if let Some(on_toggle) = self.on_toggle.clone() {
                        shell.publish(on_toggle);
                    }
                    // Keep the thumb where the pointer left it until the parent
                    // applies the new `selected` value
                    state.awaiting_toggle = Some(self.selected);
                } else {
                    state.drag_position = None;
                    state
                        .handle_position_spring
                        .set_target(rest_position(self.selected), now);
                }
                state.press_x = None;
                state.is_dragging = false;
            }

            _ => {}
        }

        if state.is_hovered != is_over {
            state.is_hovered = is_over;
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
            let bounds = layout.bounds();
            let handle_size = state.handle_size_spring.value();
            let handle_center = state.handle_position_spring.value();
            let handle_bounds = Rectangle {
                x: bounds.x + handle_center - handle_size / 2.0,
                y: bounds.y + (bounds.height - handle_size) / 2.0,
                width: handle_size,
                height: handle_size,
            };

            let bounds = Rectangle {
                x: handle_bounds.x + (handle_bounds.width - STATE_LAYER_SIZE) / 2.0,
                y: handle_bounds.y + (handle_bounds.height - STATE_LAYER_SIZE) / 2.0,
                width: STATE_LAYER_SIZE,
                height: STATE_LAYER_SIZE,
            };

            let overlay = StateLayerOverlay {
                color,
                bounds,
                clip_bounds: is_squeezed(bounds).then_some(bounds),
            };
            overlay::Element::new(Box::new(overlay))
        })
    }
}

struct StateLayerOverlay {
    color: Color,
    bounds: Rectangle,
    clip_bounds: Option<Rectangle>,
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
        let draw_state_layer = |renderer: &mut Renderer| {
            renderer.fill_quad(
                Quad {
                    bounds: layout.bounds(),
                    border: Border::default().rounded(f32::MAX),
                    ..Default::default()
                },
                self.color,
            );
        };

        match self.clip_bounds {
            Some(clip_bounds) => renderer.with_layer(clip_bounds, draw_state_layer),
            None => draw_state_layer(renderer),
        }
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
