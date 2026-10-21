//! Standard button groups organize [`Button`] widgets in a row and affect the size of pressed buttons.
use std::time::Instant;

use iced::{
    Element, Length, Pixels, Point, Size, Vector,
    advanced::{
        Layout, Widget,
        layout::Node,
        mouse,
        widget::{Tree, tree},
    },
};

use crate::{
    animation::motion::{self, Spring, SpringValue, fast_spatial},
    widget::button::Button,
};

/// Extra small spacing between buttons. Recommended for extra small buttons.
pub const SPACING_EXTRA_SMALL: Pixels = Pixels(18.0);
/// Small spacing between buttons. Recommended for small buttons.
pub const SPACING_SMALL: Pixels = Pixels(12.0);
/// Medium spacing between buttons. Recommended for medium buttons.
pub const SPACING_MEDIUM: Pixels = Pixels(8.0);
/// Large spacing between buttons. Recommended for large buttons.
pub const SPACING_LARGE: Pixels = Pixels(8.0);
/// Extra large spacing between buttons. Recommended for extra large buttons.
pub const SPACING_EXTRA_LARGE: Pixels = Pixels(8.0);
/// The default fraction of the pressed child’s width used to expand it.
pub const DEFAULT_BUTTON_EXPANDED_RATIO: f32 = 0.15;

/// Standard button groups organize [`Button`] widgets in a row and affect the size of pressed buttons.
pub struct ButtonGroup<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    buttons: Vec<Element<'a, Message, Theme, Renderer>>,
    shrink_wrap: bool,
    motion_scheme: motion::Scheme,
    activation_spring: Option<Spring>,
    spacing: Pixels,
    expanded_ratio: f32,

    // Cache
    nodes: Vec<Node>,
    button_widths: Vec<Length>,
    buttons_enabled: Vec<bool>,
}

impl<'a, Message, Theme, Renderer> ButtonGroup<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    /// Creates a new [`ButtonGroup`] widget.
    #[must_use]
    pub fn new(buttons: impl IntoIterator<Item = Button<'a, Message, Renderer>>) -> Self {
        let buttons: Vec<_> = buttons.into_iter().collect();
        let button_widths = buttons.iter().map(|b| b.size.width).collect();
        let buttons_enabled = buttons.iter().map(|b| !b.is_disabled()).collect();
        let is_filling = buttons.iter().any(|b| b.size.width == Length::Fill);

        Self {
            buttons: buttons.into_iter().map(|b| b.into()).collect(),
            shrink_wrap: !is_filling,
            motion_scheme: motion::Scheme::default(),
            activation_spring: None,
            spacing: SPACING_SMALL,
            expanded_ratio: DEFAULT_BUTTON_EXPANDED_RATIO,
            nodes: Vec::new(),
            buttons_enabled,
            button_widths,
        }
    }

    /// Sets the motion scheme for the spring used for animating button sizes.
    #[must_use]
    pub fn motion_scheme(mut self, scheme: motion::Scheme) -> Self {
        self.motion_scheme = scheme;
        self
    }

    /// Sets the spring used for animating button sizes.
    #[must_use]
    pub fn activation_spring(mut self, spring: Spring) -> Self {
        self.activation_spring = Some(spring);
        self
    }

    /// Sets the spring used for animating button sizes, if `Some`.
    #[must_use]
    pub fn activation_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.activation_spring = spring;
        self
    }

    /// Sets the spacing between buttons in the group.
    #[must_use]
    pub fn spacing(mut self, spacing: Pixels) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the expanded button ratio of the group.
    ///
    /// The expanded ratio is a fraction of the pressed child’s width used to expand it.
    #[must_use]
    pub fn expanded_ratio(mut self, ratio: f32) -> Self {
        self.expanded_ratio = ratio.max(0.0);
        self
    }

    fn get_activation_spring(&self) -> Spring {
        self.activation_spring
            .unwrap_or(fast_spatial(motion::Scheme::default()))
    }
}

struct State {
    pressed_button: Option<usize>,
    spring_model: Spring,
    size_springs: Vec<SpringValue<f32>>,
    button_nodes: Vec<Node>,
}

