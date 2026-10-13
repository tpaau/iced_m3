//! The M3 shape system includes original shapes, a corner radius scale, and built-in shape morphing

use std::time::Instant;

use iced::{Element, Point, Size};
use iced_widget::{
    Canvas,
    canvas::{self, Cache, Fill, LineCap, LineDash, LineJoin, Path, Stroke, fill::Rule},
};

pub use polymorpher;
use polymorpher::{Morph, RoundedPolygon};

use crate::{
    animation::motion::{SimulatedSpring, Spring},
    theme::ColorScheme,
};

pub(crate) fn cubics_to_iced_path(cubics: &[polymorpher::Cubic], size: Size) -> Path {
    let side = size.width.min(size.height);
    let offset_x = (size.width - side) * 0.5;
    let offset_y = (size.height - side) * 0.5;

    let map =
        |p: polymorpher::geometry::Point| Point::new(offset_x + p.x * side, offset_y + p.y * side);

    Path::new(|builder| {
        if let Some(first) = cubics.first() {
            builder.move_to(map(first.anchor0()));

            for cubic in cubics {
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

pub(crate) fn to_iced_path(polygon: &RoundedPolygon, size: Size) -> Path {
    let polygon = polygon.clone().normalized();
    cubics_to_iced_path(&polygon.cubics, size)
}

/// The style of the shape.
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
    fn new(spring: Spring, morph: Morph, now: Instant) -> Self {
        Self {
            spring: SimulatedSpring::new(spring, now),
            morph,
        }
    }

    fn value(&self, size: Size) -> Path {
        let cubics = self.morph.as_cubics(self.spring.position);
        cubics_to_iced_path(&cubics, size)
    }

    fn set_target(&mut self, last_target: RoundedPolygon, target: RoundedPolygon, now: Instant) {
        self.morph = Morph::new(last_target, target);
        self.spring.reset(now);
    }

    pub fn step(&mut self, now: Instant) {
        self.spring.step(now);
    }
}

/// Adds a margin to the shape canvas so that it doesn't get clipped when the morph transition spring overshoots.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum OvershootMargin {
    /// Don't add any margin to the shape (this is the default).
    #[default]
    None,
    /// Set a manual margin value.
    Fixed(f32),
    /// Automatically detect the margin.
    Auto,
}

impl OvershootMargin {
    fn value(&self, spring: &Spring, size: f32) -> f32 {
        match self {
            OvershootMargin::None => 0.0,
            OvershootMargin::Fixed(value) => *value,
            // TODO: Maybe there's a better way of doing this?
            // `Spring::overshoot_distance(...)` returns the maximum distance a vertex could
            // overshoot with if it was displaced from one edge of the canvas to the other. However,
            // this should never happen with correctly labeled shapes, so we divide the value by 4.
            OvershootMargin::Auto => spring.overshoot_distance(size) / 4.0,
        }
    }
}

/// Abstract Material shape widget.
pub struct MaterialShape<'a, Renderer>
where
    Renderer: iced_widget::renderer::geometry::Renderer,
{
    polygon: RoundedPolygon,
    style: Style<'a>,
    size: f32,
    cache: Cache<Renderer>,
    spring: Option<Spring>,
    overshoot_margin: OvershootMargin,
}

impl<'a, Renderer> MaterialShape<'a, Renderer>
where
    Renderer: iced_widget::renderer::geometry::Renderer,
{
    #[must_use]
    pub fn new(polygon: RoundedPolygon, style: Style<'a>) -> Self {
        Self {
            polygon,
            style,
            size: 100.0,
            cache: Cache::default(),
            spring: None,
            overshoot_margin: OvershootMargin::default(),
        }
    }

    /// Sets the size of the shape.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Sets the transition spring used for shape morphing.
    #[must_use]
    pub fn transition_spring(mut self, spring: Spring) -> Self {
        self.spring = Some(spring);
        self
    }

    /// Sets the transition spring used for shape morphing, if `Some`.
    ///
    /// If `None`, the shape won't morph.
    #[must_use]
    pub fn transition_spring_maybe(mut self, spring: Option<Spring>) -> Self {
        self.spring = spring;
        self
    }

    /// Sets the [overshoot margin](OvershootMargin) of the shape.
    #[must_use]
    pub fn overshoot_margin(mut self, margin: OvershootMargin) -> Self {
        self.overshoot_margin = margin;
        self
    }
}

#[derive(Default)]
pub struct State {
    transition: Option<MorphTransition>,
    prev_polygon: Option<RoundedPolygon>,
}

impl<'a, Message, Theme, Renderer> canvas::Program<Message, Theme, Renderer>
    for MaterialShape<'a, Renderer>
where
    Renderer: iced_widget::renderer::geometry::Renderer,
{
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
                transition.set_target(
                    state.prev_polygon.clone().unwrap_or(self.polygon.clone()),
                    self.polygon.clone(),
                    now,
                );
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
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let padding = self
            .spring
            .as_ref()
            .map(|s| self.overshoot_margin.value(s, self.size))
            .unwrap_or(0.0);
        let shape_size = iced::Size::new(
            (bounds.width - 2.0 * padding).max(0.0),
            (bounds.height - 2.0 * padding).max(0.0),
        );

        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let path = state
                .transition
                .as_ref()
                .map(|t| t.value(shape_size))
                .unwrap_or(to_iced_path(&self.polygon, shape_size));

            frame.translate(iced::Vector::new(padding, padding));
            frame.fill(&path, self.style.fill);

            if self.style.stroke.width > 0.0 {
                frame.stroke(&path, self.style.stroke);
            }
        });

        vec![geometry]
    }
}

impl<'a, Message, Theme, Renderer> From<MaterialShape<'a, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + iced_widget::renderer::geometry::Renderer,
{
    fn from(value: MaterialShape<'a, Renderer>) -> Self {
        let size = value.size;
        Element::new(Canvas::new(value).width(size).height(size))
    }
}
