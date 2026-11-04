use iced::{
    Border, Element, Font, Length, Pixels, Point, Size, Vector,
    advanced::{
        Layout, Widget, layout, mouse, overlay,
        renderer::Quad,
        text::{self, Paragraph},
        widget::Tree,
    },
};

use crate::widget::menu::{self, Style};

const LABEL_CENTER: f32 =
    (menu::constants::GROUP_LABEL_CELL_HEIGHT.0 - menu::constants::GROUP_LABEL_LINE_HEIGHT.0) / 2.0;

pub struct GroupElement<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer,
{
    pub style: Style,
    pub label_font: Option<Font>,
    pub label: Option<text::Fragment<'a>>,
    pub width: Pixels,

    // Cache
    // If the element is `None`, that means that it's a spacer
    pub items: Vec<Option<Element<'a, Message, Theme, Renderer>>>,
    pub label_paragraph: Option<Renderer::Paragraph>,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for GroupElement<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width.into(), Length::Shrink)
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.items.iter().flatten().collect::<Vec<_>>());
    }

    fn children(&self) -> Vec<Tree> {
        self.items.iter().flatten().map(Tree::new).collect()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max_width = self.width.0 - 2.0 * menu::constants::MENU_GROUP_PADDING.0;
        let mut children = Vec::with_capacity(self.items.len() + self.label.is_some() as usize);
        if let Some(label) = self.label.as_ref() {
            let max_width = max_width - menu::constants::GROUP_LABEL_LEFT_OFFSET.0;
            let limits = limits.max_width(max_width);
            let paragraph = <Renderer as iced::advanced::text::Renderer>::Paragraph::with_text(
                iced::advanced::text::Text {
                    content: label,
                    bounds: limits.max(),
                    size: menu::constants::GROUP_LABEL_FONT_SIZE,
                    line_height: text::LineHeight::Absolute(
                        menu::constants::GROUP_LABEL_LINE_HEIGHT,
                    ),
                    font: self.label_font.unwrap_or_else(|| renderer.default_font()),
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                    align_x: text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                },
            );

            children.push(layout::Node::new(Size::new(
                max_width,
                menu::constants::GROUP_LABEL_CELL_HEIGHT.0,
            )));

            self.label_paragraph = Some(paragraph);
        }

        let mut tree_index = 0;
        for item in &mut self.items {
            if let Some(item) = item {
                let limits = limits.max_width(max_width);

                children.push(item.as_widget_mut().layout(
                    &mut tree.children[tree_index],
                    renderer,
                    &limits,
                ));

                tree_index += 1;
            } else {
                children.push(layout::Node::new(Size::new(
                    max_width,
                    menu::constants::SPACER_HEIGHT.0,
                )));
            }
        }

        let mut offset_y = menu::constants::MENU_GROUP_PADDING.0;
        for child in &mut children {
            child.move_to_mut(Point::new(menu::constants::MENU_GROUP_PADDING.0, offset_y));
            offset_y += child.bounds().height + menu::constants::MENU_ITEM_SPACING.0;
        }

        layout::Node::with_children(
            Size::new(
                self.width.0,
                offset_y - menu::constants::MENU_ITEM_SPACING.0
                    + menu::constants::MENU_GROUP_PADDING.0,
            ),
            children,
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
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: self.style.container_border,
                shadow: crate::style::shadow(self.style.shadow_color, self.style.elevation),
                ..Default::default()
            },
            self.style.container_color,
        );

        let mut children = layout.children();
        if let Some(paragraph) = &self.label_paragraph {
            let bounds = children.next().unwrap().bounds();
            let position = bounds.position()
                + Vector::new(menu::constants::GROUP_LABEL_LEFT_OFFSET.0, LABEL_CENTER);
            renderer.fill_paragraph(paragraph, position, self.style.label_color, bounds);
        }

        let mut tree_index = 0;
        for item in &self.items {
            if let Some(item) = item {
                item.as_widget().draw(
                    &tree.children[tree_index],
                    renderer,
                    theme,
                    style,
                    children.next().unwrap(),
                    cursor,
                    viewport,
                );
                tree_index += 1;
            } else {
                // If the item is `None` then we draw the spacer
                renderer.fill_quad(
                    Quad {
                        bounds: children.next().unwrap().bounds(),
                        border: Border::default().rounded(f32::MAX),
                        ..Default::default()
                    },
                    self.style.separator_color,
                );
            }
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
        let mut layout_children = layout.children();
        if self.label_paragraph.is_some() {
            layout_children.next();
        }
        let mut tree_children = tree.children.iter_mut();
        for item in &mut self.items {
            let layout = layout_children.next().unwrap();
            if let Some(item) = item {
                item.as_widget_mut().update(
                    tree_children.next().unwrap(),
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
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children_layouts = layout.children();
        if self.label_paragraph.is_some() {
            children_layouts.next();
        }
        let mut tree_iter = tree.children.iter();
        for item in &self.items {
            let layout = children_layouts.next().unwrap();
            if let Some(item) = item {
                let interaction = item.as_widget().mouse_interaction(
                    tree_iter.next().unwrap(),
                    layout,
                    cursor,
                    viewport,
                    renderer,
                );

                if interaction != mouse::Interaction::None {
                    return interaction;
                }
            }
        }

        mouse::Interaction::None
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &iced::Rectangle,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let layout_index_offset = self.label_paragraph.is_some() as usize;
        let mut overlays = Vec::new();
        let mut children_iter = tree.children.iter_mut();

        for (layout_index, item) in self.items.iter_mut().enumerate() {
            if let Some(item) = item
                && let Some(overlay) = item.as_widget_mut().overlay(
                    children_iter.next().expect("Tree children count mismatch!"),
                    layout.child(layout_index + layout_index_offset),
                    renderer,
                    viewport,
                    translation,
                )
            {
                overlays.push(overlay);
            }
        }

        (!overlays.is_empty()).then(|| overlay::Group::with_children(overlays).overlay())
    }
}

impl<'a, Message, Theme, Renderer> From<GroupElement<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn from(value: GroupElement<'a, Message, Theme, Renderer>) -> Self {
        Element::new(value)
    }
}
