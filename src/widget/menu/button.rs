use iced::{
    Border, Color, Element, Event, Font, Length, Point, Size,
    advanced::{
        Widget,
        layout::Node,
        mouse,
        renderer::Quad,
        text::{self, Paragraph},
    },
    touch,
};

const ICON_OFFSET: f32 = (menu::constants::ITEM_HEIGHT.0 - menu::constants::ITEM_ICON_SIZE.0) / 2.0;

use crate::{
    style::StateLayer,
    theme::ColorScheme,
    widget::{
        Action,
        hybrid_icon::Icon,
        menu::{self, Group},
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

pub enum MenuAction<'a, Message>
where
    Message: Clone,
{
    SubMenu(Vec<Group<'a, Message>>),
    Message {
        is_selected: bool,
        action: Option<Action<'a, Message>>,
    },
}

pub struct Button<'a, Message>
where
    Message: Clone,
{
    pub icon: Option<Icon<'a>>,
    pub label: text::Fragment<'a>,
    pub supporting_text: Option<text::Fragment<'a>>,
    pub action: MenuAction<'a, Message>,
}

pub(crate) struct ButtonWidget<'a, Message, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    data: Button<'a, Message>,
    style: Style,
    label_font: Option<Font>,

    // Cache
    is_hovered: bool,
    label_paragraph: Option<Renderer::Paragraph>,
    supporting_text_paragraph: Option<Renderer::Paragraph>,
    leading_icon_paragraph: Option<Renderer::Paragraph>,
}

impl<'a, Message, Renderer> ButtonWidget<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    #[must_use]
    pub fn new(data: Button<'a, Message>, style: Style, label_font: Option<Font>) -> Self {
        Self {
            data,
            style,
            label_font,
            is_hovered: false,
            label_paragraph: None,
            supporting_text_paragraph: None,
            leading_icon_paragraph: None,
        }
    }

    fn is_enabled(&self) -> bool {
        match &self.data.action {
            MenuAction::SubMenu(_) => true,
            MenuAction::Message {
                is_selected: _,
                action,
            } => action.is_some(),
        }
    }
}

struct State {
    is_pressed: bool,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ButtonWidget<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, menu::constants::ITEM_HEIGHT.into())
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        let state = State { is_pressed: false };
        iced::advanced::widget::tree::State::new(state)
    }

    fn layout(
        &mut self,
        _tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
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

        let trailing_icon_node = matches!(self.data.action, MenuAction::SubMenu(_)).then_some({
            let bounds = Size::new(
                menu::constants::ITEM_ICON_SIZE.0,
                menu::constants::ITEM_ICON_SIZE.0,
            );

            Node::new(bounds).move_to(Point::new(
                menu::constants::ITEM_CONTENT_PADDING.0,
                ICON_OFFSET,
            ))
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
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

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
                self.style.icon_color,
                layout.bounds(),
            )
        }

        let label_layout = children.next().unwrap();
        renderer.fill_paragraph(
            self.label_paragraph.as_ref().unwrap(),
            label_layout.position(),
            self.style.label_color,
            label_layout.bounds(),
        );

        if let Some(text) = self.supporting_text_paragraph.as_ref() {
            let layout = children.next().unwrap();
            renderer.fill_paragraph(
                text,
                layout.position(),
                self.style.supporting_text_color,
                layout.bounds(),
            );
        }

        let state_layer_color = if state.is_pressed && self.is_hovered {
            self.style.state_layer.pressed
        } else if self.is_hovered {
            self.style.state_layer.hovered
        } else {
            self.style.state_layer.idle
        };
        renderer.fill_quad(
            Quad {
                bounds: container_bounds,
                border: Border::default().rounded(menu::constants::ITEM_CORNER_RADIUS.0),
                ..Default::default()
            },
            state_layer_color,
        );
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
                            MenuAction::SubMenu(_) => todo!("Submenus"),
                            MenuAction::Message {
                                is_selected: _,
                                action,
                            } => {
                                if let Some(on_press) = action {
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
        _tree: &iced::advanced::widget::Tree,
        _layout: iced::advanced::Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.is_hovered && self.is_enabled() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }
}

impl<'a, Message, Theme, Renderer> From<ButtonWidget<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn from(value: ButtonWidget<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}
