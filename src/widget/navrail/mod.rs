mod item;

use iced::{
    Color, Element, Font, Length, Padding, Pixels, Size, Vector,
    advanced::{Widget, layout::Node, mouse, svg, text, widget::Tree},
};

use crate::{
    animation::motion::{self, Spring},
    style::StateLayer,
    theme::ColorScheme,
    widget::{
        self, OnPress, badge,
        button::{self, ElevationStates, Outline},
        common_icons::{self},
        hybrid_icon::Icon,
        navrail::item::{INDICATOR_LARGE_HEIGHT, INDICATOR_SMALL_TOTAL_HEIGHT},
    },
};

pub use item::Content as Item;
pub use item::Style as ItemStyle;

const COLLAPSED_ITEM_SPACE: f32 = 4.0;
const MENU_ICON_SIZE: f32 = 24.0;
const MENU_BUTTON_PADDING: f32 = 4.0;
const SECTION_SPACE: f32 = 16.0;
// Small FAB container size
const FAB_OFFSET: f32 = (constants::CONTAINER_COLLAPSED_WIDTH - 56.0) / 2.0;
const MENU_OFFSET: f32 =
    (constants::CONTAINER_COLLAPSED_WIDTH - MENU_ICON_SIZE - MENU_BUTTON_PADDING * 2.0) / 2.0;

mod constants {
    use crate::widget::navrail::item::INDICATOR_SMALL_WIDTH;

    pub const CONTAINER_COLLAPSED_WIDTH: f32 = 96.0;
    pub const CONTAINER_EXPANDED_MIN_WIDTH: f32 = 220.0;
    pub const CONTAINER_EXPANDED_MAX_WIDTH: f32 = 360.0;
    pub const ITEM_OFFSET: f32 = (CONTAINER_COLLAPSED_WIDTH - INDICATOR_SMALL_WIDTH) / 2.0;
    pub const CONTAINER_VERTICAL_PADDING: f32 = 44.0;
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemAlignment {
    #[default]
    Top,
    Center,
    Bottom,
}

pub struct Fab<'a, Message>
where
    Message: Clone,
{
    pub icon: Icon<'a>,
    pub label: text::Fragment<'a>,
    pub style: crate::widget::fab::Style,
    pub on_press: OnPress<'a, Message>,
}

#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum Mode {
    #[default]
    Collapsed,
    Expanded {
        /// The width of the navigation rail container.
        ///
        ///This value will be clamped between the minimum and maximum container width.
        width: Pixels,
    },
}

