use std::time::Instant;

use iced::{
    Border, Color, Element, Length, Point, Rectangle, Size,
    advanced::{Overlay, Widget, layout::Node, mouse, overlay, renderer::Quad},
};
use iced_widget::core::{Svg, svg::Handle};

use crate::{
    animation::motion::{SpringMotion, fast_spatial},
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
    WhenEnabled,
    Always,
}

struct StateStyle {
    track_color: Color,
    handle_color: Color,
    icon_color: Color,
    outline_color: Option<Color>,
}

struct Style {
    state_layer_selected: StateLayer,
    state_later_unselected: StateLayer,
    selected: StateStyle,
    selected_disabled: StateStyle,
    unselected: StateStyle,
    unselected_disabled: StateStyle,
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
                outline_color: None,
            },
            selected_disabled: StateStyle {
                track_color: theme.on_surface().scale_alpha(TRACK_DISABLED_OPACITY),
                handle_color: theme
                    .surface()
                    .scale_alpha(HANDLE_DISABLED_SELECTED_OPACITY),
                icon_color: theme.on_surface().scale_alpha(ICON_DISABLED_OPACITY),
                outline_color: None,
            },
            unselected: StateStyle {
                track_color: theme.surface_container_highest(),
                handle_color: theme.outline(),
                icon_color: theme.surface_container_highest(),
                outline_color: Some(theme.outline()),
            },
            unselected_disabled: StateStyle {
                track_color: theme.on_surface().scale_alpha(TRACK_DISABLED_OPACITY),
                handle_color: theme
                    .on_surface()
                    .scale_alpha(HANDLE_DISABLED_UNSELECTED_OPACITY),
                icon_color: theme
                    .surface_container_highest()
                    .scale_alpha(ICON_DISABLED_OPACITY),
                outline_color: Some(theme.on_surface().scale_alpha(OUTLINE_DISABLED_OPACITY)),
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
}

struct State {
    is_hovered: bool,
    is_pressed: bool,
    check_icon: Handle,
    close_icon: Handle,
    handle_position_spring: SpringMotion,
    handle_size_spring: SpringMotion,
}

impl Default for State {
    fn default() -> Self {
        let instant = Instant::now();
        Self {
            is_hovered: false,
            is_pressed: false,
            check_icon: Handle::from_memory(common_icons::CHECK),
            close_icon: Handle::from_memory(common_icons::CLOSE),
            // TODO: Optional expressive
            handle_position_spring: SpringMotion::new(fast_spatial(true), 0.0, instant),
            handle_size_spring: SpringMotion::new(fast_spatial(true), 0.0, instant),
        }
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
        iced::advanced::widget::tree::State::new(State::default())
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
        let style = self.style.state(self.selected, self.on_toggle.is_some());
        let track_bounds = layout.bounds();
        let handle_bounds = layout.children().nth(0).unwrap().bounds();
        let icon_bounds = layout.children().nth(1).unwrap().bounds();

        let border = Border::default().rounded(f32::MAX);
        let border = match style.outline_color {
            Some(color) => border.color(color).width(TRACK_DISABLED_OUTLINE_WIDTH),
            None => border,
        };
        renderer.fill_quad(
            Quad {
                bounds: track_bounds,
                border,
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

        let icon = if self.icon_mode == IconMode::Always {
            let state = tree.state.downcast_ref::<State>();
            match self.selected {
                true => Some(Svg::new(state.check_icon.clone())),
                false => match self.on_toggle.is_some() {
                    true => Some(Svg::new(state.close_icon.clone())),
                    false => None,
                },
            }
        } else if self.icon_mode == IconMode::WhenEnabled && self.selected {
            let state = tree.state.downcast_ref::<State>();
            Some(Svg::new(state.check_icon.clone()))
        } else {
            None
        };

        let color = Color {
            r: style.icon_color.r,
            g: style.icon_color.g,
            b: style.icon_color.b,
            a: 1.0,
        };
        icon.map(|icon| {
            renderer.draw_svg(
                icon.color(color).opacity(style.icon_color.a),
                icon_bounds,
                icon_bounds,
            )
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
        let instant = Instant::now();
        if !state.handle_position_spring.is_at_rest() || !state.handle_size_spring.is_at_rest() {
            shell.invalidate_layout();
            shell.request_redraw();
        }
        state.handle_size_spring.step(instant);
        state.handle_position_spring.step(instant);
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
        (self.on_toggle.is_some() && tree.state.downcast_ref::<State>().is_hovered).then_some({
            let state = tree.state.downcast_ref::<State>();
            let color = match state.is_pressed {
                true => self.style.state_layer(self.selected).pressed,
                false => self.style.state_layer(self.selected).hovered,
            };

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
