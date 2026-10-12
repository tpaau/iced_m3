use iced::{
    Element, Font, Length, Task,
    widget::{column, container, row, text::IntoFragment},
};
use iced_m3::{
    animation::motion::{self, fast_spatial},
    spacing::SPACE_150,
    theme::{Accent, ColorScheme, Mode, Theme},
    widget::{
        button, shape,
        shapes::{
            OvershootMargin,
            polymorpher::{
                RoundedPolygon,
                shapes::{self, circle},
            },
        },
    },
};

const APP_NAME: &str = "Shapes Demo";

#[derive(Clone)]
enum Message {
    SetPolygon(RoundedPolygon),
}

struct State {
    theme: Theme,
    polygon: RoundedPolygon,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            polygon: circle(None),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Circle => "Circle",
            Self::Square => "Square",
            Self::Slanted => "Slanted",
            Self::Arch => "Arch",
            Self::Fan => "Fan",
            Self::Arrow => "Arrow",
            Self::SemiCircle => "SemiCircle",
            Self::Oval => "Oval",
            Self::Pill => "Pill",
            Self::Triangle => "Triangle",
            Self::Diamond => "Diamond",
            Self::ClamShell => "ClamShell",
            Self::Pentagon => "Pentagon",
            Self::Gem => "Gem",
            Self::Sunny => "Sunny",
            Self::VerySunny => "VerySunny",
            Self::Cookie4Sided => "Cookie4Sided",
            Self::Cookie6Sided => "Cookie6Sided",
            Self::Cookie7Sided => "Cookie7Sided",
            Self::Cookie9Sided => "Cookie9Sided",
            Self::Cookie12Sided => "Cookie12Sided",
            Self::Ghostish => "Ghostish",
            Self::Clover4Leaf => "Clover4Leaf",
            Self::Clover8Leaf => "Clover8Leaf",
            Self::Burst => "Burst",
            Self::SoftBurst => "SoftBurst",
            Self::Boom => "Boom",
            Self::SoftBoom => "SoftBoom",
            Self::Flower => "Flower",
            Self::Puffy => "Puffy",
            Self::PuffyDiamond => "PuffyDiamond",
            Self::PixelCircle => "PixelCircle",
            Self::PixelTriangle => "PixelTriangle",
            Self::Bun => "Bun",
            Self::Heart => "Heart",
        }
    }

    pub fn to_rounded_polygon(self) -> RoundedPolygon {
        match self {
            Self::Circle => shapes::circle(None),
            Self::Square => shapes::square(),
            Self::Slanted => shapes::slanted(),
            Self::Arch => shapes::arch(),
            Self::Fan => shapes::fan(),
            Self::Arrow => shapes::arrow(),
            Self::SemiCircle => shapes::semi_circle(),
            Self::Oval => shapes::oval(),
            Self::Pill => shapes::pill(),
            Self::Triangle => shapes::triangle(),
            Self::Diamond => shapes::diamond(),
            Self::ClamShell => shapes::clam_shell(),
            Self::Pentagon => shapes::pentagon(),
            Self::Gem => shapes::gem(),
            Self::Sunny => shapes::sunny(),
            Self::VerySunny => shapes::very_sunny(),
            Self::Cookie4Sided => shapes::cookie4(),
            Self::Cookie6Sided => shapes::cookie6(),
            Self::Cookie7Sided => shapes::cookie7(),
            Self::Cookie9Sided => shapes::cookie9(),
            Self::Cookie12Sided => shapes::cookie12(),
            Self::Ghostish => shapes::ghostish(),
            Self::Clover4Leaf => shapes::clover4(),
            Self::Clover8Leaf => shapes::clover8(),
            Self::Burst => shapes::burst(),
            Self::SoftBurst => shapes::soft_burst(),
            Self::Boom => shapes::boom(),
            Self::SoftBoom => shapes::soft_boom(),
            Self::Flower => shapes::flower(),
            Self::Puffy => shapes::puffy(),
            Self::PuffyDiamond => shapes::puffy_diamond(),
            Self::PixelCircle => shapes::pixel_circle(),
            Self::PixelTriangle => shapes::pixel_triangle(),
            Self::Bun => shapes::bun(),
            Self::Heart => shapes::heart(),
        }
    }
}

pub fn shape_to_str(shape: &Shape) -> &'static str {
    shape.as_str()
}

impl From<Shape> for RoundedPolygon {
    fn from(shape: Shape) -> Self {
        shape.to_rounded_polygon()
    }
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        let options = &[
            Shape::Circle,
            Shape::Square,
            Shape::Slanted,
            Shape::Arch,
            Shape::Fan,
            Shape::Arrow,
            Shape::SemiCircle,
            Shape::Oval,
            Shape::Pill,
            Shape::Triangle,
            Shape::Diamond,
            Shape::ClamShell,
            Shape::Pentagon,
            Shape::Gem,
            Shape::Sunny,
            Shape::VerySunny,
            Shape::Cookie4Sided,
            Shape::Cookie6Sided,
            Shape::Cookie7Sided,
            Shape::Cookie9Sided,
            Shape::Cookie12Sided,
            Shape::Ghostish,
            Shape::Clover4Leaf,
            Shape::Clover8Leaf,
            Shape::Burst,
            Shape::SoftBurst,
            Shape::Boom,
            Shape::SoftBoom,
            Shape::Flower,
            Shape::Puffy,
            Shape::PuffyDiamond,
            Shape::PixelCircle,
            Shape::PixelTriangle,
            Shape::Bun,
            Shape::Heart,
        ];

        let buttons = options.into_iter().map(|shape| {
            button(
                button::Style::filled(&self.theme, Accent::Primary),
                button::Content::Label(shape_to_str(shape).into_fragment()),
            )
            .on_press(Message::SetPolygon((*shape).into()))
            .into()
        });

        container(
            column![
                iced::widget::text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                shape(
                    self.polygon.clone(),
                    iced_m3::widget::shapes::Style::new(&self.theme)
                )
                .overshoot_margin(OvershootMargin::Auto)
                .transition_spring(fast_spatial(motion::Scheme::Expressive))
                .size(200.0),
                row(buttons).spacing(SPACE_150).wrap()
            ]
            .spacing(20),
        )
        .padding(10.0)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| iced::widget::container::Style::default().background(self.theme.background()))
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetPolygon(polygon) => self.polygon = polygon,
        }
        Task::none()
    }
}

fn main() -> iced::Result {
    iced::application(
        || {
            (
                State::default(),
                Task::batch(
                    fonts::get_fonts()
                        .into_iter()
                        .map(|b| iced::font::load(b).discard()),
                ),
            )
        },
        State::update,
        State::view,
    )
    .title(APP_NAME)
    .default_font(Font::with_name("Noto Sans"))
    .run()
}
