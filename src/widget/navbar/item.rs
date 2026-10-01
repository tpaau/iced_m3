use iced::advanced::Widget;
use iced::{Font, Size};

const LABEL_SIZE: f32 = 12.0;
const LABEL_LINE_HEIGHT: f32 = 16.0;
const ICON_LABEL_SPACE: f32 = 4.0;

const INDICATOR_SMALL_SIZE: Size = Size::new(56.0, 32.0);
const INDICATOR_SMALL_TOTAL_HEIGHT: f32 =
    INDICATOR_SMALL_SIZE.height + ICON_LABEL_SPACE + LABEL_LINE_HEIGHT;

const INDICATOR_HEIGHT: f32 = 40.0;

use crate::animation::motion::{self, Spring, default_effects, fast_effects};
pub use crate::widget::internal::navigation_common::NavigationItemContent as Content;
pub use crate::widget::internal::navigation_common::NavigationItemStyle as Style;
pub use crate::widget::internal::navigation_common::{Colors, State};

pub struct Item<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    expanded: bool,
    style: Style,
    content: Content<'a, Message>,
    label_font: Option<Font>,
    active: bool,
    active_transition_spring: Option<Spring>,
    color_spring: Option<Spring>,

    // Cache
    label_paragraph: Option<Renderer::Paragraph>,
    icon_paragraph: Option<Renderer::Paragraph>,
}

impl<'a, Message, Renderer> Item<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    #[must_use]
    pub fn new(
        style: Style,
        content: Content<'a, Message>,
        expanded: bool,
        label_font: Option<Font>,
        active: bool,
    ) -> Self {
        Self {
            expanded,
            style,
            content,
            label_font,
            active,
            active_transition_spring: None,
            color_spring: None,
            label_paragraph: None,
            icon_paragraph: None,
        }
    }

    pub fn motion_scheme_maybe(mut self, scheme: Option<motion::Scheme>) -> Self {
        if let Some(scheme) = scheme {
            self.color_spring = Some(fast_effects(scheme));
            self.active_transition_spring = Some(default_effects(scheme));
        }
        self
    }

    #[must_use]
    pub fn active_transition_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.active_transition_spring = spring;
        self
    }

    #[must_use]
    pub fn color_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.color_spring = spring;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Item<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::text::Renderer + iced::advanced::svg::Renderer,
    Renderer::Font: From<iced::Font>,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        todo!()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        todo!()
    }

    fn size(&self) -> Size<iced::Length> {
        todo!()
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        todo!()
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        todo!()
    }

    fn update(
        &mut self,
        _tree: &mut iced::advanced::widget::Tree,
        _event: &iced::Event,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) {
        todo!()
    }

    fn mouse_interaction(
        &self,
        _tree: &iced::advanced::widget::Tree,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        todo!()
    }
}
