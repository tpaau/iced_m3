use std::time::Instant;

use iced::{Event, Point, Rectangle, advanced::Shell, border::Radius, mouse, touch, window};
use iced_widget::renderer::geometry;

use super::{
    message::Message,
    model::{Model, Pointer},
    render,
    update::{Context, Effect, update},
    view::{Scene, view},
};
use crate::style::StateLayer;

/// Ripple state for one widget, together with the glue to iced.
///
/// Call [`Ripple::handle`] from `Widget::update` (before the widget processes
/// the event itself) and [`Ripple::draw`] from `Widget::draw`.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Ripple {
    model: Model,
}

impl Ripple {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Translates `event` into ripple messages and requests redraws as needed.
    ///
    /// `bounds` is both the interactive area and the area the ripple is drawn in.
    pub fn handle<Message_>(
        &mut self,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
        enabled: bool,
        shell: &mut Shell<'_, Message_>,
    ) {
        self.handle_in(event, bounds, bounds, cursor, enabled, shell);
    }

    /// Like [`Ripple::handle`], for widgets whose ripple area (`target`) is
    /// different from the area that reacts to the pointer (`bounds`).
    pub fn handle_in<Message_>(
        &mut self,
        event: &Event,
        bounds: Rectangle,
        target: Rectangle,
        cursor: mouse::Cursor,
        enabled: bool,
        shell: &mut Shell<'_, Message_>,
    ) {
        let now = match event {
            Event::Window(window::Event::RedrawRequested(now)) => *now,
            _ => Instant::now(),
        };
        let context = Context {
            size: target.size(),
            enabled,
        };
        let captured = shell.is_event_captured();
        let over = cursor.is_over(bounds);

        let mut messages = Vec::with_capacity(3);
        if over != self.model.hovered {
            messages.push(Message::Hover(over));
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if over && !captured => {
                if let Some(position) = cursor.position() {
                    messages.push(Message::Press {
                        pointer: Pointer::Mouse,
                        at: Point::new(position.x - target.x, position.y - target.y),
                    });
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                messages.push(Message::Release {
                    pointer: Pointer::Mouse,
                });
                if over {
                    messages.push(Message::Click);
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) if over => {
                messages.push(Message::ContextMenu);
            }
            Event::Touch(touch::Event::FingerPressed { id, position })
                if bounds.contains(*position) && !captured =>
            {
                messages.push(Message::Press {
                    pointer: Pointer::Touch(id.0),
                    at: Point::new(position.x - target.x, position.y - target.y),
                });
            }
            Event::Touch(touch::Event::FingerLifted { id, position }) => {
                let pointer = Pointer::Touch(id.0);
                if bounds.contains(*position) {
                    messages.push(Message::Release { pointer });
                    messages.push(Message::Click);
                } else {
                    messages.push(Message::Cancel { pointer });
                }
            }
            Event::Touch(touch::Event::FingerLost { id, .. }) => {
                messages.push(Message::Cancel {
                    pointer: Pointer::Touch(id.0),
                });
            }
            _ => {}
        }
        messages.push(Message::Tick);

        let mut effect = Effect::Idle;
        for message in messages {
            let (model, e) = update(self.model, message, context, now);
            self.model = model;
            effect = e;
        }

        match effect {
            Effect::Idle => {}
            Effect::Redraw => shell.request_redraw(),
            Effect::WakeAt(at) => shell.request_redraw_at(at),
        }
    }

    /// Whether anything would be drawn right now.
    #[must_use]
    pub fn is_visible(&self, size: iced::Size, enabled: bool) -> bool {
        let context = Context { size, enabled };
        view(&self.model, context, Instant::now()) != Scene::default()
    }

    /// Draws the hover and press layers.
    pub fn draw<R: geometry::Renderer>(
        &self,
        renderer: &mut R,
        bounds: Rectangle,
        radius: Radius,
        layer: StateLayer,
        enabled: bool,
    ) {
        let context = Context {
            size: bounds.size(),
            enabled,
        };
        let scene = view(&self.model, context, Instant::now());
        render::draw(renderer, bounds, radius, layer, scene);
    }
}
