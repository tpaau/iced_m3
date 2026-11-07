use iced::{
    Border, Color, Element, Event, Font, Length, Point, Radians, Rectangle, Size, Vector,
    advanced::{
        Widget,
        layout::{Limits, Node},
        mouse, overlay,
        renderer::Quad,
        text::{self, Paragraph},
        widget::Tree,
    },
    touch,
};

const ICON_OFFSET: f32 = (menu::constants::ITEM_HEIGHT.0 - menu::constants::ITEM_ICON_SIZE.0) / 2.0;

use crate::{
    style::StateLayer,
    theme::ColorScheme,
    widget::{
        Action,
        common_icons::ARROW_RIGHT,
        hybrid_icon::Icon,
        menu::{self, Menu},
        ripple::Ripple,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub container_color: Color,
    pub icon_color: Color,
    pub label_color: Color,
    pub supporting_text_color: Color,
    pub state_layer: StateLayer,
}

impl Style {
    pub fn standard(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            container_color: theme.surface_container_low(),
            icon_color: theme.on_surface_variant(),
            label_color: theme.on_surface(),
            supporting_text_color: theme.on_surface_variant(),
            state_layer: StateLayer::new(theme.on_surface()),
        }
    }

    pub fn vibrant(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            container_color: theme.tertiary_container(),
            icon_color: theme.on_tertiary_container(),
            label_color: theme.on_tertiary_container(),
            supporting_text_color: theme.on_tertiary_container(),
            state_layer: StateLayer::new(theme.on_tertiary_container()),
        }
    }
}

/// Action to be performed when the button is pressed.
pub enum MenuAction<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    /// Open a submenu.
    SubMenu {
        /// Whether the submenu is opened.
        is_opened: bool,
        /// The closure from which the menu surface is produced.
        surface: Box<dyn Fn() -> Menu<'a, Message, Theme, Renderer> + 'a>,
        /// The action that toggles the submenu.
        on_press: Action<'a, Message>,
    },
    /// Emit a message.
    Message {
        /// Whether the button is selected or not.
        ///
        /// This is purely visual - it will make the button appear in standard style for vibrant
        /// menus and in vibrant style for standard menus.
        is_selected: bool,
        /// The action to perform when the button is pressed.
        ///
        /// If this is `None`, the button is disabled.
        on_press: Option<Action<'a, Message>>,
    },
}

pub struct Button<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    pub icon: Option<Icon<'a>>,
    pub label: text::Fragment<'a>,
    pub supporting_text: Option<text::Fragment<'a>>,
    pub action: MenuAction<'a, Message, Theme, Renderer>,
}

pub(crate) struct ButtonWidget<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    data: Button<'a, Message, Theme, Renderer>,
    style: Style,
    label_font: Option<Font>,

    // Cache
    is_hovered: bool,
    label_paragraph: Option<Renderer::Paragraph>,
    supporting_text_paragraph: Option<Renderer::Paragraph>,
    leading_icon_paragraph: Option<Renderer::Paragraph>,
    surface_element: Option<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> ButtonWidget<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    #[must_use]
    pub fn new(
        data: Button<'a, Message, Theme, Renderer>,
        style: Style,
        label_font: Option<Font>,
    ) -> Self {
        Self {
            data,
            style,
            label_font,
            is_hovered: false,
            label_paragraph: None,
            supporting_text_paragraph: None,
            leading_icon_paragraph: None,
            surface_element: None,
        }
    }

    fn is_enabled(&self) -> bool {
        match &self.data.action {
            MenuAction::SubMenu {
                is_opened: _,
                surface: _,
                on_press: _,
            } => true,
            MenuAction::Message {
                is_selected: _,
                on_press,
            } => on_press.is_some(),
        }
    }
}

