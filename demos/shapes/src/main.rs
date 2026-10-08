use iced::{
    Element, Font, Length, Task,
    widget::{column, container, row, text::IntoFragment},
};
use iced_m3::{
    spacing::SPACE_150,
    theme::{Accent, ColorScheme, Mode, Theme},
    widget::{
        button, shape,
        shapes::{self, Shape},
    },
};

const APP_NAME: &str = "Shapes Demo";

#[derive(Clone)]
enum Message {
    SetShape(Shape),
}

struct State {
    theme: Theme,
    shape: Shape,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            shape: Shape::Circle,
        }
    }
}

fn shape_to_str(shape: &Shape) -> &str {
    match shape {
        Shape::Circle => "Circle",
        Shape::Square => "Square",
        Shape::Slanted => "Slanted",
        Shape::Arch => "Arch",
        Shape::Fan => "Fan",
        Shape::Arrow => "Arrow",
        Shape::SemiCircle => "SemiCircle",
        Shape::Oval => "Oval",
        Shape::Pill => "Pill",
        Shape::Triangle => "Triangle",
        Shape::Diamond => "Diamond",
        Shape::ClamShell => "ClamShell",
        Shape::Pentagon => "Pentagon",
        Shape::Gem => "Gem",
        Shape::Sunny => "Sunny",
        Shape::VerySunny => "VerySunny",
        Shape::Cookie4Sided => "Cookie4Sided",
        Shape::Cookie6Sided => "Cookie6Sided",
        Shape::Cookie7Sided => "Cookie7Sided",
        Shape::Cookie9Sided => "Cookie9Sided",
        Shape::Cookie12Sided => "Cookie12Sided",
        Shape::Ghostish => "Ghostish",
        Shape::Clover4Leaf => "Clover4Leaf",
        Shape::Clover8Leaf => "Clover8Leaf",
        Shape::Burst => "Burst",
        Shape::SoftBurst => "SoftBurst",
        Shape::Boom => "Boom",
        Shape::SoftBoom => "SoftBoom",
        Shape::Flower => "Flower",
        Shape::Puffy => "Puffy",
        Shape::PuffyDiamond => "PuffyDiamond",
        Shape::PixelCircle => "PixelCircle",
        Shape::PixelTriangle => "PixelTriangle",
        Shape::Bun => "Bun",
        Shape::Heart => "Heart",
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
            .on_press(Message::SetShape(*shape))
            .into()
        });

        container(
            column![
                iced::widget::text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                shape(self.shape, shapes::Style::new(&self.theme)).size(200.0),
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
            Message::SetShape(shape) => self.shape = shape,
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
