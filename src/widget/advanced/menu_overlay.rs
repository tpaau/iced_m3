use iced::{
    Element, Event, Length, Pixels, Point, Rectangle, Size, Vector,
    advanced::{
        Layout, Widget,
        layout::{self, Limits},
        mouse, overlay,
    },
    keyboard, touch,
};

/// Describes which edge of `anchor` the menu surface is attached to, and where along that edge it
/// is placed.
///
/// Variant names are `EdgePosition`:
/// - `Edge` is the side of `anchor` the menu attaches to
/// - `Position` is the menu's alignment along that edge
///
/// For example, [`TopLeft`](Placement::TopLeft) means the menu is attached to the top edge of
/// `anchor` and aligned to the left. [`LeftTop`](Placement::LeftTop) means it is attached to the
/// left edge and aligned to the top.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Placement {
    TopLeft {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    TopCenter {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    TopRight {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    RightTop {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    RightCenter {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    RightBottom {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    BottomRight {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    BottomCenter {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    BottomLeft {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    LeftBottom {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    LeftCenter {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
    LeftTop {
        /// Padding between the menu surface and the anchor.
        padding: Pixels,
    },
}

impl Default for Placement {
    fn default() -> Self {
        Self::BottomLeft {
            padding: Pixels(0.0),
        }
    }
}

impl Placement {
    #[must_use]
    fn flip_x(self) -> Self {
        match self {
            Self::TopLeft { padding } => Self::TopRight { padding },
            Self::TopRight { padding } => Self::TopLeft { padding },
            Self::RightTop { padding } => Self::LeftTop { padding },
            Self::RightCenter { padding } => Self::LeftCenter { padding },
            Self::RightBottom { padding } => Self::LeftBottom { padding },
            Self::BottomRight { padding } => Self::BottomLeft { padding },
            Self::BottomLeft { padding } => Self::BottomRight { padding },
            Self::LeftBottom { padding } => Self::RightBottom { padding },
            Self::LeftCenter { padding } => Self::RightCenter { padding },
            Self::LeftTop { padding } => Self::RightTop { padding },
            Self::TopCenter { padding: _ } | Self::BottomCenter { padding: _ } => self,
        }
    }

    #[must_use]
    fn flip_y(self) -> Self {
        match self {
            Self::TopLeft { padding } => Self::BottomLeft { padding },
            Self::TopCenter { padding } => Self::BottomCenter { padding },
            Self::TopRight { padding } => Self::BottomRight { padding },
            Self::RightTop { padding } => Self::RightBottom { padding },
            Self::RightBottom { padding } => Self::RightTop { padding },
            Self::BottomRight { padding } => Self::TopRight { padding },
            Self::BottomCenter { padding } => Self::TopCenter { padding },
            Self::BottomLeft { padding } => Self::TopLeft { padding },
            Self::LeftBottom { padding } => Self::LeftTop { padding },
            Self::LeftTop { padding } => Self::LeftBottom { padding },
            Self::LeftCenter { padding: _ } | Self::RightCenter { padding: _ } => self,
        }
    }

    fn offset(&self, bounds: &Rectangle, overlay_bounds: &Rectangle) -> Vector {
        match self {
            Placement::TopLeft { padding } => Vector::new(
                -overlay_bounds.width + bounds.width,
                -overlay_bounds.height - padding.0,
            ),
            Placement::TopCenter { padding } => Vector::new(
                (-overlay_bounds.width + bounds.width) / 2.0,
                -overlay_bounds.height - padding.0,
            ),
            Placement::TopRight { padding } => Vector::new(0.0, -overlay_bounds.height - padding.0),
            Placement::RightCenter { padding } => Vector::new(
                bounds.width + padding.0,
                (-overlay_bounds.height + bounds.height) / 2.0,
            ),
            Placement::RightTop { padding } => Vector::new(
                bounds.width + padding.0,
                -overlay_bounds.height + bounds.height,
            ),
            Placement::RightBottom { padding } => Vector::new(bounds.width + padding.0, 0.0),
            Placement::BottomRight { padding } => Vector::new(0.0, bounds.height + padding.0),
            Placement::BottomCenter { padding } => Vector::new(
                (-overlay_bounds.width + bounds.width) / 2.0,
                bounds.height + padding.0,
            ),
            Placement::BottomLeft { padding } => Vector::new(
                -overlay_bounds.width + bounds.width,
                bounds.height + padding.0,
            ),
            Placement::LeftBottom { padding } => {
                Vector::new(-overlay_bounds.width - padding.0, 0.0)
            }
            Placement::LeftCenter { padding } => Vector::new(
                -overlay_bounds.width - padding.0,
                (-overlay_bounds.height + bounds.height) / 2.0,
            ),
            Placement::LeftTop { padding } => Vector::new(
                -overlay_bounds.width - padding.0,
                -overlay_bounds.height + bounds.height,
            ),
        }
    }
}

/// Properties regarding the menu press.
///
/// This struct is created when the menu surface overlay captures a mouse press, either inside or
/// outside the surface overlay's bounds. It's then passed to the `on_press` closure, which may emit
/// a `Message` to perform certain actions (eg. close the menu).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressProperties {
    /// The position of the mouse click.
    pub position: Option<Point>,
    /// The mouse button that was pressed.
    pub button: mouse::Button,
    /// The bounds of the menu surface during the press.
    pub bounds: Rectangle,
    /// The bounds of the menu anchor during the press.
    pub anchor_bounds: Rectangle,
    /// Whether the press was captured by the menu surface.
    pub was_captured: bool,
}

/// Properties regarding a keyboard key press.
///
/// This struct is created when the menu surface overlay captures a keyboard key press. It's then
/// passed to the `on_key` closure, which may emit a `Message` to perform certain actions (eg. close
/// the menu).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyProperties {
    /// The key pressed.
    pub key: keyboard::Key,
    /// The key pressed with all keyboard modifiers applied, except Ctrl.
    pub modified_key: keyboard::Key,
    /// The state of the modifier keys.
    pub modifiers: keyboard::Modifiers,
}

/// Helper widget for creating robust menu surface overlays.
///
/// This widget is stateless, meaning it's your job to keep track of whether the menu is open or
/// not. The [`MenuOverlay`] makes this easier by allowing you to set `on_press` and `on_key`
/// triggers for certain events while the menu surface is opened.
///
/// It also allows for different placements of the surface with the [`Placement`] enum, enabling you
/// to create various kinds of overlays.
pub struct MenuOverlay<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    anchor: Element<'a, Message, Theme, Renderer>,
    surface: Box<dyn Fn() -> Element<'a, Message, Theme, Renderer> + 'a>,
    // I decided to go with a closure + boolean flag instead of an element `Option` for the surface
    // because I plan to add animations to this widget in the future.
    opened: bool,
    placement: Placement,
    on_press: Option<Box<dyn Fn(PressProperties) -> Option<Message> + 'a>>,
    on_key: Option<Box<dyn Fn(KeyProperties) -> Option<Message> + 'a>>,
    capture_anchor_presses: bool,
    index: f32,
}

impl<'a, Message, Theme, Renderer> MenuOverlay<'a, Message, Theme, Renderer> {
    /// Creates a new [`MenuOverlay`] widget.
    #[must_use]
    pub fn new(
        anchor: impl Into<Element<'a, Message, Theme, Renderer>>,
        surface: impl Fn() -> Element<'a, Message, Theme, Renderer> + 'a,
        opened: bool,
    ) -> Self {
        Self {
            anchor: anchor.into(),
            surface: Box::new(surface),
            opened,
            placement: Placement::default(),
            on_press: None,
            on_key: None,
            capture_anchor_presses: true,
            index: 1.0,
        }
    }

    /// Sets the [`Placement`] of the [`MenuOverlay`].
    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the action to be called when a mouse button is pressed while the surface is open.
    #[must_use]
    pub fn on_press(mut self, on_press: impl Fn(PressProperties) -> Option<Message> + 'a) -> Self {
        self.on_press = Some(Box::new(on_press));
        self
    }

    /// Sets the action to be called when a mouse button is pressed while the surface is open, if `Some`.
    #[must_use]
    pub fn on_press_maybe(
        mut self,
        on_press: Option<impl Fn(PressProperties) -> Option<Message> + 'a>,
    ) -> Self {
        self.on_press =
            on_press.map(|f| Box::new(f) as Box<dyn Fn(PressProperties) -> Option<Message> + 'a>);
        self
    }

    /// Sets the action to be called when a keyboard key is pressed while the surface is open.
    #[must_use]
    pub fn on_key(mut self, on_key: impl Fn(KeyProperties) -> Option<Message> + 'a) -> Self {
        self.on_key = Some(Box::new(on_key));
        self
    }

    /// Sets the action to be called when a keyboard key is pressed while the surface is open, if `Some`.
    #[must_use]
    pub fn on_key_maybe(
        mut self,
        on_key: Option<impl Fn(KeyProperties) -> Option<Message> + 'a>,
    ) -> Self {
        self.on_key =
            on_key.map(|f| Box::new(f) as Box<dyn Fn(KeyProperties) -> Option<Message> + 'a>);
        self
    }

    /// Sets the [`index`](overlay::Overlay::index) of the menu surface overlay.
    ///
    /// The default index is `1.0`.
    #[must_use]
    pub fn index(mut self, index: f32) -> Self {
        self.index = index;
        self
    }

    /// Whether the menu should capture mouse/touch press events inside the anchor bounds.
    ///
    /// This is useful for preventing the menu from reopening instantly when pressing the anchor
    /// while the menu is open, if the anchor is a button.
    #[must_use]
    pub fn capture_anchor_presses(mut self, capture: bool) -> Self {
        self.capture_anchor_presses = capture;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for MenuOverlay<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        self.anchor.as_widget().tag()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        self.anchor.as_widget().state()
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        self.anchor.as_widget().children()
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        self.anchor.as_widget().diff(tree);
    }

    fn size_hint(&self) -> Size<Length> {
        self.anchor.as_widget().size_hint()
    }

    fn size(&self) -> Size<Length> {
        self.anchor.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.anchor.as_widget_mut().layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.anchor
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn operate(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.anchor
            .as_widget_mut()
            .operate(tree, layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.anchor.as_widget_mut().update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &iced::advanced::widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.anchor
            .as_widget()
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced::advanced::widget::Tree,
        layout: Layout<'b>,
        _renderer: &Renderer,
        _viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.opened.then_some({
            let mut bounds = layout.bounds();
            bounds.x += translation.x;
            bounds.y += translation.y;
            overlay::Element::new(Box::new(Overlay {
                surface: (self.surface)(),
                anchor_bounds: bounds,
                placement: self.placement,
                tree,
                on_press: self.on_press.as_ref().map(|on_press| on_press.as_ref()),
                on_key: self.on_key.as_ref().map(|on_key| on_key.as_ref()),
                capture_anchor_presses: self.capture_anchor_presses,
                index: self.index,
            }))
        })
    }
}

impl<'a, Message, Theme, Renderer> From<MenuOverlay<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::Renderer,
{
    fn from(value: MenuOverlay<'a, Message, Theme, Renderer>) -> Self {
        Element::new(value)
    }
}

struct Overlay<'a, 'b, Message, Theme, Renderer> {
    surface: Element<'a, Message, Theme, Renderer>,
    anchor_bounds: Rectangle,
    placement: Placement,
    tree: &'b mut iced::advanced::widget::Tree,
    on_press: Option<&'b (dyn Fn(PressProperties) -> Option<Message> + 'a)>,
    on_key: Option<&'b (dyn Fn(KeyProperties) -> Option<Message> + 'a)>,
    capture_anchor_presses: bool,
    index: f32,
}

impl<'a, 'b, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn index(&self) -> f32 {
        self.index
    }

    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let position = self.anchor_bounds.position();
        let layout = self
            .surface
            .as_widget_mut()
            .layout(self.tree, renderer, &Limits::new(Size::ZERO, bounds))
            .move_to(position);

        let overlay_bounds = layout.bounds();
        let offset = self.placement.offset(&self.anchor_bounds, &overlay_bounds);

        let over_right = overlay_bounds.width + overlay_bounds.x + offset.x > bounds.width;
        let over_left = overlay_bounds.x + offset.x < 0.0;

        let placement_flipped = self.placement.flip_x();
        let offset_flipped = placement_flipped.offset(&self.anchor_bounds, &overlay_bounds);
        let flipped_fits_right =
            overlay_bounds.width + overlay_bounds.x + offset_flipped.x < bounds.width;
        let flipped_fits_left = overlay_bounds.x + offset_flipped.x > 0.0;

        if (over_right && flipped_fits_left) || (over_left && flipped_fits_right) {
            self.placement = placement_flipped;
        }

        let over_top = overlay_bounds.y + offset.y < 0.0;
        let over_bottom = overlay_bounds.height + overlay_bounds.y + offset.y > bounds.height;

        let placement_flipped = self.placement.flip_y();
        let offset_flipped = placement_flipped.offset(&self.anchor_bounds, &overlay_bounds);
        let flipped_fits_top = overlay_bounds.y + offset_flipped.y > 0.0;
        let flipped_fits_bottom =
            overlay_bounds.height + overlay_bounds.y + offset_flipped.y < bounds.height;

        if (over_top && flipped_fits_bottom) || (over_bottom && flipped_fits_top) {
            self.placement = placement_flipped;
        }

        self.surface
            .as_widget_mut()
            .layout(self.tree, renderer, &Limits::new(Size::ZERO, bounds))
            .move_to(position + self.placement.offset(&self.anchor_bounds, &overlay_bounds))
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.surface.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_widget::core::widget::Operation,
    ) {
        self.surface
            .as_widget_mut()
            .operate(self.tree, layout, renderer, operation);
    }

    fn update(
        &mut self,
        event: &iced::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
    ) {
        self.surface.as_widget_mut().update(
            self.tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );

        let was_captured = shell.is_event_captured();
        if let Event::Mouse(mouse::Event::ButtonPressed(button)) = event {
            if let Some(on_press) = self.on_press {
                let properties = PressProperties {
                    position: cursor.position(),
                    button: *button,
                    anchor_bounds: self.anchor_bounds,
                    bounds: layout.bounds(),
                    was_captured,
                };

                if let Some(message) = (on_press)(properties) {
                    shell.publish(message)
                }
            }
        } else if let Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            modified_key,
            physical_key: _,
            location: _,
            modifiers,
            text: _,
            repeat: _,
        }) = event
            && let Some(on_key) = self.on_key
        {
            let properties = KeyProperties {
                key: key.clone(),
                modified_key: modified_key.clone(),
                modifiers: *modifiers,
            };

            if let Some(message) = (on_key)(properties) {
                shell.publish(message)
            }
        }

        if self.capture_anchor_presses {
            match event {
                Event::Mouse(event) => match event {
                    mouse::Event::ButtonPressed(_) | mouse::Event::ButtonReleased(_)
                        if cursor.is_over(self.anchor_bounds) =>
                    {
                        shell.capture_event();
                    }
                    _ => {}
                },
                Event::Touch(event) => match event {
                    touch::Event::FingerPressed { id: _, position } => {
                        if self.anchor_bounds.contains(*position) {
                            shell.capture_event();
                        }
                    }
                    touch::Event::FingerLifted { id, position }
                        if self.anchor_bounds.contains(*position) =>
                    {
                        shell.capture_event();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.surface.as_widget().mouse_interaction(
            self.tree,
            layout,
            cursor,
            &layout.bounds(),
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        self.surface.as_widget_mut().overlay(
            self.tree,
            layout,
            renderer,
            &layout.bounds(),
            Vector::ZERO,
        )
    }
}
