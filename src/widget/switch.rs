use iced::{
    Border, Color, Element, Length, Point, Rectangle, Size,
    advanced::{Overlay, Widget, layout::Node, mouse, overlay, renderer::Quad},
};
use iced_widget::core::{Svg, svg::Handle};

use crate::{theme::ColorScheme, widget::common_icons};

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
const STATE_LAYER_OPACITY: f32 = 0.08;

struct State {
    is_hovered: bool,
    is_pressed: bool,
    check_icon: Handle,
    close_icon: Handle,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_hovered: false,
            is_pressed: false,
            check_icon: Handle::from_memory(common_icons::CHECK),
            close_icon: Handle::from_memory(common_icons::CLOSE),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IconMode {
    Never,
    #[default]
    WhenEnabled,
    Always,
}

pub struct Switch<'a, Message>
where
    Message: Clone,
{
    theme: &'a dyn ColorScheme,
    icon_mode: IconMode,
    enabled: bool,
    on_toggle: Option<Message>,
}

impl<'a, Message> Switch<'a, Message>
where
    Message: Clone,
{
    #[must_use]
    pub fn new(theme: &'a dyn ColorScheme, enabled: bool) -> Self {
        Self {
            theme,
            icon_mode: IconMode::default(),
            enabled,
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

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Switch<'a, Message>
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
        // const HANDLE_SIZE_NO_ICON: f32 = 16.0;
        // const HANDLE_SIZE_WITH_ICON: f32 = 24.0;
        // const HANDLE_SIZE_PRESSED: f32 = 28.0;

        let state = tree.state.downcast_ref::<State>();
        let handle_size = if state.is_pressed && self.on_toggle.is_some() {
            HANDLE_SIZE_PRESSED
        } else if self.icon_mode == IconMode::Always || self.enabled {
            HANDLE_SIZE_WITH_ICON
        } else {
            HANDLE_SIZE_NO_ICON
        };

        let padding = (TRACK_SIZE.height - handle_size) / 2.0;
        let handle_x = if self.enabled {
            TRACK_SIZE.width - handle_size - padding
        } else {
            padding
        };

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
        let track_bounds = layout.bounds();
        let handle_bounds = layout.children().nth(0).unwrap().bounds();
        let icon_bounds = layout.children().nth(1).unwrap().bounds();

        let enabled = self.on_toggle.is_some();
        let outline_color = match enabled {
            true => self.theme.outline(),
            false => self
                .theme
                .on_surface()
                .scale_alpha(OUTLINE_DISABLED_OPACITY),
        };
        let (track_color, handle_color, icon_color, icon_opacity, border_width) = match self.enabled
        {
            true => match enabled {
                true => (
                    self.theme.primary(),
                    self.theme.on_primary(),
                    self.theme.on_primary_container(),
                    1.0,
                    0.0,
                ),
                false => (
                    self.theme.on_surface().scale_alpha(TRACK_DISABLED_OPACITY),
                    self.theme
                        .surface()
                        .scale_alpha(HANDLE_DISABLED_SELECTED_OPACITY),
                    self.theme.on_surface(),
                    ICON_DISABLED_OPACITY,
                    0.0,
                ),
            },
            false => match enabled {
                true => (
                    self.theme.surface_container_highest(),
                    self.theme.outline(),
                    self.theme.surface_container_highest(),
                    1.0,
                    TRACK_DISABLED_OUTLINE_WIDTH,
                ),
                false => (
                    self.theme.on_surface().scale_alpha(TRACK_DISABLED_OPACITY),
                    self.theme
                        .on_surface()
                        .scale_alpha(HANDLE_DISABLED_UNSELECTED_OPACITY),
                    self.theme.surface_container_highest(),
                    ICON_DISABLED_OPACITY,
                    TRACK_DISABLED_OUTLINE_WIDTH,
                ),
            },
        };

        renderer.fill_quad(
            Quad {
                bounds: track_bounds,
                border: Border::default()
                    .rounded(f32::MAX)
                    .width(border_width)
                    .color(outline_color),
                ..Default::default()
            },
            track_color,
        );

        renderer.fill_quad(
            Quad {
                bounds: handle_bounds,
                border: Border::default().rounded(f32::MAX),
                ..Default::default()
            },
            handle_color,
        );

        let icon = if self.icon_mode == IconMode::Always {
            let state = tree.state.downcast_ref::<State>();
            match self.enabled {
                true => Some(Svg::new(state.check_icon.clone())),
                false => match self.on_toggle.is_some() {
                    true => Some(Svg::new(state.close_icon.clone())),
                    false => None,
                },
            }
        } else if self.icon_mode == IconMode::WhenEnabled && self.enabled {
            let state = tree.state.downcast_ref::<State>();
            Some(Svg::new(state.check_icon.clone()))
        } else {
            None
        };

        icon.map(|icon| {
            renderer.draw_svg(
                icon.color(icon_color).opacity(icon_opacity),
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
            let color = match self.enabled {
                true => self.theme.primary(),
                false => self.theme.on_surface(),
            }
            .scale_alpha(STATE_LAYER_OPACITY);

            let handle_bounds = layout.children().nth(0).unwrap().bounds();
            let bounds = Rectangle {
                x: handle_bounds.x + (handle_bounds.width - STATE_LAYER_SIZE) / 2.0,
                y: handle_bounds.y + (handle_bounds.height - STATE_LAYER_SIZE) / 2.0,
                width: STATE_LAYER_SIZE,
                height: STATE_LAYER_SIZE,
            };

            let overlay = StateLayer { color, bounds };
            overlay::Element::new(Box::new(overlay))
        })
    }
}

struct StateLayer {
    color: Color,
    bounds: Rectangle,
}

impl<Message, Theme, Renderer> Overlay<Message, Theme, Renderer> for StateLayer
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

impl<'a, Message> From<Switch<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(value: Switch<'a, Message>) -> Self {
        Element::new(value)
    }
}
