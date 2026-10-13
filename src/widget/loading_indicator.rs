//! Loading indicators show the progress of a process for a short wait time

use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

use iced::{Color, Element, Radians, Size, Vector};
use iced_widget::{
    Canvas,
    canvas::{self, Fill},
};
use polymorpher::{
    Morph,
    shapes::{cookie4, cookie9, oval, pentagon, pill, soft_burst, sunny},
};

use crate::{
    animation::motion::SimulatedSpring, theme::ColorScheme, widget::shapes::cubics_to_iced_path,
};

mod constants {
    use std::sync::LazyLock;

    use polymorpher::shapes::{cookie4, cookie9, oval, pentagon, pill, soft_burst, sunny};

    pub const CONTAINER_SIZE: f32 = 48.0;
    pub const INDICATOR_SIZE: f32 = 38.0;
    pub const LINEAR_ROTATION_CYCLE_SECONDS: f32 = 4.666;
    pub const MORPH_CYCLE_SECONDS: f32 = 0.650;
    pub const INDICATOR_SCALING_FACTOR: LazyLock<f32> = LazyLock::new(|| {
        let polygons = [
            soft_burst(),
            cookie9(),
            pentagon(),
            pill(),
            sunny(),
            cookie4(),
            oval(),
        ];
        let mut factor = f32::MAX;
        for polygon in polygons {
            let bounds = polygon.aabb(false).max;
            factor = factor.min(bounds.x.min(bounds.y));
        }
        factor
    });
}

#[cfg(feature = "pub-internal-const")]
pub use constants::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Style {
    Contained {
        indicator_color: Color,
        container_color: Color,
    },
    Uncontained {
        indicator_color: Color,
    },
}

impl Style {
    pub fn contained(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self::Contained {
            indicator_color: theme.on_primary_container(),
            container_color: theme.primary_container(),
        }
    }

    pub fn uncontained(theme: &(impl ColorScheme + ?Sized)) -> Self {
        Self::Uncontained {
            indicator_color: theme.primary(),
        }
    }
}

pub struct LoadingIndicator {
    style: Style,
}

impl LoadingIndicator {
    #[must_use]
    pub fn new(style: Style) -> Self {
        Self { style }
    }
}

pub struct State {
    start: Instant,
    last_cycle: Duration,
    spring: SimulatedSpring,
    morphs: [Morph; 7],
}

impl Default for State {
    fn default() -> Self {
        let now = Instant::now();

        Self {
            start: Instant::now(),
            spring: SimulatedSpring::new(
                crate::animation::motion::constants::FAST_SPATIAL_EXPRESSIVE,
                now,
            ),
            last_cycle: Duration::default(),
            morphs: [
                Morph::new(soft_burst(), cookie9()),
                Morph::new(cookie9(), pentagon()),
                Morph::new(pentagon(), pill()),
                Morph::new(pill(), sunny()),
                Morph::new(sunny(), cookie4()),
                Morph::new(cookie4(), oval()),
                Morph::new(oval(), soft_burst()),
            ],
        }
    }
}

impl<'a, Message, Theme, Renderer> canvas::Program<Message, Theme, Renderer> for LoadingIndicator
where
    Renderer: iced_widget::renderer::geometry::Renderer,
{
    type State = State;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let (indicator_color, container_color) = match self.style {
            Style::Contained {
                indicator_color,
                container_color,
            } => (indicator_color, Some(container_color)),
            Style::Uncontained { indicator_color } => (indicator_color, None),
        };

        if let Some(color) = container_color {
            let radius = constants::CONTAINER_SIZE / 2.0;
            let circle = canvas::Path::circle(frame.center(), radius);
            frame.fill(&circle, color);
        }

        let size = constants::INDICATOR_SIZE;
        let offset = Vector::new((frame.width() - size) / 2.0, (frame.height() - size) / 2.0);
        let half = size / 2.0;

        let current_morph = &state.morphs[(state.last_cycle.as_secs_f32()
            / constants::MORPH_CYCLE_SECONDS) as usize
            % state.morphs.len()];

        let path = cubics_to_iced_path(
            &current_morph.as_cubics(state.spring.position),
            Size::new(constants::INDICATOR_SIZE, constants::INDICATOR_SIZE),
        );

        let elapsed = Instant::now().duration_since(state.start).as_secs_f32();
        let angle = Radians(
            elapsed * std::f32::consts::TAU / constants::LINEAR_ROTATION_CYCLE_SECONDS
                + PI / 4.0 * state.spring.position
                + PI / 4.0 * state.last_cycle.as_secs_f32() / constants::MORPH_CYCLE_SECONDS,
        );

        frame.translate(Vector::new(offset.x + half, offset.y + half));
        frame.rotate(angle);
        frame.scale(*constants::INDICATOR_SCALING_FACTOR);
        frame.translate(Vector::new(-half, -half));

        frame.fill(
            &path,
            Fill {
                style: canvas::Style::Solid(indicator_color),
                rule: canvas::fill::Rule::NonZero,
            },
        );

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        _event: &iced::Event,
        _bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let now = Instant::now();
        state.spring.step(now);

        let elapsed = Instant::now().duration_since(state.start);
        if elapsed.as_secs_f32() - state.last_cycle.as_secs_f32() > constants::MORPH_CYCLE_SECONDS {
            state.last_cycle += Duration::from_secs_f32(constants::MORPH_CYCLE_SECONDS);
            state.spring.reset(now);
        }

        Some(canvas::Action::request_redraw())
    }
}

impl<'a, Message, Theme, Renderer> From<LoadingIndicator> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + iced_widget::renderer::geometry::Renderer,
{
    fn from(value: LoadingIndicator) -> Self {
        Element::new(
            Canvas::new(value)
                .width(constants::CONTAINER_SIZE)
                .height(constants::CONTAINER_SIZE),
        )
    }
}
