use iced::{
    Color, Element, Font, Length, Task,
    widget::{center, column, container, row, text},
};
use iced_m3::{
    style::StateLayer,
    theme::{ColorScheme, Mode, Theme},
    widget::{
        OnPress,
        button::Content,
        card::{Interaction, MAX_CARD_BETWEEN_PADDING, Style},
    },
};

const APP_NAME: &str = "Card Demo";

#[derive(Clone)]
enum Message {
    Noop,
    SetDisabled,
    SetNonInteractive,
    SetPressable,
}

struct State<'a> {
    theme: Theme,
    interaction: Interaction<'a, Message>,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            interaction: Interaction::default(),
        }
    }
}

impl<'a> State<'a> {
    fn view(&self) -> Element<'_, Message> {
        let card_width = 250.0;
        let card_height = 150.0;

        let card =
            move |style: Style, label: &'a str, label_color: Color| -> Element<'a, Message> {
                let interaction = match &self.interaction {
                    Interaction::NonInteractive => Interaction::NonInteractive,
                    Interaction::Press(on_press) => {
                        Interaction::Press(OnPress::Direct(on_press.resolve()))
                    }
                    Interaction::Disabled => Interaction::Disabled,
                };
                iced_m3::widget::card(style, center(text(label).size(24.0).color(label_color)))
                    .interaction(interaction)
                    .width(card_width)
                    .height(card_height)
                    .into()
            };

        let buttons = row![
            iced_m3::widget::button(&self.theme, Content::Label("Non-interactive".into()))
                .on_press(Message::SetNonInteractive),
            iced_m3::widget::button(&self.theme, Content::Label("Disabled".into()))
                .on_press(Message::SetDisabled),
            iced_m3::widget::button(&self.theme, Content::Label("Clickable".into()))
                .on_press(Message::SetPressable),
        ]
        .spacing(4.0);

        let cards = column![
            row![
                card(
                    Style::elevated(&self.theme),
                    "Elevated",
                    self.theme.on_surface()
                ),
                card(
                    Style::filled(&self.theme),
                    "Filled",
                    self.theme.on_surface()
                )
            ]
            .spacing(MAX_CARD_BETWEEN_PADDING),
            row![
                card(
                    Style::outlined(&self.theme),
                    "Outlined",
                    self.theme.on_surface()
                ),
                card(
                    Style {
                        container_color: self.theme.tertiary_container(),
                        state_layer: StateLayer::new(self.theme.on_tertiary_container()),
                        ..Style::filled(&self.theme)
                    },
                    "Custom",
                    self.theme.on_tertiary_container()
                )
            ]
            .spacing(MAX_CARD_BETWEEN_PADDING)
        ]
        .spacing(MAX_CARD_BETWEEN_PADDING);

        container(
            column![
                text(APP_NAME)
                    .font(fonts::text_bold())
                    .size(24.0)
                    .color(self.theme.on_surface()),
                buttons,
                cards,
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
            Message::Noop => {}
            Message::SetDisabled => self.interaction = Interaction::Disabled,
            Message::SetNonInteractive => self.interaction = Interaction::NonInteractive,
            Message::SetPressable => {
                self.interaction = Interaction::Press(OnPress::Direct(Message::Noop))
            }
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
