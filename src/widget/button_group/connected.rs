//! Connected button groups help people select options, switch views, or sort elements in a page.

use iced::{
    Element, Font, Length, Padding, Pixels, Point,
    advanced::{Widget, layout::Node, mouse, widget::Tree},
    border::Radius,
    padding,
};

use crate::{
    animation::motion::{self, Spring},
    widget::button::{self, Content, CornerRadius, CornerStyle, Style},
};

#[derive(Debug, Clone)]
pub struct Button<'a, Message>
where
    Message: Clone,
{
    pub content: Content<'a>,
    pub on_press: Option<Message>,
    pub selected: bool,
}

/// The size of elements in the connected button group.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub container_height: Pixels,
    pub button_spacing: Pixels,
    pub outer_corner_size: Pixels,
    pub inner_corner_size: Pixels,
    pub pressed_corner_size: Pixels,
    pub icon_size: Pixels,
    pub label_size: Pixels,
    pub content_spacing: Pixels,
    pub content_padding: Padding,
}

impl Default for Size {
    fn default() -> Self {
        Self::small()
    }
}

impl Size {
    pub fn extra_small() -> Self {
        Self {
            container_height: Pixels(32.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(8.0),
            pressed_corner_size: Pixels(4.0),
            icon_size: button::constants::ICON_SIZE_EXTRA_SMALL,
            label_size: button::constants::LABEL_SIZE_EXTRA_SMALL,
            content_spacing: button::constants::CONTENT_SPACING_EXTRA_SMALL,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_EXTRA_SMALL),
        }
    }

    pub fn small() -> Self {
        Self {
            container_height: Pixels(40.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(8.0),
            pressed_corner_size: Pixels(4.0),
            icon_size: button::constants::ICON_SIZE_SMALL,
            label_size: button::constants::LABEL_SIZE_SMALL,
            content_spacing: button::constants::CONTENT_SPACING_SMALL,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_SMALL),
        }
    }

    pub fn medium() -> Self {
        Self {
            container_height: Pixels(56.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(8.0),
            pressed_corner_size: Pixels(4.0),
            icon_size: button::constants::ICON_SIZE_MEDIUM,
            label_size: button::constants::LABEL_SIZE_MEDIUM,
            content_spacing: button::constants::CONTENT_SPACING_MEDIUM,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_MEDIUM),
        }
    }

    pub fn large() -> Self {
        Self {
            container_height: Pixels(96.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(16.0),
            pressed_corner_size: Pixels(12.0),
            icon_size: button::constants::ICON_SIZE_LARGE,
            label_size: button::constants::LABEL_SIZE_LARGE,
            content_spacing: button::constants::CONTENT_SPACING_LARGE,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_LARGE),
        }
    }

    pub fn extra_large() -> Self {
        Self {
            container_height: Pixels(136.0),
            button_spacing: Pixels(2.0),
            outer_corner_size: Pixels(f32::MAX),
            inner_corner_size: Pixels(20.0),
            pressed_corner_size: Pixels(16.0),
            icon_size: button::constants::ICON_SIZE_EXTRA_LARGE,
            label_size: button::constants::LABEL_SIZE_EXTRA_LARGE,
            content_spacing: button::constants::CONTENT_SPACING_EXTRA_LARGE,
            content_padding: padding::horizontal(button::constants::CONTENT_PADDING_EXTRA_LARGE),
        }
    }
}