impl Mode {
    fn width(&self) -> Pixels {
        match self {
            Mode::Collapsed => Pixels(constants::CONTAINER_COLLAPSED_WIDTH),
            Mode::Expanded { width } => Pixels(Into::<f32>::into(*width).clamp(
                constants::CONTAINER_EXPANDED_MIN_WIDTH,
                constants::CONTAINER_EXPANDED_MAX_WIDTH,
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub menu_button_style: button::Style,
    pub item_style: ItemStyle,
    pub container_color: Color,
    pub badge_style: badge::Style,
}

impl Style {
    pub fn new(theme: &(impl ColorScheme + ?Sized)) -> Self {
        let style = button::StateStyle {
            container: Color::TRANSPARENT,
            label: Color::TRANSPARENT,
            icon: theme.on_surface(),
            outline: Outline::default(),
        };
        Self {
            menu_button_style: button::Style {
                regular: style,
                unselected: style,
                selected: style,
                disabled: style,
                disabled_unselected: style,
                disabled_selected: style,
                elevation: ElevationStates::new(theme.shadow()),
                state_layer: StateLayer::new(theme.on_surface()),
                state_layer_unselected: StateLayer::new(theme.on_surface()),
                state_layer_selected: StateLayer::new(theme.on_surface()),
            },
            item_style: ItemStyle::new(theme),
            container_color: Color::TRANSPARENT,
            badge_style: badge::Style::new(theme),
        }
    }
}

pub struct NavRail<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    style: Style,
    mode: Mode,
    on_menu_pressed: Option<Box<dyn Fn(Mode) -> Message + 'a>>,
    fab: Option<Fab<'a, Message>>,
    items: Vec<Item<'a, Message>>,
    label_font: Option<Font>,
    active_index: usize,
    item_alignment: ItemAlignment,
    container_vertical_padding: Option<f32>,
    motion_scheme: Option<motion::Scheme>,
    item_active_transition_spring: Option<Spring>,
    item_color_spring: Option<Spring>,

    menu_button: Option<Element<'a, Message, Theme, Renderer>>,
    fab_element: Option<Element<'a, Message, Theme, Renderer>>,
    item_elements: Vec<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> NavRail<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    #[must_use]
    pub fn new(style: Style, items: Vec<Item<'a, Message>>) -> Self {
        Self {
            style,
            mode: Mode::default(),
            on_menu_pressed: None,
            fab: None,
            items,
            label_font: None,
            active_index: 0,
            item_alignment: ItemAlignment::default(),
            container_vertical_padding: None,
            motion_scheme: None,
            item_active_transition_spring: None,
            item_color_spring: None,
            menu_button: None,
            fab_element: None,
            item_elements: Vec::new(),
        }
    }

    #[must_use]
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    #[must_use]
    pub fn mode_maybe(self, maybe_mode: Option<Mode>) -> Self {
        match maybe_mode {
            Some(mode) => self.mode(mode),
            None => self,
        }
    }

    #[must_use]
    pub fn fab(mut self, fab: Fab<'a, Message>) -> Self {
        self.fab = Some(fab);
        self
    }

    #[must_use]
    pub fn fab_maybe(mut self, maybe_fab: Option<Fab<'a, Message>>) -> Self {
        self.fab = maybe_fab;
        self
    }

    #[must_use]
    pub fn on_menu_pressed(mut self, on_press: impl Fn(Mode) -> Message + 'a) -> Self {
        self.on_menu_pressed = Some(Box::new(on_press));
        self
    }

    #[must_use]
    pub fn on_menu_pressed_maybe(
        mut self,
        on_press: Option<impl Fn(Mode) -> Message + 'a>,
    ) -> Self {
        self.on_menu_pressed =
            on_press.map(|callback| Box::new(callback) as Box<dyn Fn(Mode) -> Message + 'a>);
        self
    }

    #[must_use]
    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }

    #[must_use]
    pub fn label_font(mut self, font: iced::Font) -> Self {
        self.label_font = Some(font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, maybe_font: Option<iced::Font>) -> Self {
        self.label_font = maybe_font;
        self
    }

    #[must_use]
    pub fn item_alignment(mut self, alignment: ItemAlignment) -> Self {
        self.item_alignment = alignment;
        self
    }

    #[must_use]
    pub fn item_alignment_maybe(self, maybe_alignment: Option<ItemAlignment>) -> Self {
        match maybe_alignment {
            Some(alignment) => self.item_alignment(alignment),
            None => self,
        }
    }

    #[must_use]
    pub fn container_vertical_padding(mut self, padding: f32) -> Self {
        self.container_vertical_padding = Some(padding);
        self
    }

    #[must_use]
    pub fn container_vertical_padding_maybe(mut self, maybe_padding: Option<f32>) -> Self {
        self.container_vertical_padding = maybe_padding;
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
    pub fn item_active_transition_spring(mut self, spring: Spring) -> Self {
        self.item_active_transition_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn item_active_transition_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.item_active_transition_spring = spring;
        self
    }

    #[must_use]
    pub fn item_color_spring(mut self, spring: Spring) -> Self {
        self.item_color_spring = Some(spring);
        self
    }

    #[must_use]
    pub fn item_color_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.item_color_spring = spring;
        self
    }
}

struct State {}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for NavRail<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        let state = State {};

        iced::advanced::widget::tree::State::new(state)
    }

    fn diff(&self, tree: &mut Tree) {
        let capacity = self.item_elements.len()
            + self.menu_button.is_some() as usize
            + self.fab_element.is_some() as usize;
        let mut children = Vec::with_capacity(capacity);

        self.menu_button.as_ref().map(|menu| children.push(menu));
        self.fab_element.as_ref().map(|fab| children.push(fab));
        children.extend(self.item_elements.iter());

        tree.diff_children(&children);
    }

    fn children(&self) -> Vec<Tree> {
        let capacity = self.item_elements.len()
            + self.menu_button.is_some() as usize
            + self.fab_element.is_some() as usize;
        let mut children = Vec::with_capacity(capacity);

        self.menu_button
            .as_ref()
            .map(|menu| children.push(Tree::new(menu)));
        self.fab_element
            .as_ref()
            .map(|fab| children.push(Tree::new(fab)));
        children.extend(self.item_elements.iter().map(Tree::new));

        children
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.mode.width().0), Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let container_bounds =
            limits.resolve(Length::Fixed(self.mode.width().0), Length::Fill, Size::ZERO);
        let container_vertical_padding = self
            .container_vertical_padding
            .unwrap_or(constants::CONTAINER_VERTICAL_PADDING);
        let mut offset = container_vertical_padding;
        let section_space = SECTION_SPACE;

        let capacity = self.item_elements.len()
            + self.menu_button.is_some() as usize
            + self.fab_element.is_some() as usize;
        let mut children = Vec::with_capacity(capacity);
        let mut tree_index = 0;

        let menu_button_layout = self.menu_button.as_mut().map(|menu| {
            let index = tree_index;
            tree_index += 1;

            let layout = menu
                .as_widget_mut()
                .layout(&mut tree.children[index], renderer, limits)
                .translate(Vector::new(MENU_OFFSET, offset));

            offset += layout.bounds().height + section_space;

            layout
        });

        let fab_layout = self.fab_element.as_mut().map(|fab| {
            let index = tree_index;
            tree_index += 1;

            let layout = fab
                .as_widget_mut()
                .layout(&mut tree.children[index], renderer, limits)
                .translate(Vector::new(FAB_OFFSET, offset));

            offset += layout.bounds().height + section_space;

            layout
        });

        menu_button_layout.map(|layout| children.push(layout));
        fab_layout.map(|layout| children.push(layout));

        let item_space = match self.mode {
            Mode::Collapsed => COLLAPSED_ITEM_SPACE,
            Mode::Expanded { width: _ } => 0.0,
        };
        let item_height = match self.mode {
            Mode::Collapsed => INDICATOR_SMALL_TOTAL_HEIGHT,
            Mode::Expanded { width: _ } => INDICATOR_LARGE_HEIGHT,
        };
        let total_height = item_height * self.item_elements.len() as f32
            + item_space * (self.item_elements.len() - 1) as f32;
        match self.item_alignment {
            ItemAlignment::Center => {
                offset = ((container_bounds.height - container_vertical_padding + offset
                    - total_height)
                    / 2.0)
                    .max(offset)
            }
            ItemAlignment::Bottom => {
                offset = container_bounds.height - container_vertical_padding - total_height;
            }
            ItemAlignment::Top => {}
        };

        for item in &mut self.item_elements {
            let index = tree_index;
            tree_index += 1;

            let layout = item
                .as_widget_mut()
                .layout(&mut tree.children[index], renderer, limits)
                .translate(Vector::new(constants::ITEM_OFFSET, offset));

            offset += layout.bounds().height + item_space;

            children.push(layout);
        }

        Node::with_children(container_bounds, children)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let mut layouts = layout.children();
        let mut tree_index = 0;

        if let Some(menu) = &self.menu_button {
            let child_layout = layouts.next().unwrap();

            menu.as_widget().draw(
                &tree.children[tree_index],
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );

            tree_index += 1;
        }

        if let Some(fab) = &self.fab_element {
            let child_layout = layouts.next().unwrap();

            fab.as_widget().draw(
                &tree.children[tree_index],
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );

            tree_index += 1;
        }

        for item in &self.item_elements {
            let child_layout = layouts.next().unwrap();

            item.as_widget().draw(
                &tree.children[tree_index],
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );

            tree_index += 1;
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut tree_index = 0;
        let mut layouts = layout.children();

        if let Some(menu_button) = &self.menu_button {
            let interaction = menu_button.as_widget().mouse_interaction(
                &tree.children[tree_index],
                layouts.next().unwrap(),
                cursor,
                viewport,
                renderer,
            );
            tree_index += 1;
            if interaction != mouse::Interaction::None {
                return interaction;
            }
        }

        if let Some(fab) = &self.fab_element {
            let interaction = fab.as_widget().mouse_interaction(
                &tree.children[tree_index],
                layouts.next().unwrap(),
                cursor,
                viewport,
                renderer,
            );
            tree_index += 1;
            if interaction != mouse::Interaction::None {
                return interaction;
            }
        }

        for item in &self.item_elements {
            let interaction = item.as_widget().mouse_interaction(
                &tree.children[tree_index],
                layouts.next().unwrap(),
                cursor,
                viewport,
                renderer,
            );
            tree_index += 1;
            if interaction != mouse::Interaction::None {
                return interaction;
            }
        }

        mouse::Interaction::None
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        let mut layouts = layout.children();
        let mut tree_index = 0;

        if let Some(menu) = &mut self.menu_button {
            let child_layout = layouts.next().unwrap();

            menu.as_widget_mut().update(
                &mut tree.children[tree_index],
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );

            tree_index += 1;
        }

        if let Some(fab) = &mut self.fab_element {
            let child_layout = layouts.next().unwrap();

            fab.as_widget_mut().update(
                &mut tree.children[tree_index],
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );

            tree_index += 1;
        }

        for item in &mut self.item_elements {
            let child_layout = layouts.next().unwrap();

            item.as_widget_mut().update(
                &mut tree.children[tree_index],
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );

            tree_index += 1;
        }
    }
}

impl<'a, Message, Theme, Renderer> From<NavRail<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn from(mut value: NavRail<'a, Message, Theme, Renderer>) -> Self {
        value.menu_button = value.on_menu_pressed.as_ref().map(|on_press| {
            let icon = match value.mode {
                Mode::Collapsed => svg::Handle::from_memory(common_icons::MENU),
                Mode::Expanded { width: _ } => svg::Handle::from_memory(common_icons::MENU_OPEN),
            };
            let button_size = Pixels(MENU_ICON_SIZE + MENU_BUTTON_PADDING * 2.0);
            crate::widget::button(
                value.style.menu_button_style,
                button::Content::Icon(Icon::Svg(icon)),
            )
            .size(button::Size {
                width: Length::Fixed(button_size.0),
                height: button_size,
                spacing: Pixels::ZERO,
                padding: Padding::default(),
                icon_size: MENU_ICON_SIZE,
                label_size: 0.0,
                corner_radius: button::CornerRadius {
                    style: button::CornerStyle::Rounded,
                    shape_morph: false,
                    rounded: f32::MAX.into(),
                    square: f32::MAX.into(),
                    pressed: f32::MAX.into(),
                },
            })
            .on_press((on_press)(value.mode))
            .into()
        });

        let fab = std::mem::take(&mut value.fab);
        value.fab_element = fab.map(|fab| {
            let content = match value.mode {
                Mode::Expanded { width: _ } => widget::fab::Content::Extended {
                    icon: fab.icon,
                    label: fab.label,
                },
                Mode::Collapsed => widget::fab::Content::Reguar { icon: fab.icon },
            };

            widget::fab(fab.style, content, fab.on_press)
                .label_font_maybe(value.label_font)
                .into()
        });

        let items = std::mem::take(&mut value.items);
        value.item_elements = items
            .into_iter()
            .enumerate()
            .map(|(index, content)| {
                let active = index == value.active_index;
                let expanded = matches!(value.mode, Mode::Expanded { width: _ });
                item::Item::new(
                    value.style.item_style,
                    content,
                    expanded,
                    value.label_font,
                    active,
                )
                .motion_scheme_maybe(value.motion_scheme)
                .active_transition_spring_maybe(value.item_active_transition_spring)
                .color_spring_maybe(value.item_color_spring)
                .into()
            })
            .collect();

        Element::new(value)
    }
}