struct State {
    is_pressed: bool,
    ripple: Ripple,
    surface_tree: Option<Tree>,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ButtonWidget<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a
        + iced::advanced::text::Renderer<Font = iced::Font>
        + iced::advanced::svg::Renderer
        + iced_widget::renderer::geometry::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, menu::constants::ITEM_HEIGHT.into())
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        let state = State {
            is_pressed: false,
            ripple: Ripple::new(),
            surface_tree: None,
        };
        iced::advanced::widget::tree::State::new(state)
    }

    fn layout(&mut self, _tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let intrinsic = Size::new(limits.max().width, menu::constants::ITEM_HEIGHT.0);

        let container_size = limits.resolve(
            iced::Length::Fixed(intrinsic.width),
            iced::Length::Fixed(intrinsic.height),
            intrinsic,
        );

        let leading_icon_node = self.data.icon.as_ref().map(|icon| {
            let bounds = Size::new(
                menu::constants::ITEM_ICON_SIZE.0,
                menu::constants::ITEM_ICON_SIZE.0,
            );

            if let Icon::Text { text, font } = icon {
                let paragraph = <Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(
                    iced::advanced::text::Text {
                        content: text,
                        bounds,
                        size: menu::constants::ITEM_ICON_SIZE,
                        line_height: text::LineHeight::Absolute(menu::constants::ITEM_ICON_SIZE),
                        font: font.unwrap_or_else(|| renderer.default_font()),
                        shaping: text::Shaping::Advanced,
                        wrapping: text::Wrapping::None,
                        align_x: text::Alignment::Left,
                        align_y: iced::alignment::Vertical::Top,
                    },
                );

                self.leading_icon_paragraph = Some(paragraph);
            }

            Node::new(bounds).move_to(Point::new(
                menu::constants::ITEM_CONTENT_PADDING.0,
                ICON_OFFSET,
            ))
        });

        let trailing_icon_node = matches!(
            self.data.action,
            MenuAction::SubMenu {
                is_opened: _,
                surface: _,
                on_press: _
            }
        )
        .then_some({
            let bounds = Size::new(
                menu::constants::ITEM_ICON_SIZE.0,
                menu::constants::ITEM_ICON_SIZE.0,
            );

            Node::new(bounds)
        });

        let max_center_width = container_size.width
            - leading_icon_node
                .as_ref()
                .map(|n| n.bounds().width + menu::constants::ITEM_CONTENT_SPACING.0)
                .unwrap_or_default()
            - trailing_icon_node
                .as_ref()
                .map(|n| n.bounds().width + menu::constants::ITEM_CONTENT_SPACING.0)
                .unwrap_or_default();

        let supporting_text_bounds = self.data.supporting_text.as_ref().map(|text| {
            let bounds = Size::new(
                max_center_width,
                menu::constants::ITEM_SUPPORTING_TEXT_LINE_HEIGHT.0,
            );

            let paragraph = <Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(
                iced::advanced::text::Text {
                    content: text,
                    bounds,
                    size: menu::constants::ITEM_SUPPORTING_TEXT_SIZE,
                    line_height: text::LineHeight::Absolute(
                        menu::constants::ITEM_SUPPORTING_TEXT_LINE_HEIGHT,
                    ),
                    font: self.label_font.unwrap_or_else(|| renderer.default_font()),
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                    align_x: text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                },
            );

            self.supporting_text_paragraph = Some(paragraph);

            bounds
        });

        let trailing_icon_node = trailing_icon_node.map(|n| {
            let width = n.bounds().width;
            n.move_to(Point::new(
                menu::constants::ITEM_CONTENT_PADDING.0
                    + leading_icon_node
                        .as_ref()
                        .map(|n| n.bounds().width + menu::constants::ITEM_CONTENT_SPACING.0)
                        .unwrap_or_default()
                    + max_center_width
                    + menu::constants::ITEM_CONTENT_SPACING.0
                    - width,
                ICON_OFFSET,
            ))
        });

        let label_bounds = Size::new(
            container_size.width
                - leading_icon_node
                    .as_ref()
                    .map(|n| n.bounds().width + menu::constants::ITEM_CONTENT_SPACING.0)
                    .unwrap_or_default()
                - trailing_icon_node
                    .as_ref()
                    .map(|n| n.bounds().width + menu::constants::ITEM_CONTENT_SPACING.0)
                    .unwrap_or_default(),
            menu::constants::ITEM_LABEL_LINE_HEIGHT.0,
        );
        let (label_node, supporting_text_node) = {
            let left_offset = leading_icon_node
                .as_ref()
                .map(|n| n.bounds().width + menu::constants::ITEM_CONTENT_SPACING.0)
                .unwrap_or_default()
                + menu::constants::ITEM_CONTENT_PADDING.0;
            let top_offset = (menu::constants::ITEM_HEIGHT.0
                - menu::constants::ITEM_LABEL_LINE_HEIGHT.0
                - supporting_text_bounds
                    .map(|b| b.height + menu::constants::ITEM_LABEL_SUPPORTING_TEXT_SPACING.0)
                    .unwrap_or_default())
                / 2.0;

            let label_node = Node::new(label_bounds).move_to(Point::new(left_offset, top_offset));

            let supporting_text_node = supporting_text_bounds.map(|bounds| {
                Node::new(bounds).move_to(Point::new(
                    left_offset,
                    top_offset
                        + label_node.bounds().height
                        + menu::constants::ITEM_LABEL_SUPPORTING_TEXT_SPACING.0,
                ))
            });

            (label_node, supporting_text_node)
        };

        let label_paragraph = <Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(
            iced::advanced::text::Text {
                content: &self.data.label,
                bounds: label_bounds,
                size: menu::constants::ITEM_LABEL_FONT_SIZE,
                line_height: text::LineHeight::Absolute(menu::constants::ITEM_LABEL_LINE_HEIGHT),
                font: self.label_font.unwrap_or_else(|| renderer.default_font()),
                shaping: text::Shaping::Advanced,
                wrapping: text::Wrapping::None,
                align_x: text::Alignment::Left,
                align_y: iced::alignment::Vertical::Top,
            },
        );

        self.label_paragraph = Some(label_paragraph);

        let children = [
            leading_icon_node,
            Some(label_node),
            supporting_text_node,
            trailing_icon_node,
        ]
        .into_iter()
        .flatten()
        .collect();
        Node::with_children(container_size, children)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let content_opacity = match self.is_enabled() {
            true => 1.0,
            false => menu::constants::ITEM_CONTENT_DISABLED_OPACITY,
        };

        let container_bounds = layout.bounds();
        renderer.fill_quad(
            Quad {
                bounds: container_bounds,
                border: Border::default().rounded(menu::constants::ITEM_CORNER_RADIUS.0),
                ..Default::default()
            },
            self.style.container_color,
        );

        let mut children = layout.children();
        if let Some(text) = self.leading_icon_paragraph.as_ref() {
            let layout = children.next().unwrap();
            renderer.fill_paragraph(
                text,
                layout.position(),
                self.style.icon_color.scale_alpha(content_opacity),
                layout.bounds(),
            )
        } else if let Some(Icon::Svg(handle)) = self.data.icon.as_ref() {
            let bounds = children.next().unwrap().bounds();
            renderer.draw_svg(
                iced_widget::core::Svg {
                    handle: handle.clone(),
                    color: Some(self.style.icon_color),
                    rotation: Radians(0.0),
                    opacity: content_opacity,
                },
                bounds,
                bounds,
            );
        }

        let label_layout = children.next().unwrap();
        renderer.fill_paragraph(
            self.label_paragraph.as_ref().unwrap(),
            label_layout.position(),
            self.style.label_color.scale_alpha(content_opacity),
            label_layout.bounds(),
        );

        if let Some(text) = self.supporting_text_paragraph.as_ref() {
            let layout = children.next().unwrap();
            renderer.fill_paragraph(
                text,
                layout.position(),
                self.style
                    .supporting_text_color
                    .scale_alpha(content_opacity),
                layout.bounds(),
            );
        }

        // Trailing icon for submenu buttons
        if let Some(layout) = children.next() {
            let bounds = layout.bounds();
            renderer.draw_svg(
                iced_widget::core::Svg {
                    handle: ARROW_RIGHT.clone(),
                    color: Some(self.style.icon_color),
                    rotation: Radians(0.0),
                    opacity: content_opacity,
                },
                bounds,
                bounds,
            );
        }

        state.ripple.draw(
            renderer,
            container_bounds,
            iced::border::Radius::from(menu::constants::ITEM_CORNER_RADIUS.0),
            self.style.state_layer,
            self.is_enabled(),
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        state
            .ripple
            .handle(event, layout.bounds(), cursor, self.is_enabled(), shell);
        let is_hovered = cursor.is_over(layout.bounds());
        if self.is_hovered != is_hovered {
            self.is_hovered = is_hovered;
            shell.request_redraw();
        }
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.is_enabled() && self.is_hovered {
                    state.is_pressed = true;
                    shell.capture_event();
                }
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if self.is_enabled() && state.is_pressed {
                    state.is_pressed = false;

                    if self.is_hovered {
                        match &self.data.action {
                            MenuAction::SubMenu {
                                is_opened: _,
                                surface: _,
                                on_press,
                            } => shell.publish(on_press.resolve()),
                            MenuAction::Message {
                                is_selected: _,
                                on_press,
                            } => {
                                if let Some(on_press) = on_press {
                                    shell.publish(on_press.resolve());
                                }
                            }
                        }
                    }

                    shell.capture_event();
                }
                shell.request_redraw();
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                state.is_pressed = false;
                shell.request_redraw();
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: iced::advanced::Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.is_hovered && self.is_enabled() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: iced::advanced::Layout<'b>,
        _renderer: &Renderer,
        _viewport: &Rectangle,
        _translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        if let MenuAction::SubMenu {
            is_opened,
            surface,
            on_press: _,
        } = &self.data.action
            && *is_opened
        {
            // TODO: Will this update then the surface closure changes?
            if self.surface_element.is_none() {
                self.surface_element = Some((surface)().into());
            }
            let state = tree.state.downcast_mut::<State>();
            let surface = self.surface_element.as_mut().unwrap();
            if state.surface_tree.is_none() {
                state.surface_tree = Some(Tree::new(surface.as_widget()));
            }
            Some(overlay::Element::new(Box::new(Overlay {
                surface,
                anchor_bounds: layout.bounds(),
                tree: state.surface_tree.as_mut().unwrap(),
            })))
        } else {
            None
        }
    }
}

