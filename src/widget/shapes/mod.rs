mod values;

use iced::Element;
use iced_widget::{
    Canvas,
    canvas::{self, Cache, Fill, LineCap, LineDash, LineJoin, Stroke, fill::Rule},
};

use crate::{
    theme::ColorScheme,
    widget::shapes::values::{
        ARCH, ARROW, BOOM, BUN, BURST, CIRCLE, CLAM_SHELL, CLOVER_4_LEAF, CLOVER_8_LEAF,
        COOKIE_4_SIDED, COOKIE_6_SIDED, COOKIE_7_SIDED, COOKIE_9_SIDED, COOKIE_12_SIDED, DIAMOND,
        FAN, FLOWER, GEM, GHOSTISH, HEART, OVAL, PENTAGON, PILL, PIXEL_CIRCLE, PIXEL_TRIANGLE,
        PUFFY, PUFFY_DIAMOND, Point, SEMI_CIRCLE, SLANTED, SOFT_BOOM, SOFT_BURST, SQUARE, SUNNY,
        TRIANGLE, VERY_SUNNY,
    },
};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    MoveTo(Point),
    LineTo(Point),
    QuadTo {
        control: Point,
        to: Point,
    },
    CubicTo {
        control_1: Point,
        control_2: Point,
        to: Point,
    },
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shape {
    Circle,
    Square,
    Slanted,
    Arch,
    Fan,
    Arrow,
    SemiCircle,
    Oval,
    Pill,
    Triangle,
    Diamond,
    ClamShell,
    Pentagon,
    Gem,
    Sunny,
    VerySunny,
    Cookie4Sided,
    Cookie6Sided,
    Cookie7Sided,
    Cookie9Sided,
    Cookie12Sided,
    Ghostish,
    Clover4Leaf,
    Clover8Leaf,
    Burst,
    SoftBurst,
    Boom,
    SoftBoom,
    Flower,
    Puffy,
    PuffyDiamond,
    PixelCircle,
    PixelTriangle,
    Bun,
    Heart,
}

impl Shape {
    pub fn data(&self) -> values::ShapeData {
        match self {
            Self::Circle => CIRCLE,
            Self::Square => SQUARE,
            Self::Slanted => SLANTED,
            Self::Arch => ARCH,
            Self::Fan => FAN,
            Self::Arrow => ARROW,
            Self::SemiCircle => SEMI_CIRCLE,
            Self::Oval => OVAL,
            Self::Pill => PILL,
            Self::Triangle => TRIANGLE,
            Self::Diamond => DIAMOND,
            Self::ClamShell => CLAM_SHELL,
            Self::Pentagon => PENTAGON,
            Self::Gem => GEM,
            Self::Sunny => SUNNY,
            Self::VerySunny => VERY_SUNNY,
            Self::Cookie4Sided => COOKIE_4_SIDED,
            Self::Cookie6Sided => COOKIE_6_SIDED,
            Self::Cookie7Sided => COOKIE_7_SIDED,
            Self::Cookie9Sided => COOKIE_9_SIDED,
            Self::Cookie12Sided => COOKIE_12_SIDED,
            Self::Ghostish => GHOSTISH,
            Self::Clover4Leaf => CLOVER_4_LEAF,
            Self::Clover8Leaf => CLOVER_8_LEAF,
            Self::Burst => BURST,
            Self::SoftBurst => SOFT_BURST,
            Self::Boom => BOOM,
            Self::SoftBoom => SOFT_BOOM,
            Self::Flower => FLOWER,
            Self::Puffy => PUFFY,
            Self::PuffyDiamond => PUFFY_DIAMOND,
            Self::PixelCircle => PIXEL_CIRCLE,
            Self::PixelTriangle => PIXEL_TRIANGLE,
            Self::Bun => BUN,
            Self::Heart => HEART,
        }
        .into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Style<'a> {
    pub fill: Fill,
    pub stroke: Stroke<'a>,
}

impl<'a> Style<'a> {
    pub fn new(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self {
            fill: Fill {
                style: canvas::Style::Solid(theme.primary()),
                rule: Rule::NonZero,
            },
            stroke: Stroke {
                style: canvas::Style::Solid(theme.on_primary()),
                width: 0.0,
                line_cap: LineCap::default(),
                line_join: LineJoin::default(),
                line_dash: LineDash::default(),
            },
        }
    }
}

pub struct MaterialShape<'a> {
    shape: Shape,
    style: Style<'a>,
    size: f32,
    cache: Cache,
}

impl<'a> MaterialShape<'a> {
    #[must_use]
    pub fn new(shape: Shape, style: Style<'a>) -> Self {
        Self {
            shape,
            style,
            size: 100.0,
            cache: Cache::default(),
        }
    }

    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl<'a, Message, Theme> canvas::Program<Message, Theme, iced::Renderer> for MaterialShape<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let path = self.shape.data().path();

        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            frame.with_save(|frame| {
                frame.scale(self.size);

                frame.fill(&path, self.style.fill);

                if self.style.stroke.width > 0.0 {
                    frame.stroke(&path, self.style.stroke);
                }
            });
        });

        vec![geometry]
    }
}

impl<'a, Message, Theme> From<MaterialShape<'a>> for Element<'a, Message, Theme, iced::Renderer>
where
    Message: 'a,
    Theme: 'a,
{
    fn from(value: MaterialShape<'a>) -> Self {
        let size = value.size;
        Element::new(Canvas::new(value).width(size).height(size))
    }
}
