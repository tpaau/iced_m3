use std::sync::LazyLock;

use iced::{
    Element, Font, Length, Task,
    widget::{column, container, row, text::IntoFragment},
};
use iced_m3::{
    theme::{Accent, ColorScheme, Mode, Theme},
    widget::{
        button::{self, Content},
        button_group, connected_button_group,
        hybrid_icon::Icon,
    },
};

const APP_NAME: &str = "Button Groups Demo";
static STARS: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8d0).unwrap());

#[derive(Clone)]
enum Message {
    SelectIndex(usize),
    ToggleButon(usize),
}

struct State {
    theme: Theme,
    selected_index: usize,
    buttons: [bool; 3],
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            selected_index: 0,
            buttons: [false, false, false],
        }
    }
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        let single_select = [
            button_group::connected::Size::extra_small(),
            button_group::connected::Size::small(),
            button_group::connected::Size::medium(),
            button_group::connected::Size::large(),
            button_group::connected::Size::extra_large(),
        ]
        .into_iter()
        .map(|size| {
            connected_button_group(
                button::Style::filled(&self.theme, Accent::Primary),
                (0..3)
                    .into_iter()
                    .map(|i| button_group::connected::Button {
                        content: Content::Full {
                            icon: Icon::Text {
                                text: STARS.into_fragment(),
                                font: Some(match self.selected_index == i {
                                    true => fonts::icons_filled(),
                                    false => fonts::icons_outlined(),
                                }),
                            },
                            label: "Label".into(),
                        },
                        on_press: Some(Message::SelectIndex(i)),
                        selected: self.selected_index == i,
                    })
                    .collect(),
            )
            .size(size)
            .into()
        });

        let multi_select = [
            button_group::connected::Size::extra_small(),
            button_group::connected::Size::small(),
            button_group::connected::Size::medium(),
            button_group::connected::Size::large(),
            button_group::connected::Size::extra_large(),
        ]
        .into_iter()
        .map(|size| {
            connected_button_group(
                button::Style::filled(&self.theme, Accent::Primary),
                self.buttons
                    .into_iter()
                    .enumerate()
                    .map(|(index, selected)| button_group::connected::Button {
                        content: Content::Full {
                            icon: Icon::Text {
                                text: STARS.into_fragment(),
                                font: Some(match selected {
                                    true => fonts::icons_filled(),
                                    false => fonts::icons_outlined(),
                                }),
                            },
                            label: "Label".into(),
                        },
                        on_press: Some(Message::ToggleButon(index)),
                        selected,
                    })
                    .collect(),
            )
            .size(size)
            .into()
        });

        let connected_button_groups = row![
            column![
                iced::widget::text("Single select"),
                column(single_select).spacing(10)
            ]
            .spacing(10),
            column![
                iced::widget::text("Multi select"),
                column(multi_select).spacing(10)
            ]
            .spacing(10)
        ]
        .spacing(20);

        container(
            column![
                iced::widget::text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                connected_button_groups
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
            Message::SelectIndex(index) => self.selected_index = index,
            Message::ToggleButon(index) => self.buttons[index] = !self.buttons[index],
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