impl<'a, Message, Theme, Renderer> From<ButtonWidget<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a
        + iced::advanced::text::Renderer<Font = iced::Font>
        + iced::advanced::svg::Renderer
        + iced_widget::renderer::geometry::Renderer,
{
    fn from(value: ButtonWidget<'a, Message, Theme, Renderer>) -> Self {
        Element::new(value)
    }
}

struct Overlay<'a, 'b, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    tree: &'b mut Tree,
    surface: &'b mut Element<'a, Message, Theme, Renderer>,
    anchor_bounds: Rectangle,
}

impl<'a, 'b, Message, Theme, Renderer> iced::advanced::Overlay<Message, Theme, Renderer>
    for Overlay<'a, 'b, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        let anchor_pos = self.anchor_bounds.position();

        let overlay_layout = self.surface.as_widget_mut().layout(
            self.tree,
            renderer,
            &Limits::new(Size::ZERO, bounds),
        );
        let overlay_bounds = overlay_layout.bounds();

        let mut target_pos = Point::new(anchor_pos.x + overlay_bounds.width, anchor_pos.y);

        let over_right = overlay_bounds.width + overlay_bounds.x + target_pos.x > bounds.width;
        if over_right {
            let flipped_x = anchor_pos.x - overlay_bounds.width;
            let flipped_fits_left = flipped_x > 0.0;
            if flipped_fits_left {
                target_pos.x = flipped_x;
            }
        }

        let over_bottom = overlay_bounds.height + overlay_bounds.y + target_pos.y > bounds.height;
        if over_bottom {
            let flipped_y = anchor_pos.y - overlay_bounds.height + self.anchor_bounds.height;
            let flipped_fits_top = flipped_y > 0.0;
            if flipped_fits_top {
                target_pos.y = flipped_y;
            }
        }

        overlay_layout.move_to(target_pos)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
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

    fn update(
        &mut self,
        event: &Event,
        layout: iced::advanced::Layout<'_>,
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
    }

    fn mouse_interaction(
        &self,
        layout: iced::advanced::Layout<'_>,
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

    fn operate(
        &mut self,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_widget::core::widget::Operation,
    ) {
        self.surface
            .as_widget_mut()
            .operate(self.tree, layout, renderer, operation);
    }

    fn overlay<'c>(
        &'c mut self,
        layout: iced::advanced::Layout<'c>,
        renderer: &Renderer,
    ) -> Option<iced::advanced::overlay::Element<'c, Message, Theme, Renderer>> {
        self.surface.as_widget_mut().overlay(
            self.tree,
            layout,
            renderer,
            &layout.bounds(),
            Vector::ZERO,
        )
    }
}
