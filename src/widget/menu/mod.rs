pub mod button;
mod group;

use iced::{
    Border, Color, Element, Font, Length, Pixels, Point, Size, Vector,
    advanced::{
        Layout, Widget, layout, mouse, overlay,
        text::{self},
        widget::Tree,
    },
};

pub(crate) mod constants {
    use iced::Pixels;

    /// Default width of a [`Menu`] widget.
    pub const DEFAULT_WIDTH: Pixels = Pixels(192.0);
    /// Container border radius of a [`Menu`] widget.
    pub const CONTAINER_BORDER_RADIUS: Pixels = Pixels(16.0);
    /// Spacing between groups in a [`Menu`] widget.
    pub const MENU_GROUP_BETWEEN_SPACING: Pixels = Pixels(2.0);
    /// Padding of a group in a [`Menu`] widget.
    pub const MENU_GROUP_PADDING: Pixels = Pixels(4.0);
    /// Spacing between items of a group in a [`Menu`] widget.
    pub const MENU_ITEM_SPACING: Pixels = Pixels(2.0);
    /// Height of the items spacer.
    pub const SPACER_HEIGHT: Pixels = Pixels(1.0);
    pub const ITEM_LABEL_FONT_SIZE: Pixels = Pixels(14.0);
    pub const ITEM_LABEL_LINE_HEIGHT: Pixels = Pixels(20.0);
    pub const ITEM_CORNER_RADIUS: Pixels = Pixels(12.0);
    pub const GROUP_LABEL_FONT_SIZE: Pixels = Pixels(14.0);
    pub const GROUP_LABEL_LINE_HEIGHT: Pixels = Pixels(20.0);
    pub const ITEM_HEIGHT: Pixels = Pixels(44.0);
    pub const GROUP_LABEL_CELL_HEIGHT: Pixels = Pixels(32.0);
    pub const GROUP_LABEL_LEFT_OFFSET: Pixels = Pixels(12.0);
    pub const ITEM_ICON_SIZE: Pixels = Pixels(20.0);
    pub const ITEM_CONTENT_PADDING: Pixels = Pixels(12.0);
    pub const ITEM_CONTENT_SPACING: Pixels = Pixels(8.0);
    pub const ITEM_SUPPORTING_TEXT_SIZE: Pixels = Pixels(12.0);
    pub const ITEM_SUPPORTING_TEXT_LINE_HEIGHT: Pixels = Pixels(16.0);
    pub const ITEM_LABEL_SUPPORTING_TEXT_SPACING: Pixels = Pixels(0.0);
    pub const ITEM_CONTENT_DISABLED_OPACITY: f32 = 0.38;
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

use crate::{
    style::{DIM_ALPHA, Elevation},
    theme::ColorScheme,
    widget::menu::{
        button::{Button, ButtonWidget, MenuAction},
        group::GroupElement,
    },
};

/// Single item in a [menu group](Group).
#[allow(clippy::large_enum_variant)] // The separators are rarely used
pub enum Item<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    /// Standard menu button.
    Button(Button<'a, Message, Theme, Renderer>),
    /// Custom widget.
    Element(Element<'a, Message, Theme, Renderer>),
    /// Horizontal rule adding visual separation between adjacent items.
    Separator,
}

/// Group of [items](Item) in a [menu](Menu).
pub struct Group<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    /// Optional label text for the group.
    pub label: Option<text::Fragment<'a>>,
    /// The [items](Item) in the group.
    pub items: Vec<Item<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Group<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    /// Creates a new group with the given [items](Item) and no label text.
    pub fn new(items: Vec<Item<'a, Message, Theme, Renderer>>) -> Self {
        Self { label: None, items }
    }
}

/// The style of the [`Menu`] widget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub container_color: Color,
    pub container_border: Border,
    pub shadow_color: Color,
    pub elevation: Elevation,
    pub label_color: Color,
    pub separator_color: Color,
    pub item: button::Style,
    pub item_selected: button::Style,
}

impl Style {
    /// Creates a new style for the [`Menu`] widget given a color scheme and a `vibrant` flag.
    ///
    /// Standard menus (`vibrant == false`) have a more toned down surface container color, while
    /// vibrant menus (`vibrant == true`) use the tertiary accent.
    pub fn new(theme: &(impl ColorScheme + ?Sized), vibrant: bool) -> Self {
        match vibrant {
            true => Self {
                container_color: theme.tertiary_container(),
                container_border: Border::default().rounded(constants::CONTAINER_BORDER_RADIUS.0),
                shadow_color: theme.shadow(),
                elevation: Elevation::Level2,
                label_color: theme.on_tertiary_container(),
                separator_color: theme.tertiary().scale_alpha(DIM_ALPHA),
                item: button::Style::vibrant(theme),
                item_selected: button::Style::standard(theme),
            },
            false => Self {
                container_color: theme.surface_container_low(),
                container_border: Border::default().rounded(constants::CONTAINER_BORDER_RADIUS.0),
                shadow_color: theme.shadow(),
                elevation: Elevation::Level2,
                label_color: theme.on_surface_variant(),
                separator_color: theme.outline_variant(),
                item: button::Style::standard(theme),
                item_selected: button::Style::vibrant(theme),
            },
        }
    }
}