/// Connected button groups help people select options, switch views, or sort elements in a page.
pub struct ButtonGroup<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    model: Vec<Button<'a, Message>>,
    style: Style,
    size: Size,
    label_font: Option<Font>,
    motion_scheme: Option<motion::Scheme>,
    style_spring: Option<Spring>,
    state_layer_spring: Option<Spring>,
    corner_radius_spring: Option<Spring>,

    // Cache
    button_widgets: Vec<button::Button<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> ButtonGroup<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    #[must_use]
    pub fn new(style: Style, model: Vec<Button<'a, Message>>) -> Self {
        let button_widgets = Vec::with_capacity(model.len());
        Self {
            model,
            style,
            size: Size::default(),
            button_widgets,
            label_font: None,
            motion_scheme: None,
            style_spring: None,
            state_layer_spring: None,
            corner_radius_spring: None,
        }
    }

    #[must_use]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn button(mut self, button: Button<'a, Message>) -> Self {
        self.model.push(button);
        self
    }

    #[must_use]
    pub fn button_maybe(mut self, button: Option<Button<'a, Message>>) -> Self {
        if let Some(button) = button {
            self.model.push(button);
        }
        self
    }

    #[must_use]
    pub fn label_font(mut self, font: Font) -> Self {
        self.label_font = Some(font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, font: Option<Font>) -> Self {
        self.label_font = font;
        self
    }

    #[must_use]
    pub fn motion_scheme(mut self, scheme: motion::Scheme) -> Self {
        self.motion_scheme = Some(scheme);
        self
    }

    #[must_use]
    pub fn motion_scheme_maybe(mut self, scheme: Option<motion::Scheme>) -> Self {
        self.motion_scheme = scheme;
        self
    }

    #[must_use]
    pub fn style_spring(mut self, spring: Spring) -> Self {
        self.style_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn style_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.style_spring = spring;
        self
    }

    #[must_use]
    pub fn state_layer_spring(mut self, spring: Spring) -> Self {
        self.state_layer_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn state_layer_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.state_layer_spring = spring;
        self
    }

    #[must_use]
    pub fn corner_radius_spring(mut self, spring: Spring) -> Self {
        self.corner_radius_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn corner_radius_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.corner_radius_spring = spring;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ButtonGroup<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn size(&self) -> iced::Size<Length> {
        iced::Size::new(Length::Shrink, self.size.container_height.into())
    }

    fn children(&self) -> Vec<Tree> {
        self.button_widgets
            .iter()
            .map(|button| Tree::new(button as &dyn Widget<Message, Theme, Renderer>))
            .collect()
    }

    fn diff(&self, tree: &mut Tree) {
        let children: Vec<&dyn Widget<Message, Theme, Renderer>> = self
            .button_widgets
            .iter()
            .map(|button| button as &dyn Widget<Message, Theme, Renderer>)
            .collect();

        tree.diff_children(&children);
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
        let nodes: Vec<_> = self
            .button_widgets
            .iter_mut()
            .enumerate()
            .map(|(index, button)| {
                let node = (button as &mut dyn Widget<Message, Theme, Renderer>).layout(
                    &mut tree.children[index],
                    renderer,
                    limits,
                );

                node
            })
            .collect();

        // TODO: There shouldn't be a widest constraint
        let widest = nodes
            .iter()
            .map(|n| n.bounds())
            .max_by(|a, b| a.size().width.total_cmp(&b.size().width))
            .map(|bounds| bounds.width);

        if let Some(width) = widest {
            let nodes = self
                .button_widgets
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    Node::with_children(
                        iced::Size::new(width, self.size.container_height.0),
                        nodes[i].children().to_vec(),
                    )
                    .move_to(Point::new(
                        i as f32 * (width + self.size.button_spacing.0),
                        0.0,
                    ))
                })
                .collect();

            return Node::with_children(
                iced::Size::new(
                    self.button_widgets.len() as f32 * (width + self.size.button_spacing.0),
                    self.size.container_height.0,
                ),
                nodes,
            );
        }

        Node::new(iced::Size::new(0.0, self.size.container_height.0))
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        for (index, button) in self.button_widgets.iter().enumerate() {
            button.draw(
                &tree.children[index],
                renderer,
                theme,
                style,
                layout.child(index),
                cursor,
                viewport,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        for (index, button) in self.button_widgets.iter_mut().enumerate() {
            (button as &mut dyn Widget<Message, Theme, Renderer>).update(
                &mut tree.children[index],
                event,
                layout.child(index),
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            )
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        for (index, button) in self.button_widgets.iter().enumerate() {
            let interaction = (button as &dyn Widget<Message, Theme, Renderer>).mouse_interaction(
                &tree.children[index],
                layout.child(index),
                cursor,
                viewport,
                renderer,
            );

            if interaction != mouse::Interaction::None {
                return interaction;
            }
        }

        mouse::Interaction::default()
    }
}

impl<'a, Message, Theme, Renderer> From<ButtonGroup<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn from(mut value: ButtonGroup<'a, Message, Renderer>) -> Self {
        value.button_widgets = value
            .model
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, model)| {
                let left_radius = if index == 0 {
                    value.size.outer_corner_size.0
                } else {
                    value.size.inner_corner_size.0
                };
                let right_radius = if index == value.model.len() - 1 {
                    value.size.outer_corner_size.0
                } else {
                    value.size.inner_corner_size.0
                };
                let corner_radius = CornerRadius {
                    style: CornerStyle::Rounded,
                    shape_morph: true,
                    rounded: Radius {
                        top_left: left_radius,
                        top_right: right_radius,
                        bottom_right: right_radius,
                        bottom_left: left_radius,
                    },
                    square: value.size.outer_corner_size.0.into(),
                    pressed: value.size.pressed_corner_size.0.into(),
                };

                let size = button::Size {
                    width: Length::Shrink,
                    height: value.size.container_height.into(),
                    spacing: value.size.content_spacing.into(),
                    padding: value.size.content_padding,
                    icon_size: value.size.icon_size,
                    label_size: value.size.label_size,
                    corner_radius,
                };

                crate::widget::button(value.style, model.content)
                    .selected(model.selected)
                    .label_font_maybe(value.label_font)
                    .on_press_maybe(model.on_press)
                    .motion_scheme_maybe(value.motion_scheme)
                    .style_spring_maybe(value.style_spring)
                    .state_layer_spring_maybe(value.state_layer_spring)
                    .corner_radius_spring_maybe(value.corner_radius_spring)
                    .size(size)
            })
            .collect();
        Element::new(value)
    }
}
