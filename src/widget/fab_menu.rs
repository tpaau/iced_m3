use iced::{
    Element, Event, Font, Point, Rectangle, Size,
    advanced::{
        Widget,
        layout::{Limits, Node},
        overlay, svg, text,
        widget::{Tree, tree},
    },
};

use crate::{
    theme::{Accent, ColorScheme},
    widget::{
        self, OnPress,
        button::{self},
        common_icons,
        fab::{self, Fab},
        hybrid_icon::Icon,
    },
};

pub use crate::widget::advanced::drop_down_menu::Placement;

const BUTTON_SIZE: fab::Size = fab::Size::Regular;
const BUTTON_TRIGGER_BETWEEN_SPACE: f32 = 8.0;
const BUTTON_SPACING: f32 = 4.0;

pub struct Item<'a, Message>
where
    Message: Clone,
{
    pub on_press: OnPress<'a, Message>,
    pub label: text::Fragment<'a>,
    pub icon: Icon<'a>,
}

pub struct FABMenu<'a, Message>
where
    Message: Clone,
{
    close_button_expanded: Element<'a, Message>,
    close_button_collapsed: Element<'a, Message>,
    items: Vec<Element<'a, Message>>,
}

impl<'a, Message> FABMenu<'a, Message>
where
    Message: 'a + Clone,
{
    #[must_use]
    pub fn new<I>(
        items: I,
        size: fab::Size,
        label_font: Option<Font>,
        theme: &(impl ColorScheme + ?Sized),
        accent: Accent,
    ) -> Self
    where
        I: IntoIterator<Item = Item<'a, Message>>,
    {
        let close_button_expanded = Fab::new_dummy(
            button::Style::fab_vibrant(theme, accent),
            fab::Content::Reguar {
                icon: Icon::Svg(svg::Handle::from_memory(common_icons::CLOSE)),
            },
        )
        .corner_style(button::CornerStyle::Custom {
            resting: f32::MAX.into(),
            pressed: f32::MAX.into(),
        })
        .label_font_maybe(label_font)
        .size(size)
        .into();

        let close_button_collapsed = Fab::new_dummy(
            button::Style::fab_tonal(theme, accent),
            fab::Content::Reguar {
                icon: Icon::Svg(svg::Handle::from_memory(common_icons::ADD)),
            },
        )
        .label_font_maybe(label_font)
        .size(size)
        .into();

        let items = items
            .into_iter()
            .map(|i| {
                widget::fab(
                    button::Style::fab_tonal(theme, accent),
                    widget::fab::Content::Extended {
                        icon: i.icon,
                        label: i.label,
                    },
                    i.on_press,
                )
                .corner_style(button::CornerStyle::Custom {
                    resting: f32::MAX.into(),
                    pressed: f32::MAX.into(),
                })
                .label_font_maybe(label_font)
                .size(BUTTON_SIZE)
                .into()
            })
            .collect();

        Self {
            close_button_expanded,
            close_button_collapsed,
            items,
        }
    }
}

#[derive(Default)]
struct State {
    is_opened: bool,
    is_pressed: bool,
}

