use iced::{
    Alignment, Element, Font, Length, Task,
    widget::{column, container, row, text},
};
use iced_m3::{
    theme::{Accent, ColorScheme, Mode, Theme},
    widget::{
        self,
        button::{self, Content},
        switch::IconMode,
    },
};

const APP_NAME: &str = "Switch Demo";

#[derive(Clone)]
enum Message {
    Toggle1,
    Toggle2,
    SetIconMode(IconMode),
}

struct State {
    theme: Theme,
    toggled1: bool,
    toggled2: bool,
    icon_mode: IconMode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            toggled1: false,
            toggled2: false,
            icon_mode: IconMode::default(),
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
                row![
                    text("With expressive animation"),
                    widget::switch(&self.theme, self.toggled1)
                        .expressive_animation(true)
                        .icon_mode(self.icon_mode)
                        .on_toggle(Message::Toggle1)
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                row![
                    text("With standard animation"),
                    widget::switch(&self.theme, self.toggled2)
                        .expressive_animation(false)
                        .icon_mode(self.icon_mode)
                        .on_toggle(Message::Toggle2)
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                row![
                    text("Disabled selected"),
                    widget::switch(&self.theme, true).icon_mode(self.icon_mode),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                row![
                    text("Disabled unselected"),
                    widget::switch(&self.theme, false).icon_mode(self.icon_mode),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
                column![
                    text("Icon mode"),
                    row![
                        widget::button(
                            button::Style::filled(&self.theme, Accent::default()),
                            Content::Label("Never".into()),
                        )
                        .on_press(Message::SetIconMode(IconMode::Never)),
                        widget::button(
                            button::Style::filled(&self.theme, Accent::default()),
                            Content::Label("When Selected".into()),
                        )
                        .on_press(Message::SetIconMode(IconMode::WhenSelected)),
                        widget::button(
                            button::Style::filled(&self.theme, Accent::default()),
                            Content::Label("Always".into()),
                        )
                        .on_press(Message::SetIconMode(IconMode::Always))
                    ]
                    .spacing(8)
                ]
                .spacing(8)
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
            Message::Toggle1 => self.toggled1 = !self.toggled1,
            Message::Toggle2 => self.toggled2 = !self.toggled2,
            Message::SetIconMode(icon_mode) => self.icon_mode = icon_mode,
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