fn normalize_node<'a>(child: &'a mut Node, parent: Layout<'a>) -> Layout<'a> {
    let child_pos = child.bounds().position();
    let parent_pos = parent.position();
    let pos = Point::new(child_pos.x + parent_pos.x, child_pos.y + parent_pos.y);
    child.move_to_mut(pos);
    Layout::new(child)
}

fn scale_node(node: Node, target_width: f32) -> Node {
    let original_size = node.size();
    let position = node.bounds().position();

    // Children are assumed to be center-aligned
    let child_translation = Vector::new(-(original_size.width - target_width) / 2.0, 0.0);
    let mut children = node.children().to_vec();
    for child in &mut children {
        child.translate_mut(child_translation);
    }

    Node::with_children(Size::new(target_width, original_size.height), children).move_to(position)
}

fn resize_buttons(
    mut nodes: Vec<Node>,
    springs: &[SpringValue<f32>],
    expanded_ratio: f32,
) -> Vec<Node> {
    let len = nodes.len();
    if len <= 1 {
        return nodes;
    }

    // 1. Get original widths and calculate growth for each active button
    let original_widths: Vec<f32> = nodes.iter().map(|n| n.size().width).collect();
    let gains: Vec<f32> = springs
        .iter()
        .enumerate()
        .map(|(i, spring)| original_widths[i] * spring.value() * expanded_ratio)
        .collect();

    // 2. Calculate net width change (Delta W) for each button
    let mut delta_widths = vec![0.0; len];
    for i in 0..len {
        let g = gains[i];
        delta_widths[i] += g;

        if i == 0 {
            // First button pushes only the next one
            delta_widths[i + 1] -= g;
        } else if i == len - 1 {
            // Last button pushes only the previous one
            delta_widths[i - 1] -= g;
        } else {
            // Middle buttons push both neighbors equally
            delta_widths[i - 1] -= g / 2.0;
            delta_widths[i + 1] -= g / 2.0;
        }
    }

    // 3. Apply transformations and translate by the accumulated offset
    let mut current_offset = 0.0;
    for i in 0..len {
        let target_width = original_widths[i] + delta_widths[i];
        nodes[i] =
            scale_node(nodes[i].clone(), target_width).translate(Vector::new(current_offset, 0.0));

        current_offset += delta_widths[i];
    }

    nodes
}

impl State {
    fn update_button_count(&mut self, count: usize) {
        self.pressed_button = None;
        let now = Instant::now();
        self.size_springs
            .resize_with(count, || SpringValue::new(0.0, self.spring_model, now));
    }