impl<'a, Message> Widget<Message, iced::Theme, iced::Renderer> for FABMenu<'a, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> iced::Size<iced::Length> {
        self.close_button_collapsed.as_widget().size()
    }

    fn children(&self) -> Vec<Tree> {
        let mut children = Vec::with_capacity(1 + self.items.len());

        children.push(Tree::new(&self.close_button_collapsed));
        children.extend(self.items.iter().map(Tree::new));

        children
    }

    fn diff(&self, tree: &mut Tree) {
        let mut children = Vec::with_capacity(1 + self.items.len());

        children.push(&self.close_button_collapsed);
        children.extend(self.items.iter());

        tree.diff_children(&children);
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &iced::Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        self.close_button_collapsed
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, &limits)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let widget = match state.is_opened {
            true => &self.close_button_expanded,
            false => &self.close_button_collapsed,
        };
        widget.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.close_button_collapsed.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        match event {
            iced::Event::Mouse(iced::mouse::Event::ButtonPressed(_)) => {
                if cursor.is_over(layout.bounds()) {
                    let state = tree.state.downcast_mut::<State>();
                    state.is_pressed = true;
                    shell.capture_event();
                }
            }
            iced::Event::Mouse(iced::mouse::Event::ButtonReleased(_)) => {
                let state = tree.state.downcast_mut::<State>();
                if state.is_pressed {
                    state.is_pressed = false;

                    if cursor.is_over(layout.bounds()) {
                        state.is_opened = !state.is_opened;
                    }
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> iced::advanced::mouse::Interaction {
        self.close_button_collapsed.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'b>,
        _renderer: &iced::Renderer,
        _viewport: &Rectangle,
        _translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, iced::Theme, iced::Renderer>> {
        let state = tree.state.downcast_ref::<State>();
        state
            .is_opened
            .then_some(overlay::Element::new(Box::new(Overlay {
                base_bounds: layout.bounds(),
                tree,
                items: &mut self.items,
            })))
    }
}

impl<'a, Message> From<FABMenu<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(menu: FABMenu<'a, Message>) -> Self {
        Element::new(menu)
    }
}

struct Overlay<'a, 'b, Message> {
    base_bounds: Rectangle,
    tree: &'b mut Tree,
    items: &'b mut Vec<Element<'a, Message>>,
}

impl<'a, 'b, Message> iced::advanced::Overlay<Message, iced::Theme, iced::Renderer>
    for Overlay<'a, 'b, Message>
{
    fn layout(
        &mut self,
        renderer: &iced::Renderer,
        bounds: iced::Size,
    ) -> iced::advanced::layout::Node {
        let item_height = BUTTON_SIZE.container_height();
        let children: Vec<_> = self
            .items
            .iter_mut()
            .enumerate()
            .map(|(i, item)| {
                let item = item.as_widget_mut().layout(
                    &mut self.tree.children[i + 1],
                    renderer,
                    &Limits::new(Size::ZERO, bounds),
                );

                let y = -item_height * (i + 1) as f32
                    - BUTTON_TRIGGER_BETWEEN_SPACE
                    - BUTTON_SPACING * i as f32;
                let x = self.base_bounds.width - item.size().width;

                item.move_to(Point::new(x, y))
            })
            .collect();

        Node::with_children(self.base_bounds.size(), children).move_to(self.base_bounds.position())
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
    ) {
        let viewport = iced::Rectangle::new(iced::Point::ORIGIN, layout.bounds().size());
        for (i, item) in self.items.iter().enumerate() {
            item.as_widget().draw(
                &self.tree.children[i + 1],
                renderer,
                theme,
                style,
                layout.child(i),
                cursor,
                &viewport,
            );
        }
    }

    fn update(
        &mut self,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
    ) {
        let viewport = iced::Rectangle::new(iced::Point::ORIGIN, layout.bounds().size());
        for (i, item) in self.items.iter_mut().enumerate() {
            item.as_widget_mut().update(
                &mut self.tree.children[i + 1],
                event,
                layout.child(i),
                cursor,
                renderer,
                clipboard,
                shell,
                &viewport,
            );
        }

        let should_close = match event {
            Event::Mouse(iced::mouse::Event::ButtonPressed(_)) => {
                !shell.is_event_captured() && !cursor.is_over(self.base_bounds)
            }
            Event::Mouse(iced::mouse::Event::ButtonReleased(_)) => {
                !cursor.is_over(self.base_bounds)
            }
            _ => false,
        };

        if should_close {
            let state = self.tree.state.downcast_mut::<State>();
            state.is_opened = false;
            shell.invalidate_widgets();
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &iced::Renderer,
    ) -> iced::advanced::mouse::Interaction {
        for (i, _) in self.items.iter().enumerate() {
            if cursor.is_over(layout.child(i).bounds()) {
                return iced::mouse::Interaction::Pointer;
            }
        }

        iced::mouse::Interaction::None
    }
}
