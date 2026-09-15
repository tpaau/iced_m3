use crate::theme::ColorScheme;
use iced::{
    Border, Color, Element, Pixels, Rectangle, Size, Vector,
    advanced::{
        Widget,
        layout::Node,
        text::{self, Paragraph},
        widget::tree,
    },
};

const LARGE_LABEL_SIZE: f32 = 16.0;
const LARGE_BADGE_OFFSET: Vector<f32> = Vector::new(4.0, -2.0);
const BADGE_CONTENT_PADDING: f32 = 4.0;
const LABEL_FONT_SIZE: f32 = 11.0;
const LABEL_MAX_WIDTH: f32 = 34.0;

struct State<P>
where
    P: iced::advanced::text::Paragraph,
{
    badge_bounds: Option<Rectangle>,
    paragraph: Option<P>,
}

impl<P> State<P>
where
    P: iced::advanced::text::Paragraph,
{
    fn new() -> Self {
        Self {
            badge_bounds: None,
            paragraph: None,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoundsMode {
    #[default]
    Real,
    Symmetrical,
}

pub struct Badge<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced_widget::core::text::Renderer,
{
    theme: &'a dyn ColorScheme,
    base: Element<'a, Message, Theme, Renderer>,
    label: Option<&'a str>,
    label_font: Option<Renderer::Font>,
    container_color: Option<Color>,
    label_color: Option<Color>,
    bounds_mode: BoundsMode,
}

impl<'a, Message, Theme, Renderer> Badge<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced_widget::core::text::Renderer,
{
    #[must_use]
    pub fn new(
        theme: &'a impl ColorScheme,
        base: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            theme,
            base: base.into(),
            label: None,
            label_font: None,
            container_color: None,
            label_color: None,
            bounds_mode: BoundsMode::default(),
        }
    }

    #[must_use]
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    #[must_use]
    pub fn label_maybe(mut self, label: Option<&'a str>) -> Self {
        self.label = label;
        self
    }

    #[must_use]
    pub fn label_font(mut self, font: Renderer::Font) -> Self {
        self.label_font = Some(font);
        self
    }

    #[must_use]
    pub fn label_font_maybe(mut self, font: Option<Renderer::Font>) -> Self {
        self.label_font = font;
        self
    }

    #[must_use]
    pub fn container_color(mut self, color: Color) -> Self {
        self.container_color = Some(color);
        self
    }

    #[must_use]
    pub fn container_color_maybe(mut self, color: Option<Color>) -> Self {
        self.container_color = color;
        self
    }

    #[must_use]
    pub fn label_color(mut self, color: Color) -> Self {
        self.label_color = Some(color);
        self
    }

    #[must_use]
    pub fn label_color_maybe(mut self, color: Option<Color>) -> Self {
        self.label_color = color;
        self
    }

    #[must_use]
    pub fn bounds_mode(mut self, mode: BoundsMode) -> Self {
        self.bounds_mode = mode;
        self
    }

    #[must_use]
    pub fn bounds_mode_maybe(self, mode_maybe: Option<BoundsMode>) -> Self {
        match mode_maybe {
            Some(mode) => self.bounds_mode(mode),
            None => self,
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Badge<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced_widget::core::text::Renderer,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        tree::Tag::of::<State<Renderer::Paragraph>>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        tree::State::new(State::<Renderer::Paragraph>::new())
    }

    fn size(&self) -> iced::Size<iced::Length> {
        self.base.as_widget().size()
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        tree.diff_children(&[self.base.as_widget()]);
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        vec![iced::advanced::widget::Tree::new(self.base.as_widget())]
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        match self.label {
            Some(label) => {
                let mut node =
                    self.base
                        .as_widget_mut()
                        .layout(&mut tree.children[0], renderer, limits);

                node.translate_mut(Vector { x: 0.0, y: 2.0 });
                let base_bounds = node.bounds();

                let state = tree.state.downcast_mut::<State<Renderer::Paragraph>>();

                let paragraph = <Renderer as text::Renderer>::Paragraph::with_text(text::Text {
                    content: label,
                    bounds: Size::INFINITE,
                    size: Pixels(LABEL_FONT_SIZE),
                    line_height: text::LineHeight::Relative(1.0),
                    font: self.label_font.unwrap_or(renderer.default_font()),
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                    align_x: text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                });

                let width = (paragraph.min_bounds().width + 2.0 * BADGE_CONTENT_PADDING)
                    .clamp(LARGE_LABEL_SIZE, LABEL_MAX_WIDTH);

                let node_offset = if self.bounds_mode == BoundsMode::Symmetrical {
                    let offset = width - LARGE_LABEL_SIZE + LARGE_BADGE_OFFSET.x;
                    node.translate_mut(Vector { x: offset, y: 0.0 });
                    offset + base_bounds.width - LARGE_LABEL_SIZE + LARGE_BADGE_OFFSET.x
                } else {
                    base_bounds.width - LARGE_LABEL_SIZE + LARGE_BADGE_OFFSET.x
                };

                let badge = Rectangle {
                    x: node_offset,
                    y: 0.0,
                    width,
                    height: LARGE_LABEL_SIZE,
                };

                state.paragraph = Some(paragraph);
                state.badge_bounds = Some(badge);

                match self.bounds_mode {
                    BoundsMode::Real => {
                        let base_size = base_bounds.union(&badge).size();
                        Node::with_children(base_size, vec![node])
                    }
                    BoundsMode::Symmetrical => {
                        let badge_inverse = Rectangle {
                            x: base_bounds.x + badge.width,
                            y: badge.y + LARGE_BADGE_OFFSET.y,
                            width: badge.width,
                            height: badge.height,
                        };
                        let base_size = base_bounds.union(&badge).union(&badge_inverse).size();
                        Node::with_children(base_size, vec![node])
                    }
                }
            }
            None => {
                let node =
                    self.base
                        .as_widget_mut()
                        .layout(&mut tree.children[0], renderer, limits);

                Node::with_children(node.bounds().size(), vec![node])
            }
        }
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
        let base_layout = layout.children().next().unwrap();
        self.base.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            base_layout,
            cursor,
            viewport,
        );

        let bounds = layout.bounds();
        if self.label.is_some() {
            let state = tree.state.downcast_ref::<State<Renderer::Paragraph>>();
            let paragraph = state.paragraph.as_ref().unwrap();
            let badge_bounds = state.badge_bounds.clone().unwrap();

            renderer.with_layer(bounds, |renderer| {
                renderer.fill_quad(
                    iced::advanced::renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x + badge_bounds.x,
                            y: bounds.y + badge_bounds.y,
                            width: badge_bounds.width,
                            height: badge_bounds.height,
                        },
                        border: Border::default().rounded(f32::MAX),
                        ..Default::default()
                    },
                    self.container_color.unwrap_or(self.theme.error()),
                );

                renderer.fill_paragraph(
                    &paragraph,
                    bounds.position()
                        + Vector {
                            x: badge_bounds.x + BADGE_CONTENT_PADDING,
                            y: badge_bounds.y + (badge_bounds.height - LABEL_FONT_SIZE) / 2.0,
                        },
                    self.label_color.unwrap_or(self.theme.on_error()),
                    bounds,
                );
            });
        } else {
            let size = 6.0;
            renderer.fill_quad(
                iced::advanced::renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x + bounds.width - size,
                        y: bounds.y,
                        width: size,
                        height: size,
                    },
                    border: Border::default().rounded(f32::MAX),
                    ..Default::default()
                },
                self.container_color.unwrap_or(self.theme.error()),
            );
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Badge<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + iced_widget::core::text::Renderer,
{
    fn from(value: Badge<'a, Message, Theme, Renderer>) -> Self {
        Element::new(value)
    }
}