    fn update_spring_model(&mut self, model: Spring) {
        if let Some(spring) = self.size_springs.first()
            && spring.spring.model != model
        {
            self.spring_model = model;
            for spring in &mut self.size_springs {
                spring.spring.model = model
            }
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ButtonGroup<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn size(&self) -> Size<Length> {
        if self.shrink_wrap {
            Size::new(Length::Shrink, Length::Shrink)
        } else {
            Size::new(Length::Fill, Length::Shrink)
        }
    }

    fn state(&self) -> tree::State {
        let now = Instant::now();
        let model = self.get_activation_spring();

        let state = State {
            pressed_button: None,
            spring_model: model,
            size_springs: self
                .buttons
                .iter()
                .map(|_| SpringValue::new(0.0, model, now))
                .collect(),
            button_nodes: Vec::new(),
        };

        tree::State::new(state)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn children(&self) -> Vec<Tree> {
        self.buttons.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.buttons);

        let state = tree.state.downcast_mut::<State>();
        state.update_spring_model(self.get_activation_spring());
        state.update_button_count(self.buttons.len());
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
        let len = self.buttons.len();
        let mut button_nodes = vec![Node::new(Size::ZERO); len];
        let mut fill_indices = Vec::new();
        let mut total_shrink_width = 0.0;

        for i in 0..len {
            if self.button_widths[i] != Length::Fill {
                let node =
                    self.buttons[i]
                        .as_widget_mut()
                        .layout(&mut tree.children[i], renderer, limits);
                total_shrink_width += node.size().width;
                button_nodes[i] = node;
            } else {
                fill_indices.push(i);
            }
        }

        if !fill_indices.is_empty() {
            let total_spacing = self.spacing.0 * (len - 1) as f32;
            let available_width = limits.max().width - total_shrink_width - total_spacing;
            let fill_width = if !fill_indices.is_empty() {
                (available_width / fill_indices.len() as f32).max(0.0)
            } else {
                0.0
            };

            for &i in &fill_indices {
                let fill_limits = limits.width(Length::Fixed(fill_width));
                let node = self.buttons[i].as_widget_mut().layout(
                    &mut tree.children[i],
                    renderer,
                    &fill_limits,
                );
                button_nodes[i] = node;
            }
        }

        let highest = button_nodes
            .iter()
            .map(|n| n.bounds().height)
            .fold(0.0, f32::max);

        let mut x = 0.0;
        for node in &mut button_nodes {
            let size = node.size();
            node.move_to_mut(Point::new(x, (highest - size.height) / 2.0));
            x += size.width + self.spacing.0;
        }

        let state = tree.state.downcast_mut::<State>();
        self.nodes = button_nodes.clone();
        state.button_nodes = button_nodes;

        Node::new(Size::new(x - self.spacing.0, highest))
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
        for (i, button) in self.buttons.iter().enumerate() {
            let mut child = self.nodes[i].clone();
            let layout = normalize_node(&mut child, layout);
            button.as_widget().draw(
                &tree.children[i],
                renderer,
                theme,
                style,
                layout,
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
        let state = tree.state.downcast_mut::<State>();
        match event {
            iced::Event::Window(iced::window::Event::RedrawRequested(at)) => {
                let mut at_rest = true;
                for spring in &mut state.size_springs {
                    if !spring.is_at_rest() {
                        at_rest = false;
                    }
                    spring.step(*at);
                }

                if !at_rest {
                    shell.request_redraw();
                }

                self.nodes = resize_buttons(
                    state.button_nodes.clone(),
                    state.size_springs.as_slice(),
                    self.expanded_ratio,
                );
            }
            iced::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let now = Instant::now();
                for spring in &mut state.size_springs {
                    spring.set_target(0.0, now);
                }
                state.pressed_button = None;
            }
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let now = Instant::now();
                for (index, (spring, node)) in state
                    .size_springs
                    .iter_mut()
                    .zip(&state.button_nodes)
                    .enumerate()
                {
                    let node = self.nodes.get(index).unwrap_or(node);
                    let bounds_position = layout.bounds().position();
                    let bounds = node
                        .clone()
                        .translate(Vector::new(bounds_position.x, bounds_position.y))
                        .bounds();

                    let active = cursor.is_over(bounds) && self.buttons_enabled[index];
                    state.pressed_button = active.then_some(index);
                    spring.set_target(active as usize as f32, now);
                    shell.request_redraw();

                    if active {
                        break;
                    }
                }
            }
            iced::Event::Mouse(mouse::Event::CursorMoved { position: _ }) => {
                if let Some(index) = state.pressed_button {
                    let node = self
                        .nodes
                        .get(index)
                        .cloned()
                        .unwrap_or(state.button_nodes[index].clone());
                    let bounds_position = layout.bounds().position();
                    let bounds = node
                        .clone()
                        .translate(Vector::new(bounds_position.x, bounds_position.y))
                        .bounds();

                    let now = Instant::now();
                    if cursor.is_over(bounds) {
                        state.size_springs[index].set_target(1.0, now);
                    } else {
                        state.size_springs[index].set_target(0.0, now);
                    }
                }
            }
            _ => {}
        }

        for (i, button) in self.buttons.iter_mut().enumerate() {
            let mut child = self.nodes[i].clone();
            let layout = normalize_node(&mut child, layout);
            button.as_widget_mut().update(
                &mut tree.children[i],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
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
        for (i, button) in self.buttons.iter().enumerate() {
            let mut child = self.nodes[i].clone();
            let layout = normalize_node(&mut child, layout);
            let interaction = button.as_widget().mouse_interaction(
                &tree.children[i],
                layout,
                cursor,
                viewport,
                renderer,
            );
            if interaction != mouse::Interaction::None {
                return interaction;
            }
        }

        return mouse::Interaction::None;
    }
}

impl<'a, Message, Theme, Renderer> From<ButtonGroup<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn from(value: ButtonGroup<'a, Message, Theme, Renderer>) -> Self {
        Element::new(value)
    }
}
