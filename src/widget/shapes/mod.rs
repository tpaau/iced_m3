use std::time::Instant;

use iced::{Element, Point, Size};
use iced_widget::{
    Canvas,
    canvas::{self, Cache, Fill, LineCap, LineDash, LineJoin, Path, Stroke, fill::Rule},
};

pub use polymorpher;
use polymorpher::{Feature, Morph, RoundedPolygon};

use crate::{
    animation::motion::{SimulatedSpring, Spring},
    theme::ColorScheme,
};

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

#[derive(Debug, Clone, PartialEq)]
struct MorphTransition {
    spring: SimulatedSpring,
    morph: Morph,
}

impl MorphTransition {
    pub fn new(spring: Spring, morph: Morph, now: Instant) -> Self {
        Self {
            spring: SimulatedSpring::new(spring, now),
            morph,
        }
    }

    pub fn value(&self, size: Size) -> Path {
        let cubics = self.morph.as_cubics(self.spring.position);
        let side = size.width.min(size.height);
        let offset_x = (size.width - side) * 0.5;
        let offset_y = (size.height - side) * 0.5;

        let map = |p: polymorpher::geometry::Point| {
            Point::new(offset_x + p.x * side, offset_y + p.y * side)
        };

        Path::new(|builder| {
            if let Some(first) = cubics.first() {
                builder.move_to(map(first.anchor0()));

                for cubic in &cubics {
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

    pub fn set_target(
        &mut self,
        last_target: Option<RoundedPolygon>,
        target: RoundedPolygon,
        now: Instant,
    ) {
        let current = if !self.spring.is_at_rest()
            && let Some(last_target) = last_target
        {
            Some(last_target)
        } else {
            None
        };

        let current = current.unwrap_or({
            let cubics = self.morph.as_cubics(self.spring.position);
            RoundedPolygon::new(
                vec![Feature::edge(cubics)],
                canvas::path::lyon_path::geom::euclid::Point2D::zero(),
            )
        });

        self.morph = Morph::new(current, target);
        self.spring.reset(now);
    }

    pub fn step(&mut self, now: Instant) {
        self.spring.step(now);
    }
}

pub struct MaterialShape<'a> {
    polygon: RoundedPolygon,
    style: Style<'a>,
    size: f32,
    cache: Cache,
    spring: Option<Spring>,
}

impl<'a> MaterialShape<'a> {
    #[must_use]
    pub fn new(polygon: RoundedPolygon, style: Style<'a>) -> Self {
        Self {
            polygon,
            style,
            size: 100.0,
            cache: Cache::default(),
            spring: None,
        }
    }

    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn transition_spring(mut self, spring: Spring) -> Self {
        self.spring = Some(spring);
        self
    }

    #[must_use]
    pub fn transition_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.spring = spring;
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

#[derive(Default)]
pub struct State {
    transition: Option<MorphTransition>,
    prev_polygon: Option<RoundedPolygon>,
}

impl<'a, Message, Theme> canvas::Program<Message, Theme, iced::Renderer> for MaterialShape<'a> {
    type State = State;

    fn update(
        &self,
        state: &mut Self::State,
        _event: &iced::Event,
        _bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        match self.spring {
            Some(spring) => {
                if state.transition.is_none() {
                    state.transition = Some(MorphTransition::new(
                        spring,
                        Morph::new(self.polygon.clone(), self.polygon.clone()),
                        Instant::now(),
                    ));
                }
            }
            None => {
                if state.transition.is_some() {
                    state.transition = None
                }
            }
        }

        if let Some(transition) = &mut state.transition {
            if let Some(spring) = self.spring {
                if transition.spring.model != spring {
                    transition.spring.model = spring;
                }
            }

            let now = Instant::now();
            if Some(&self.polygon) != state.prev_polygon.as_ref() {
                transition.set_target(state.prev_polygon.clone(), self.polygon.clone(), now);
                state.prev_polygon = Some(self.polygon.clone());
            }

            let is_at_rest = transition.spring.is_at_rest();
            transition.step(now);
            self.cache.clear();

            if !is_at_rest {
                return Some(canvas::Action::request_redraw());
            }
        }

        None
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let path = state
                .transition
                .as_ref()
                .map(|t| t.value(bounds.size()))
                .unwrap_or(to_iced_path(&self.polygon, bounds.size()));

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