/// Menu surface with [groups](Group) that can contain either [buttons](button::Button),
/// [separators](Item::Separator) or [custom elements](Item::Element).
///
/// This widget can be paired with an overlay surface provider of choice, like
/// [`MenuOverlay`](crate::widget::advanced::menu_overlay::MenuOverlay).
pub struct Menu<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    groups: Vec<Group<'a, Message, Theme, Renderer>>,
    style: Style,
    width: Pixels,
    label_font: Option<Font>,

    // Cache
    group_elements: Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Menu<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    /// Creates a new [`Menu`] widget.
    #[must_use]
    pub fn new(
        style: Style,
        groups: impl IntoIterator<Item = Group<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            groups: groups.into_iter().collect(),
            style,
            width: constants::DEFAULT_WIDTH,
            label_font: None,
            group_elements: Vec::new(),
        }
    }

    /// Sets the `width` of the menu.
    #[must_use]
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    /// Sets the label font of the menu.
    #[must_use]
    pub fn label_font(mut self, font: Font) -> Self {
        self.label_font = Some(font);
        self
    }

    /// Sets the label font of the menu, is `Some`.
    #[must_use]
    pub fn label_font_maybe(mut self, font: Option<Font>) -> Self {
        self.label_font = font;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Menu<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width.into(), Length::Shrink)
    }

    fn children(&self) -> Vec<Tree> {
        self.group_elements.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.group_elements);
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let mut groups: Vec<_> = self
            .group_elements
            .iter_mut()
            .enumerate()
            .map(|(index, group)| {
                group
                    .as_widget_mut()
                    .layout(&mut tree.children[index], renderer, limits)
            })
            .collect();

        let mut offset_y = 0.0;
        for group in &mut groups {
            group.move_to_mut(Point::new(0.0, offset_y));
            offset_y += group.bounds().height + constants::MENU_GROUP_BETWEEN_SPACING.0;
        }

        layout::Node::with_children(
            Size::new(
                self.width.into(),
                offset_y - constants::MENU_GROUP_BETWEEN_SPACING.0,
            ),
            groups,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        for (index, group) in self.group_elements.iter().enumerate() {
            group.as_widget().draw(
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
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        for (index, group) in self.group_elements.iter_mut().enumerate() {
            group.as_widget_mut().update(
                &mut tree.children[index],
                event,
                layout.child(index),
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
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        for (index, group) in self.group_elements.iter().enumerate() {
            let interaction = group.as_widget().mouse_interaction(
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

        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Idle
        } else {
            mouse::Interaction::None
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &iced::Rectangle,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let mut children = Vec::new();
        for ((group, tree), layout) in self
            .group_elements
            .iter_mut()
            .zip(tree.children.iter_mut())
            .zip(layout.children())
        {
            let pos = layout.bounds().position();
            let translation = Vector::new(pos.x + translation.x, pos.y + translation.y);

            if let Some(group) =
                group
                    .as_widget_mut()
                    .overlay(tree, layout, renderer, viewport, translation)
            {
                children.push(group);
            }
        }

        (!children.is_empty()).then(|| overlay::Group::with_children(children).overlay())
    }
}

impl<'a, Message, Theme, Renderer> From<Menu<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer:
        'a + iced::advanced::text::Renderer<Font = iced::Font> + iced::advanced::svg::Renderer,
{
    fn from(mut value: Menu<'a, Message, Theme, Renderer>) -> Self {
        let groups = std::mem::take(&mut value.groups);
        value.group_elements = groups
            .into_iter()
            .map(|g| {
                GroupElement {
                    width: value.width,
                    label_font: value.label_font,
                    style: value.style,
                    label: g.label,
                    items: g
                        .items
                        .into_iter()
                        .map(|item| match item {
                            Item::Button(data) => {
                                let style = match data.action {
                                    MenuAction::SubMenu {
                                        is_opened: _,
                                        surface: _,
                                        on_press: _,
                                    } => value.style.item,
                                    MenuAction::Message {
                                        is_selected,
                                        on_press: _,
                                    } => match is_selected {
                                        true => value.style.item_selected,
                                        false => value.style.item,
                                    },
                                };
                                Some(ButtonWidget::new(data, style, value.label_font).into())
                            }
                            Item::Element(element) => Some(element),
                            Item::Separator => None,
                        })
                        .collect(),
                    label_paragraph: None,
                }
                .into()
            })
            .collect();

        Element::new(value)
    }
}
