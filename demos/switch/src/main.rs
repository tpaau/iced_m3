use iced::{
    Alignment, Element, Font, Length, Task,
    widget::{column, container, row, text},
};
use iced_m3::{
    theme::{ColorScheme, Mode, Theme},
    widget::switch,
};

const APP_NAME: &str = "Switch Demo";

#[derive(Clone)]
enum Message {
    Toggle,
    ToggleExpressive,
}

struct State {
    theme: Theme,
    toggled: bool,
    expressive_animation: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            toggled: false,
            expressive_animation: false,
        }
    }
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                switch(&self.theme, self.toggled)
                    .expressive_animation(self.expressive_animation)
                    .on_toggle(Message::Toggle),
                switch(&self.theme, false),
                switch(&self.theme, true),
                row![
                    text("Expressive animation"),
                    switch(&self.theme, self.expressive_animation)
                        .expressive_animation(self.expressive_animation)
                        .on_toggle(Message::ToggleExpressive)
                ]
                .align_y(Alignment::Center)
                .spacing(8.0),
            ]
            .spacing(20.0),
        )
        .padding(10.0)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| iced::widget::container::Style::default().background(self.theme.background()))
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toggle => self.toggled = !self.toggled,
            Message::ToggleExpressive => self.expressive_animation = !self.expressive_animation,
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
