mod item;

use iced::Color;
use iced::Element;
use iced::Font;
use iced::Length;
use iced::Pixels;
use iced::Point;
use iced::Size;
use iced::advanced::Widget;
use iced::advanced::layout::Node;
use iced::advanced::mouse;
use iced::advanced::renderer::Quad;
use iced::advanced::widget::Tree;

use crate::animation::motion;
use crate::animation::motion::Spring;
use crate::theme::ColorScheme;

pub use item::Content as Item;
pub use item::Style as ItemStyle;

mod constants {
    pub const CONTAINER_HEIGHT: f32 = 64.0;
    pub const ITEM_LARGE_SPACING: f32 = 20.0;
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Compact,
    Large,
    #[default]
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub container_color: Color,
    pub item_style: ItemStyle,
}

impl Style {
    pub fn new(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            container_color: theme.surface_container(),
            item_style: ItemStyle::new(theme),
        }
    }
}

pub struct NavBar<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    items: Vec<Item<'a, Message>>,
    style: Style,
    label_font: Option<Font>,
    mode: Mode,
    active_index: usize,
    motion_scheme: Option<motion::Scheme>,
    effects_spring: Option<Spring>,
    spatial_spring: Option<Spring>,

    // Cache
    item_widgets: Vec<item::Item<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> NavBar<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    #[must_use]
    pub fn new(style: Style, items: Vec<Item<'a, Message>>) -> Self {
        Self {
            items,
            style,
            label_font: None,
            mode: Mode::default(),
            active_index: 0,
            motion_scheme: None,
            effects_spring: None,
            spatial_spring: None,
            item_widgets: Vec::new(),
        }
    }

    #[must_use]
    pub fn item(mut self, item: Item<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    #[must_use]
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    #[must_use]
    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index = index;
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

    fn get_items(&self, mode: item::Mode) -> Vec<item::Item<'a, Message, Renderer>> {
        self.items
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                item::Item::new(
                    self.style.item_style,
                    item,
                    self.label_font,
                    index == self.active_index,
                )
                .mode(mode)
                .motion_scheme_maybe(self.motion_scheme)
                .active_transition_spring_maybe(self.spatial_spring)
                .color_spring_maybe(self.effects_spring)
            })
            .collect()
    }

    fn compact_layout<Theme>(
        &mut self,
        container_bounds: Size,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
        let mut item_widgets = self.get_items(item::Mode::Compact);

        let cell_width = container_bounds.width / self.items.len() as f32;
        let item_nodes = item_widgets
            .iter_mut()
            .enumerate()
            .map(|(index, item)| {
                let item = (item as &mut dyn Widget<Message, Theme, Renderer>).layout(
                    &mut tree.children[index],
                    renderer,
                    limits,
                );

                let item_bounds = item.size();
                item.move_to(Point::new(
                    cell_width * index as f32 + (cell_width - item_bounds.width) / 2.0,
                    (container_bounds.height - item_bounds.height) / 2.0,
                ))
            })
            .collect();
        self.item_widgets = item_widgets;

        Node::with_children(container_bounds, item_nodes)
    }

    fn large_layout<Theme>(
        &mut self,
        container_bounds: Size,
        item_width: f32,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
        let mut item_widgets = self.get_items(item::Mode::Large {
            width: Some(Pixels(item_width)),
        });

        let spacing = constants::ITEM_LARGE_SPACING;
        let total_width =
            item_width * item_widgets.len() as f32 + spacing * (item_widgets.len() - 1) as f32;
        let offset = (container_bounds.width - total_width) / 2.0;

        let item_nodes = item_widgets
            .iter_mut()
            .enumerate()
            .map(|(index, item)| {
                let item = (item as &mut dyn Widget<Message, Theme, Renderer>).layout(
                    &mut tree.children[index],
                    renderer,
                    limits,
                );

                let item_bounds = item.size();
                item.move_to(Point::new(
                    offset + (item_width + spacing) * index as f32,
                    (container_bounds.height - item_bounds.height) / 2.0,
                ))
            })
            .collect();
        self.item_widgets = item_widgets;

        Node::with_children(container_bounds, item_nodes)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for NavBar<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(constants::CONTAINER_HEIGHT))
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::None
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        let children: Vec<&dyn Widget<Message, Theme, Renderer>> = self
            .item_widgets
            .iter()
            .map(|item| item as &dyn Widget<Message, Theme, Renderer>)
            .collect();

        tree.diff_children(&children);
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        self.item_widgets
            .iter()
            .map(|item| Tree::new(item as &dyn Widget<Message, Theme, Renderer>))
            .collect()
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let container_bounds = limits.resolve(
            Length::Fill,
            Length::Fixed(constants::CONTAINER_HEIGHT),
            Size::ZERO,
        );

        let widest_needed = self.mode == Mode::Large || self.mode == Mode::Auto;
        let widest = widest_needed
            .then(|| {
                self.get_items(item::Mode::Large { width: None })
                    .iter_mut()
                    .enumerate()
                    .map(|(index, item)| {
                        let node = (item as &mut dyn Widget<Message, Theme, Renderer>).layout(
                            &mut tree.children[index],
                            renderer,
                            limits,
                        );

                        node.bounds()
                    })
                    .max_by(|a, b| a.size().width.total_cmp(&b.size().width))
                    .map(|bounds| bounds.width)
            })
            .flatten();

        match self.mode {
            Mode::Compact => self.compact_layout::<Theme>(container_bounds, tree, renderer, limits),
            Mode::Large => match widest {
                Some(width) => {
                    self.large_layout::<Theme>(container_bounds, width, tree, renderer, limits)
                }
                None => Node::new(container_bounds),
            },
            Mode::Auto => match widest {
                Some(width) => {
                    let total_width = width * self.item_widgets.len() as f32
                        + constants::ITEM_LARGE_SPACING * (self.item_widgets.len() - 1) as f32;
                    if total_width > container_bounds.width {
                        self.compact_layout::<Theme>(container_bounds, tree, renderer, limits)
                    } else {
                        self.large_layout::<Theme>(container_bounds, width, tree, renderer, limits)
                    }
                }
                None => Node::new(container_bounds),
            },
        }
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                ..Default::default()
            },
            self.style.container_color,
        );

        for (index, item) in self.item_widgets.iter().enumerate() {
            item.draw(
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
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        for (index, item) in self.item_widgets.iter_mut().enumerate() {
            (item as &mut dyn Widget<Message, Theme, Renderer>).update(
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
        tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        for (index, item) in self.item_widgets.iter().enumerate() {
            let interaction = (item as &dyn Widget<Message, Theme, Renderer>).mouse_interaction(
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

        match cursor.is_over(layout.bounds()) {
            true => mouse::Interaction::Idle,
            false => mouse::Interaction::None,
        }
    }
}

impl<'a, Message, Theme, Renderer> From<NavBar<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn from(mut value: NavBar<'a, Message, Renderer>) -> Self {
        value.item_widgets = value.get_items(item::Mode::Compact);
        Element::new(value)
    }
}
