use iced::{Element, Point, Size};
use iced_widget::{
    Canvas,
    canvas::{self, Cache, Fill, LineCap, LineDash, LineJoin, Path, Stroke, fill::Rule},
};

pub use polymorpher;
use polymorpher::RoundedPolygon;

use crate::theme::ColorScheme;

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
    polygon: RoundedPolygon,
    style: Style<'a>,
    size: f32,
    cache: Cache,
}

impl<'a> MaterialShape<'a> {
    #[must_use]
    pub fn new(polygon: RoundedPolygon, style: Style<'a>) -> Self {
        Self {
            polygon,
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

pub fn to_iced_path(polygon: &RoundedPolygon, size: Size) -> Path {
    let polygon = polygon.clone().normalized();
    let side = size.width.min(size.height);
    let offset_x = (size.width - side) * 0.5;
    let offset_y = (size.height - side) * 0.5;

    let map =
        |p: polymorpher::geometry::Point| Point::new(offset_x + p.x * side, offset_y + p.y * side);

    Path::new(|builder| {
        if let Some(first) = polygon.cubics.first() {
            builder.move_to(map(first.anchor0()));

            for cubic in &polygon.cubics {
                builder.bezier_curve_to(
                    map(cubic.control0()),
                    map(cubic.control1()),
                    map(cubic.anchor1()),
                );
            }

            builder.close();
        }
    })
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
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let path = to_iced_path(&self.polygon, bounds.size());

            frame.fill(&path, self.style.fill);

            if self.style.stroke.width > 0.0 {
                frame.stroke(&path, self.style.stroke);
            }
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
