use std::sync::LazyLock;

use iced::{
    Element, Font, Length, Task,
    widget::{bottom_right, column, container, text::IntoFragment},
};
use iced_m3::{
    theme::{Accent, ColorScheme, Theme},
    widget::{OnPress, fab_menu, hybrid_icon::Icon},
};

const APP_NAME: &str = "FAB Menu demo";
static DOCUMENT: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe158).unwrap());
static CHAT: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe0b7).unwrap());
static FOLDER_SHARED: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe2c9).unwrap());

#[derive(Clone)]
enum Message {
    Noop,
}

struct State {
    theme: Theme,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(iced_m3::theme::Mode::Dark),
        }
    }
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let menu = fab_menu(
            vec![
                fab_menu::Item {
                    on_press: OnPress::Direct(Message::Noop),
                    label: "Document".into_fragment(),
                    icon: Icon::Text {
                        text: DOCUMENT.into_fragment(),
                        font: Some(fonts::icons_filled()),
                    },
                },
                fab_menu::Item {
                    on_press: OnPress::Direct(Message::Noop),
                    label: "Message".into_fragment(),
                    icon: Icon::Text {
                        text: CHAT.into_fragment(),
                        font: Some(fonts::icons_filled()),
                    },
                },
                fab_menu::Item {
                    on_press: OnPress::Direct(Message::Noop),
                    label: "Folder".into_fragment(),
                    icon: Icon::Text {
                        text: FOLDER_SHARED.into_fragment(),
                        font: Some(fonts::icons_filled()),
                    },
                },
            ],
            iced_m3::widget::fab::Size::Regular,
            None,
            &self.theme,
            Accent::default(),
        );

        let content = column![
            iced::widget::text(APP_NAME)
                .font(fonts::text_bold())
                .color(self.theme.on_surface())
                .size(20.0),
            bottom_right(menu)
        ];

        container(content)
            .style(|_| {
                iced::widget::container::Style::default().background(self.theme.background())
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10.0)
            .into()
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
