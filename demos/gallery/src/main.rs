use std::sync::LazyLock;

use fonts::{icons_filled, icons_outlined, text_bold, text_regular};
use iced::{
    Alignment, Color, Element, Font, Length, Size, Task,
    widget::{
        bottom_right, center, column, container, mouse_area, opaque, row, scrollable, space, stack,
        text, text::IntoFragment,
    },
};
use iced_m3::{
    animation::motion::{self, fast_spatial},
    style::{self, StateLayer, shadow},
    theme::{Accent, ColorScheme, Mode, Theme},
    widget::{
        Action, Badge, advanced, button,
        button_group::{
            self,
            standard::{self, GroupItem},
        },
        card::{self, MAX_CARD_BETWEEN_PADDING},
        connected_button_group, dialog, fab, fab_menu,
        hybrid_icon::Icon,
        menu::{self, Menu, button::MenuAction},
        navbar,
        navrail::{self, Fab, Item, Mode as RailMode},
        progress_bar,
        shapes::{
            self, OvershootMargin,
            polymorpher::{RoundedPolygon, shapes as polymorpher_shapes},
        },
        slider, standard_button_group, switch, text_input,
    },
};

const APP_NAME: &str = "Widget Gallery";

// Material Symbols Rounded code points. Reused from the other demos where
// possible so every glyph shown here is known to render.
static TOUCH_APP: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe913).unwrap());
static TUNE: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe429).unwrap());
static NOTIFICATIONS: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe7f4).unwrap());
static DASHBOARD: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe871).unwrap());
static HELP: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe887).unwrap());
static EDIT: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe3c9).unwrap());
static STARS: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8d0).unwrap());
static SEARCH: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8b6).unwrap());
static HOME: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe88a).unwrap());
static VISIBILITY: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8f4).unwrap());
static DOCUMENT: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe158).unwrap());
static CHAT: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe0b7).unwrap());
static FOLDER_SHARED: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe2c9).unwrap());
static DELETE: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe872).unwrap());
static SETTINGS: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe8b8).unwrap());
static PALETTE: LazyLock<char> = LazyLock::new(|| char::from_u32(0xe403).unwrap());

#[derive(Clone, Copy, Default)]
#[repr(usize)]
enum Tab {
    #[default]
    Buttons,
    Inputs,
    Feedback,
    Cards,
    Shapes,
    Dialogs,
    Navbar,
    Theme,
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tab::Buttons => write!(f, "Buttons"),
            Tab::Inputs => write!(f, "Inputs"),
            Tab::Feedback => write!(f, "Feedback"),
            Tab::Cards => write!(f, "Cards"),
            Tab::Shapes => write!(f, "Shapes"),
            Tab::Dialogs => write!(f, "Dialogs"),
            Tab::Navbar => write!(f, "Navbar"),
            Tab::Theme => write!(f, "Theme"),
        }
    }
}

impl Tab {
    fn index(self) -> usize {
        self as usize
    }

    fn icon(self) -> char {
        match self {
            Tab::Buttons => *TOUCH_APP,
            Tab::Inputs => *TUNE,
            Tab::Feedback => *NOTIFICATIONS,
            Tab::Cards => *DASHBOARD,
            Tab::Shapes => *STARS,
            Tab::Dialogs => *HELP,
            Tab::Navbar => *HOME,
            Tab::Theme => *PALETTE,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
enum CardMode {
    #[default]
    Press,
    Disabled,
    NonInteractive,
}

impl CardMode {
    fn label(self) -> &'static str {
        match self {
            CardMode::Press => "Pressable",
            CardMode::Disabled => "Disabled",
            CardMode::NonInteractive => "Non-interactive",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum ShapeKind {
    #[default]
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
    Cookie4,
    Cookie6,
    Cookie7,
    Cookie9,
    Cookie12,
    Ghostish,
    Clover4,
    Clover8,
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

impl ShapeKind {
    const ALL: [ShapeKind; 35] = [
        ShapeKind::Circle,
        ShapeKind::Square,
        ShapeKind::Slanted,
        ShapeKind::Arch,
        ShapeKind::Fan,
        ShapeKind::Arrow,
        ShapeKind::SemiCircle,
        ShapeKind::Oval,
        ShapeKind::Pill,
        ShapeKind::Triangle,
        ShapeKind::Diamond,
        ShapeKind::ClamShell,
        ShapeKind::Pentagon,
        ShapeKind::Gem,
        ShapeKind::Sunny,
        ShapeKind::VerySunny,
        ShapeKind::Cookie4,
        ShapeKind::Cookie6,
        ShapeKind::Cookie7,
        ShapeKind::Cookie9,
        ShapeKind::Cookie12,
        ShapeKind::Ghostish,
        ShapeKind::Clover4,
        ShapeKind::Clover8,
        ShapeKind::Burst,
        ShapeKind::SoftBurst,
        ShapeKind::Boom,
        ShapeKind::SoftBoom,
        ShapeKind::Flower,
        ShapeKind::Puffy,
        ShapeKind::PuffyDiamond,
        ShapeKind::PixelCircle,
        ShapeKind::PixelTriangle,
        ShapeKind::Bun,
        ShapeKind::Heart,
    ];

    fn label(self) -> &'static str {
        match self {
            ShapeKind::Circle => "Circle",
            ShapeKind::Square => "Square",
            ShapeKind::Slanted => "Slanted",
            ShapeKind::Arch => "Arch",
            ShapeKind::Fan => "Fan",
            ShapeKind::Arrow => "Arrow",
            ShapeKind::SemiCircle => "Semi circle",
            ShapeKind::Oval => "Oval",
            ShapeKind::Pill => "Pill",
            ShapeKind::Triangle => "Triangle",
            ShapeKind::Diamond => "Diamond",
            ShapeKind::ClamShell => "Clam shell",
            ShapeKind::Pentagon => "Pentagon",
            ShapeKind::Gem => "Gem",
            ShapeKind::Sunny => "Sunny",
            ShapeKind::VerySunny => "Very sunny",
            ShapeKind::Cookie4 => "Cookie 4",
            ShapeKind::Cookie6 => "Cookie 6",
            ShapeKind::Cookie7 => "Cookie 7",
            ShapeKind::Cookie9 => "Cookie 9",
            ShapeKind::Cookie12 => "Cookie 12",
            ShapeKind::Ghostish => "Ghostish",
            ShapeKind::Clover4 => "Clover 4",
            ShapeKind::Clover8 => "Clover 8",
            ShapeKind::Burst => "Burst",
            ShapeKind::SoftBurst => "Soft burst",
            ShapeKind::Boom => "Boom",
            ShapeKind::SoftBoom => "Soft boom",
            ShapeKind::Flower => "Flower",
            ShapeKind::Puffy => "Puffy",
            ShapeKind::PuffyDiamond => "Puffy diamond",
            ShapeKind::PixelCircle => "Pixel circle",
            ShapeKind::PixelTriangle => "Pixel triangle",
            ShapeKind::Bun => "Bun",
            ShapeKind::Heart => "Heart",
        }
    }

    fn polygon(self) -> RoundedPolygon {
        match self {
            ShapeKind::Circle => polymorpher_shapes::circle(None),
            ShapeKind::Square => polymorpher_shapes::square(),
            ShapeKind::Slanted => polymorpher_shapes::slanted(),
            ShapeKind::Arch => polymorpher_shapes::arch(),
            ShapeKind::Fan => polymorpher_shapes::fan(),
            ShapeKind::Arrow => polymorpher_shapes::arrow(),
            ShapeKind::SemiCircle => polymorpher_shapes::semi_circle(),
            ShapeKind::Oval => polymorpher_shapes::oval(),
            ShapeKind::Pill => polymorpher_shapes::pill(),
            ShapeKind::Triangle => polymorpher_shapes::triangle(),
            ShapeKind::Diamond => polymorpher_shapes::diamond(),
            ShapeKind::ClamShell => polymorpher_shapes::clam_shell(),
            ShapeKind::Pentagon => polymorpher_shapes::pentagon(),
            ShapeKind::Gem => polymorpher_shapes::gem(),
            ShapeKind::Sunny => polymorpher_shapes::sunny(),
            ShapeKind::VerySunny => polymorpher_shapes::very_sunny(),
            ShapeKind::Cookie4 => polymorpher_shapes::cookie4(),
            ShapeKind::Cookie6 => polymorpher_shapes::cookie6(),
            ShapeKind::Cookie7 => polymorpher_shapes::cookie7(),
            ShapeKind::Cookie9 => polymorpher_shapes::cookie9(),
            ShapeKind::Cookie12 => polymorpher_shapes::cookie12(),
            ShapeKind::Ghostish => polymorpher_shapes::ghostish(),
            ShapeKind::Clover4 => polymorpher_shapes::clover4(),
            ShapeKind::Clover8 => polymorpher_shapes::clover8(),
            ShapeKind::Burst => polymorpher_shapes::burst(),
            ShapeKind::SoftBurst => polymorpher_shapes::soft_burst(),
            ShapeKind::Boom => polymorpher_shapes::boom(),
            ShapeKind::SoftBoom => polymorpher_shapes::soft_boom(),
            ShapeKind::Flower => polymorpher_shapes::flower(),
            ShapeKind::Puffy => polymorpher_shapes::puffy(),
            ShapeKind::PuffyDiamond => polymorpher_shapes::puffy_diamond(),
            ShapeKind::PixelCircle => polymorpher_shapes::pixel_circle(),
            ShapeKind::PixelTriangle => polymorpher_shapes::pixel_triangle(),
            ShapeKind::Bun => polymorpher_shapes::bun(),
            ShapeKind::Heart => polymorpher_shapes::heart(),
        }
    }
}

#[derive(Clone)]
enum Message {
    Noop,
    GoTo(Tab),
    BarSelect(usize),
    ToggleExpanded,
    ToggleTheme,
    SearchChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    SliderChanged(f32),
    ToggleSwitch(usize),
    SelectGroup(usize),
    ToggleSelected(usize),
    SetCardMode(CardMode),
    SelectShape(ShapeKind),
    ToggleDropDown,
    ToggleDropDownSubMenu,
    OpenDialog,
    CloseDialog(bool),
}

struct State {
    theme: Theme,
    tab: Tab,
    bar_selected: usize,
    expanded: bool,
    search: String,
    email: String,
    password: String,
    slider_value: f32,
    switches: [bool; 3],
    group_selected: usize,
    selected_buttons: [bool; 4],
    card_mode: CardMode,
    selected_shape: ShapeKind,
    drop_down_opened: bool,
    drop_down_sub_menu_opened: bool,
    dialog_open: bool,
    dialog_result: Option<&'static str>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Theme::default(Mode::Dark),
            tab: Tab::default(),
            bar_selected: 0,
            expanded: false,
            search: String::new(),
            email: String::new(),
            password: String::new(),
            slider_value: 0.5,
            switches: [true, false, false],
            group_selected: 0,
            selected_buttons: [true, false, true, false],
            card_mode: CardMode::default(),
            selected_shape: ShapeKind::default(),
            drop_down_opened: false,
            drop_down_sub_menu_opened: false,
            dialog_open: false,
            dialog_result: None,
        }
    }
}

impl State {
    fn view(&self) -> Element<'_, Message> {
        let items = Tab::DEFAULT_TABS
            .iter()
            .copied()
            .map(|tab| {
                let badge = match tab {
                    Tab::Feedback => Some(Badge {
                        label: Some("3".into_fragment()),
                    }),
                    _ => None,
                };
                Item {
                    icon_active: Icon::Text {
                        text: tab.icon().into_fragment(),
                        font: Some(icons_filled()),
                    },
                    icon_inactive: Icon::Text {
                        text: tab.icon().into_fragment(),
                        font: Some(icons_outlined()),
                    },
                    badge,
                    label: tab.to_string().into_fragment(),
                    on_press: Message::GoTo(tab),
                }
            })
            .collect();

        let mode = match self.expanded {
            true => RailMode::Expanded {
                width: iced::Pixels(navrail::CONTAINER_EXPANDED_MIN_WIDTH),
            },
            false => RailMode::Collapsed,
        };

        let rail = iced_m3::widget::navrail(navrail::Style::new(&self.theme), items)
            .active(self.tab.index())
            .label_font(text_regular())
            .mode(mode)
            .fab_maybe(Some(Fab {
                icon: Icon::Text {
                    text: (*EDIT).into_fragment(),
                    font: Some(icons_filled()),
                },
                label: "New draft".into_fragment(),
                style: fab::Style::fab_tonal(&self.theme, Accent::Primary),
                on_press: Action::Direct(Message::OpenDialog),
            }))
            .on_menu_pressed_maybe(Some(|_| Message::ToggleExpanded));

        let header = text(APP_NAME)
            .font(text_bold())
            .size(24.0)
            .color(self.theme.on_surface());

        let screen = self.screen(self.tab);
        // The FAB menu floats above the Cards screen, independent from
        // scrolling.
        let fab_menu_layer: Element<'_, Message> = match self.tab {
            Tab::Cards => container(self.fab_menu_overlay()).padding(24.0).into(),
            _ => space().width(Length::Fill).height(Length::Fill).into(),
        };

        let content = container(
            column![
                header,
                // Vertical overflow management for the screens.
                stack![
                    scrollable(screen)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(|_theme, status| style::scrollable(status, &self.theme)),
                    fab_menu_layer,
                ]
                .width(Length::Fill)
                .height(Length::Fill),
            ]
            .spacing(16.0)
            .padding([12.0, 24.0])
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let base = container(
            row![
                container(rail).style(|_| {
                    container::Style::default().background(self.theme.surface_container())
                }),
                content
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style::default().background(self.theme.background()));

        let dialog_layer: Element<'_, Message> = match self.dialog_open {
            true => self.dialog(),
            false => space().width(Length::Fill).height(Length::Fill).into(),
        };

        stack![base, dialog_layer]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Demo-level dialog: it mirrors the library dialog's layout with the
    /// tokens from `dialog::Style`, but builds the buttons itself so shape
    /// morphing can be disabled here instead of the library.
    fn dialog(&self) -> Element<'_, Message> {
        let style = dialog::Style::new(&self.theme);

        let button_size = button::Size {
            corner_radius: button::CornerRadius::default().shape_morph(false),
            ..button::Size::default()
        };

        let buttons = row![
            space().width(Length::Fill),
            iced_m3::widget::button(
                button::Style::text(&self.theme, Accent::default()),
                button::Content::Label("Dismiss".into()),
            )
            .size(button_size)
            .on_press(Message::CloseDialog(false)),
            iced_m3::widget::button(
                button::Style::filled(&self.theme, Accent::default()),
                button::Content::Label("Accept".into()),
            )
            .size(button_size)
            .on_press(Message::CloseDialog(true)),
        ]
        .spacing(8.0);

        let dialog = container(
            column![
                space().width(280.0),
                iced_m3::widget::icon(*HELP, 24.0)
                    .font(icons_outlined())
                    .color(style.icon_color),
                space().height(16.0),
                text("Accept the draft?")
                    .size(24.0)
                    .color(style.title_color),
                space().height(16.0),
                text("The rail FAB and the button on the Dialogs screen both open this dialog.")
                    .size(16.0)
                    .color(self.theme.on_surface_variant()),
                space().height(24.0),
                buttons,
            ]
            .align_x(Alignment::Center),
        )
        .padding(24.0)
        .max_width(560.0)
        .style(move |_| {
            container::Style::default()
                .background(style.container_color)
                .border(style.container_border)
                .shadow(shadow(style.shadow_color, style.elevation))
        });

        mouse_area(
            container(opaque(dialog))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .padding(24.0)
                .style(|_| container::Style::default().background(Color::BLACK.scale_alpha(0.3))),
        )
        .on_press(Message::CloseDialog(false))
        .into()
    }

    fn screen(&self, tab: Tab) -> Element<'_, Message> {
        match tab {
            Tab::Buttons => self.buttons_screen(),
            Tab::Inputs => self.inputs_screen(),
            Tab::Feedback => self.feedback_screen(),
            Tab::Cards => self.cards_screen(),
            Tab::Shapes => self.shapes_screen(),
            Tab::Dialogs => self.dialogs_screen(),
            Tab::Navbar => self.navbar_screen(),
            Tab::Theme => self.theme_screen(),
        }
    }

    fn navbar_screen(&self) -> Element<'_, Message> {
        // Demo destinations distinct from the gallery's own tabs.
        let destinations: [(&'static str, char); 5] = [
            ("Home", *HOME),
            ("Files", *FOLDER_SHARED),
            ("Messages", *CHAT),
            ("Search", *SEARCH),
            ("Settings", *SETTINGS),
        ];

        let items: Vec<navbar::Item<'static, Message>> = destinations
            .iter()
            .enumerate()
            .map(|(i, (label, icon))| {
                let badge = match i {
                    2 => Some(Badge {
                        label: Some("3".into_fragment()),
                    }),
                    _ => None,
                };
                navbar::Item {
                    icon_active: Icon::Text {
                        text: (*icon).into_fragment(),
                        font: Some(icons_filled()),
                    },
                    icon_inactive: Icon::Text {
                        text: (*icon).into_fragment(),
                        font: Some(icons_outlined()),
                    },
                    badge,
                    label: (*label).into(),
                    on_press: Message::BarSelect(i),
                }
            })
            .collect();

        let compact = navbar(navbar::Style::new(&self.theme), items.clone())
            .active_index(self.bar_selected)
            .mode(navbar::Mode::Compact);
        let large = navbar(navbar::Style::new(&self.theme), items)
            .active_index(self.bar_selected)
            // Falls back to compact items when the pills don't fit, keeping
            // automatic (natural) item widths otherwise.
            .mode(navbar::Mode::Auto);

        // A placeholder for the view the bar would navigate to. The gallery
        // itself keeps following the rail.
        let virtual_view = container(
            column![
                text(format!(
                    "Virtual view: {}",
                    destinations[self.bar_selected].0
                ))
                .size(16.0)
                .color(self.theme.on_surface()),
                self.hint("A bar demo destination; the gallery screen keeps following the rail.",),
            ]
            .spacing(4.0),
        )
        .padding(16.0)
        .max_width(700.0)
        .center_x(Length::Fill)
        .style(|_| {
            container::Style::default()
                .background(self.theme.surface_container())
                .border(iced::Border::default().rounded(16.0))
        });

        column![
            self.section("Navigation bar"),
            self.hint(
                "Demo destinations, not the gallery's tabs; pressing an item updates the preview below.",
            ),
            container(compact).max_width(700.0).center_x(Length::Fill),
            container(large).max_width(700.0).center_x(Length::Fill),
            virtual_view,
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .into()
    }

    fn section(&self, title: &'static str) -> Element<'_, Message> {
        text(title)
            .font(text_bold())
            .size(18.0)
            .color(self.theme.on_surface())
            .into()
    }

    fn hint(&self, text_content: &'static str) -> Element<'_, Message> {
        text(text_content)
            .size(16.0)
            .color(self.theme.on_surface_variant())
            .into()
    }

    fn buttons_screen(&self) -> Element<'_, Message> {
        let styles = [
            button::Style::filled(&self.theme, Accent::default()),
            button::Style::tonal(&self.theme, Accent::default()),
            button::Style::elevated(&self.theme, Accent::default()),
            button::Style::outlined(&self.theme),
            button::Style::text(&self.theme, Accent::default()),
        ];
        let labels = ["Filled", "Tonal", "Elevated", "Outlined", "Text"];

        let enabled = styles
            .iter()
            .zip(labels)
            .map(|(style, label)| self.demo_button(*style, label, true, None, Some(Message::Noop)));
        // The text button has no selectable state in M3, so it is left out
        // of the togglable (selected) row altogether.
        let selected = (0..4).map(|i| {
            self.demo_button(
                styles[i],
                labels[i],
                true,
                Some(self.selected_buttons[i]),
                Some(Message::ToggleSelected(i)),
            )
        });
        let disabled = styles.iter().zip(labels).map(|(style, label)| {
            self.demo_button(*style, label, false, None, Some(Message::Noop))
        });

        let connected_group = connected_button_group(
            button::Style::filled(&self.theme, Accent::default()),
            (0..3usize)
                .map(|i| {
                    let selected = self.group_selected == i;
                    button_group::connected::Button {
                        content: button::Content::Full {
                            icon: Icon::Text {
                                text: (*STARS).into_fragment(),
                                font: Some(if selected {
                                    icons_filled()
                                } else {
                                    icons_outlined()
                                }),
                            },
                            label: format!("Star {}", i + 1).into(),
                        },
                        on_press: Some(Message::SelectGroup(i)),
                        selected,
                    }
                })
                .collect(),
        )
        .size(button_group::connected::Size::small());

        let standard_group = standard_button_group(
            (0..3)
                .map(|_| {
                    GroupItem::new(
                        iced_m3::widget::button(
                            button::Style::filled(&self.theme, Accent::default()),
                            button::Content::Full {
                                icon: Icon::Text {
                                    text: (*STARS).into_fragment(),
                                    font: Some(icons_filled()),
                                },
                                label: "Star".into(),
                            },
                        )
                        .on_press(Message::Noop),
                        true,
                    )
                })
                .collect::<Vec<GroupItem<'_, Message>>>(),
        )
        .spacing(standard::SPACING_MEDIUM);

        column![
            self.section("Buttons"),
            row(enabled).spacing(8.0).wrap(),
            self.hint("Selected (click to toggle)"),
            row(selected).spacing(8.0).wrap(),
            self.hint("Disabled"),
            row(disabled).spacing(8.0).wrap(),
            self.section("Connected button group (single select)"),
            connected_group,
            self.section("Standard button group"),
            standard_group,
            self.section("Button sizes"),
            row![
                self.size_button("Extra small", button::Size::extra_small()),
                self.size_button("Small", button::Size::small()),
                self.size_button("Medium", button::Size::medium()),
                self.size_button("Large", button::Size::large()),
                self.size_button("Extra large", button::Size::extra_large()),
            ]
            .spacing(12.0)
            .align_y(Alignment::Center)
            .wrap(),
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .into()
    }

    fn demo_button(
        &self,
        style: button::Style,
        label: &'static str,
        enabled: bool,
        selected: Option<bool>,
        on_press: Option<Message>,
    ) -> Element<'_, Message> {
        let content = button::Content::Full {
            icon: Icon::Text {
                text: (*STARS).into_fragment(),
                font: Some(icons_filled()),
            },
            label: label.into(),
        };

        let mut button =
            iced_m3::widget::button(style, content).on_press_maybe(on_press.filter(|_| enabled));
        if let Some(selected) = selected {
            button = button.selected(selected);
        }
        button.into()
    }

    fn size_button(&self, label: &'static str, size: button::Size) -> Element<'_, Message> {
        iced_m3::widget::button(
            button::Style::filled(&self.theme, Accent::default()),
            button::Content::Label(label.into()),
        )
        .size(size)
        .on_press(Message::Noop)
        .into()
    }

    fn inputs_screen(&self) -> Element<'_, Message> {
        let fields = column![
            text_input::TextInput::new("Search anything...", &self.search, &self.theme)
                .icon(text_input::Icon {
                    font: icons_outlined(),
                    code_point: *SEARCH,
                    size: Some(iced::Pixels(24.0)),
                    spacing: 8.0,
                    side: text_input::Side::Left,
                })
                .on_input(Message::SearchChanged)
                .width(Length::Fill),
            text_input::TextInput::new("name@example.com", &self.email, &self.theme)
                .with_label_text("Email", self.theme.background())
                .on_input(Message::EmailChanged)
                .width(Length::Fill),
            text_input::TextInput::new("Password", &self.password, &self.theme)
                .with_label_text("Password", self.theme.background())
                .secure(true)
                .on_input(Message::PasswordChanged)
                .width(Length::Fill),
        ]
        .spacing(12.0)
        .width(400.0);

        let slider_rows = column![
            self.slider_row("Volume", slider::Size::small()),
            self.slider_row("Volume (large)", slider::Size::large()),
        ]
        .spacing(12.0)
        .width(400.0);

        let switches = column![
            self.switch_row(
                "No icon",
                self.switches[0],
                switch::IconMode::Never,
                Some(Message::ToggleSwitch(0)),
            ),
            self.switch_row(
                "Icon when enabled",
                self.switches[1],
                switch::IconMode::WhenSelected,
                Some(Message::ToggleSwitch(1)),
            ),
            self.switch_row(
                "Always icon",
                self.switches[2],
                switch::IconMode::Always,
                Some(Message::ToggleSwitch(2)),
            ),
            self.switch_row("Disabled (on)", true, switch::IconMode::Always, None),
            self.switch_row("Disabled (off)", false, switch::IconMode::Never, None),
        ]
        .spacing(12.0);

        column![
            self.section("Text fields"),
            fields,
            self.section("Slider"),
            slider_rows,
            self.section("Switches"),
            self.hint("Icon modes / disabled state (no on-toggle handler = disabled)"),
            switches,
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .into()
    }

    fn switch_row(
        &self,
        label: &'static str,
        value: bool,
        icon_mode: switch::IconMode,
        toggle: Option<Message>,
    ) -> Element<'_, Message> {
        row![
            text(label).size(16.0).color(self.theme.on_surface()),
            space().width(Length::Fill),
            {
                let row_switch = switch(&self.theme, value).icon_mode(icon_mode);
                match toggle {
                    Some(toggle) => row_switch.on_toggle(toggle),
                    None => row_switch,
                }
            },
        ]
        .align_y(Alignment::Center)
        .spacing(12.0)
        .width(400.0)
        .into()
    }

    fn feedback_screen(&self) -> Element<'_, Message> {
        let bars = column![
            progress_bar(progress_bar::Style::new(&self.theme)).progress(0.25),
            progress_bar(progress_bar::Style::new(&self.theme))
                .progress(0.75)
                .height(16.0),
            progress_bar(progress_bar::Style::new(&self.theme)),
            progress_bar(progress_bar::Style::new(&self.theme)).height(16.0),
        ]
        .spacing(12.0)
        .width(400.0);

        let indicators = row![
            iced_m3::widget::loading_indicator(
                iced_m3::widget::loading_indicator::Style::contained(&self.theme,)
            ),
            iced_m3::widget::loading_indicator(
                iced_m3::widget::loading_indicator::Style::uncontained(&self.theme)
            ),
        ]
        .spacing(24.0);

        let badges = row![
            column![iced_m3::widget::icon(*HOME, 24.0).font(icons_outlined())].spacing(8.0),
            self.badge_chip(None),
            self.badge_chip(Some("9")),
            self.badge_chip(Some("999+")),
        ]
        .spacing(24.0)
        .align_y(Alignment::Center);

        column![
            self.section("Progress bars"),
            self.hint("Determinates (thin, thick) / indeterminates animate on their own"),
            bars,
            self.section("Loading indicator"),
            indicators,
            self.section("Badges"),
            badges,
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .into()
    }

    fn shapes_screen(&self) -> Element<'_, Message> {
        let shape_cards = ShapeKind::ALL.into_iter().map(|kind| {
            let selected = self.selected_shape == kind;
            let style = if selected {
                card::Style {
                    container_color: self.theme.primary_container(),
                    state_layer: StateLayer::new(self.theme.on_primary_container()),
                    ..card::Style::filled(&self.theme)
                }
            } else {
                card::Style::filled(&self.theme)
            };
            iced_m3::widget::card(
                style,
                center(
                    iced_m3::widget::shape(kind.polygon(), shapes::Style::new(&self.theme))
                        .size(64.0),
                ),
            )
            .interaction(card::Interaction::Press(Action::Direct(
                Message::SelectShape(kind),
            )))
            .width(140.0)
            .height(140.0)
            .into()
        });

        let shape_preview = row![
            iced_m3::widget::shape(
                self.selected_shape.polygon(),
                shapes::Style::new(&self.theme),
            )
            .transition_spring(fast_spatial(motion::Scheme::Expressive))
            .overshoot_margin(OvershootMargin::Auto)
            .size(220.0),
            column![
                text(format!("Morphing to: {}", self.selected_shape.label()))
                    .size(18.0)
                    .color(self.theme.on_surface()),
                self.hint("Morphs with an expressive spring; overshoot margin is automatic.",),
            ]
            .spacing(4.0),
        ]
        .align_y(Alignment::Center)
        .spacing(24.0);

        column![
            self.section("Morphing preview"),
            shape_preview,
            self.section("Shape cards"),
            self.hint("Pick a shape card; the preview above morphs to it."),
            row(shape_cards).spacing(12.0).wrap(),
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .into()
    }

    fn cards_screen(&self) -> Element<'_, Message> {
        let card_of = |style: card::Style, label: &'static str| -> Element<'static, Message> {
            let interaction = match self.card_mode {
                CardMode::Press => card::Interaction::Press(Action::Direct(Message::Noop)),
                CardMode::Disabled => card::Interaction::Disabled,
                CardMode::NonInteractive => card::Interaction::NonInteractive,
            };

            iced_m3::widget::card(
                style,
                center(text(label).size(20.0).color(self.theme.on_surface())),
            )
            .interaction(interaction)
            .width(260.0)
            .height(140.0)
            .into()
        };

        let interaction_group = connected_button_group(
            button::Style::filled(&self.theme, Accent::default()),
            [
                CardMode::Press,
                CardMode::Disabled,
                CardMode::NonInteractive,
            ]
            .into_iter()
            .map(|mode| button_group::connected::Button {
                content: button::Content::Label(mode.label().into()),
                on_press: Some(Message::SetCardMode(mode)),
                selected: self.card_mode == mode,
            })
            .collect(),
        )
        .size(button_group::connected::Size::small());

        let cards = column![
            row![
                card_of(card::Style::elevated(&self.theme), "Elevated"),
                card_of(card::Style::filled(&self.theme), "Filled"),
            ]
            .spacing(MAX_CARD_BETWEEN_PADDING),
            row![
                card_of(card::Style::outlined(&self.theme), "Outlined"),
                card_of(
                    card::Style {
                        container_color: self.theme.tertiary_container(),
                        state_layer: StateLayer::new(self.theme.on_tertiary_container()),
                        ..card::Style::filled(&self.theme)
                    },
                    "Custom container color"
                ),
            ]
            .spacing(MAX_CARD_BETWEEN_PADDING),
        ]
        .spacing(MAX_CARD_BETWEEN_PADDING);

        let fabs = row![
            iced_m3::widget::fab(
                fab::Style::fab_vibrant(&self.theme, Accent::Primary),
                fab::Content::Reguar {
                    icon: Icon::Text {
                        text: (*EDIT).into_fragment(),
                        font: Some(icons_filled()),
                    },
                },
                Action::Direct(Message::Noop),
            ),
            iced_m3::widget::fab(
                fab::Style::fab_tonal(&self.theme, Accent::Primary),
                fab::Content::Reguar {
                    icon: Icon::Text {
                        text: (*EDIT).into_fragment(),
                        font: Some(icons_filled()),
                    },
                },
                Action::Direct(Message::Noop),
            ),
            iced_m3::widget::fab(
                fab::Style::fab_vibrant(&self.theme, Accent::Tertiary),
                fab::Content::Extended {
                    icon: Icon::Text {
                        text: (*EDIT).into_fragment(),
                        font: Some(icons_filled()),
                    },
                    label: "Compose".into(),
                },
                Action::Direct(Message::Noop),
            ),
        ]
        .spacing(16.0)
        .align_y(Alignment::Center);

        column![
            self.section("Cards"),
            self.hint("Interaction mode applied to all cards below:"),
            interaction_group,
            cards,
            self.section("FABs"),
            fabs,
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .into()
    }

    /// The FAB menu pinned to the bottom right of the content pane, floating
    /// above the Cards screen independently from scrolling.
    fn fab_menu_overlay(&self) -> Element<'_, Message> {
        let menu = fab_menu(
            fab_menu::Style::new(&self.theme, Accent::default()),
            vec![
                fab_menu::Item {
                    on_press: Action::Direct(Message::Noop),
                    label: "Document".into_fragment(),
                    icon: Icon::Text {
                        text: (*DOCUMENT).into_fragment(),
                        font: Some(icons_filled()),
                    },
                },
                fab_menu::Item {
                    on_press: Action::Direct(Message::Noop),
                    label: "Message".into_fragment(),
                    icon: Icon::Text {
                        text: (*CHAT).into_fragment(),
                        font: Some(icons_filled()),
                    },
                },
                fab_menu::Item {
                    on_press: Action::Direct(Message::Noop),
                    label: "Folder".into_fragment(),
                    icon: Icon::Text {
                        text: (*FOLDER_SHARED).into_fragment(),
                        font: Some(icons_filled()),
                    },
                },
            ],
            None,
        );

        bottom_right(menu).into()
    }

    fn dialogs_screen(&self) -> Element<'_, Message> {
        let groups = vec![
            menu::Group::new(vec![
                menu::Item::Button(menu::button::Button {
                    icon: Some(Icon::Text {
                        text: (*VISIBILITY).into_fragment(),
                        font: Some(icons_outlined()),
                    }),
                    label: "Show details".into(),
                    supporting_text: None,
                    action: MenuAction::Message {
                        is_selected: false,
                        on_press: Some(Action::Direct(Message::Noop)),
                    },
                }),
                menu::Item::Button(menu::button::Button {
                    icon: Some(Icon::Text {
                        text: (*DELETE).into_fragment(),
                        font: Some(icons_outlined()),
                    }),
                    label: "Delete".into(),
                    supporting_text: None,
                    action: MenuAction::Message {
                        is_selected: false,
                        on_press: Some(Action::Direct(Message::Noop)),
                    },
                }),
                menu::Item::Separator,
                menu::Item::Button(menu::button::Button {
                    icon: Some(Icon::Text {
                        text: (*STARS).into_fragment(),
                        font: Some(icons_filled()),
                    }),
                    label: "Highlighted".into(),
                    supporting_text: None,
                    action: MenuAction::Message {
                        is_selected: true,
                        on_press: Some(Action::Direct(Message::Noop)),
                    },
                }),
            ]),
            menu::Group {
                label: Some("Feedback".into()),
                items: vec![
                    menu::Item::Button(menu::button::Button {
                        icon: None,
                        label: "Plain item".into(),
                        supporting_text: Some("Supporting text".into()),
                        action: MenuAction::Message {
                            is_selected: false,
                            on_press: Some(Action::Direct(Message::Noop)),
                        },
                    }),
                    menu::Item::Button(menu::button::Button {
                        icon: None,
                        label: "Disabled item".into(),
                        supporting_text: Some("Supporting text".into()),
                        action: MenuAction::Message {
                            is_selected: false,
                            on_press: None,
                        },
                    }),
                ],
            },
        ];

        let menu_surface = iced_m3::widget::menu(menu::Style::new(&self.theme, false), groups);

        // Anchored expandable menu (MenuOverlay) with a submenu.
        let sub_menu_surface = move || -> Menu<'_, Message> {
            let groups = vec![menu::Group {
                label: Some("More options".into()),
                items: vec![menu::Item::Button(menu::button::Button {
                    icon: Some(Icon::Text {
                        text: (*DELETE).into_fragment(),
                        font: Some(icons_outlined()),
                    }),
                    label: "Delete".into(),
                    supporting_text: None,
                    action: MenuAction::Message {
                        is_selected: false,
                        on_press: Some(Action::Direct(Message::Noop)),
                    },
                })],
            }];
            iced_m3::widget::menu(menu::Style::new(&self.theme, false), groups)
        };

        let drop_down_surface = move || -> Element<'_, Message> {
            let groups = vec![menu::Group::new(vec![
                menu::Item::Button(menu::button::Button {
                    icon: Some(Icon::Text {
                        text: (*VISIBILITY).into_fragment(),
                        font: Some(icons_outlined()),
                    }),
                    label: "Show details".into(),
                    supporting_text: None,
                    action: MenuAction::Message {
                        is_selected: false,
                        on_press: Some(Action::Direct(Message::Noop)),
                    },
                }),
                menu::Item::Button(menu::button::Button {
                    icon: Some(Icon::Text {
                        text: (*DELETE).into_fragment(),
                        font: Some(icons_outlined()),
                    }),
                    label: "More options".into(),
                    supporting_text: None,
                    action: MenuAction::SubMenu {
                        is_opened: self.drop_down_sub_menu_opened,
                        surface: Box::new(sub_menu_surface),
                        on_press: Action::Direct(Message::ToggleDropDownSubMenu),
                    },
                }),
            ])];
            iced_m3::widget::menu(menu::Style::new(&self.theme, false), groups).into()
        };

        let drop_down = advanced::menu_overlay(
            iced_m3::widget::button(
                button::Style::outlined(&self.theme),
                button::Content::Label("Menu".into()),
            )
            .selected(self.drop_down_opened)
            .on_press(Message::ToggleDropDown),
            drop_down_surface,
            self.drop_down_opened,
        )
        .on_press(|properties| {
            if let Some(position) = properties.position
                && !properties.bounds.contains(position)
            {
                Some(Message::ToggleDropDown)
            } else {
                None
            }
        });

        let dialog_trigger = row![
            iced_m3::widget::button(
                button::Style::filled(&self.theme, Accent::default()),
                button::Content::Label("Open dialog".into()),
            )
            .on_press(Message::OpenDialog),
            text(self.dialog_result.unwrap_or("Last action: none"))
                .size(16.0)
                .color(self.theme.on_surface_variant()),
        ]
        .align_y(Alignment::Center)
        .spacing(12.0);

        column![
            self.section("Menu"),
            self.hint("Groups, separators, labels, supporting text, selection"),
            row![menu_surface, drop_down].spacing(24.0),
            self.section("Dialog"),
            dialog_trigger,
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .into()
    }

    fn theme_screen(&self) -> Element<'_, Message> {
        let mode = row![
            column![
                text("Dark theme").size(16.0).color(self.theme.on_surface()),
                self.hint("Switches every widget between the dark and the light palette.",),
            ]
            .spacing(2.0)
            .width(Length::Fill),
            switch(&self.theme, matches!(self.theme.mode, Mode::Dark))
                .on_toggle(Message::ToggleTheme),
        ]
        .align_y(Alignment::Center)
        .spacing(12.0)
        .width(400.0);

        let roles = column(
            [
                ("Background", self.theme.background()),
                ("Surface container low", self.theme.surface_container_low()),
                ("Surface container", self.theme.surface_container()),
                (
                    "Surface container high",
                    self.theme.surface_container_high(),
                ),
                ("Primary", self.theme.primary()),
                ("Primary container", self.theme.primary_container()),
                ("Secondary", self.theme.secondary()),
                ("Secondary container", self.theme.secondary_container()),
                ("Tertiary", self.theme.tertiary()),
                ("Tertiary container", self.theme.tertiary_container()),
                ("Error", self.theme.error()),
                ("Error container", self.theme.error_container()),
                ("Outline", self.theme.outline()),
            ]
            .into_iter()
            .map(|(label, color)| self.swatch_row(label, color)),
        )
        .spacing(10.0);

        column![
            self.section("Mode"),
            mode,
            self.section("Color roles"),
            self.hint("The roles every widget style is derived from; they follow the mode."),
            roles,
        ]
        .spacing(12.0)
        .width(Length::Fill)
        .into()
    }

    fn swatch_row(&self, label: &'static str, color: Color) -> Element<'_, Message> {
        row![
            text(label).size(16.0).color(self.theme.on_surface()),
            space().width(Length::Fill),
            container(space().width(32.0).height(32.0)).style(move |_| {
                container::Style::default()
                    .background(color)
                    .border(iced::Border::default().rounded(8.0))
            }),
        ]
        .align_y(Alignment::Center)
        .spacing(12.0)
        .width(400.0)
        .into()
    }

    fn slider_row(&self, title: &'static str, size: slider::Size) -> Element<'_, Message> {
        column![
            row![
                text(title)
                    .size(16.0)
                    .color(self.theme.on_surface_variant()),
                space().width(Length::Fill),
                text(format!("{}%", (self.slider_value * 100.0).round() as u32))
                    .size(16.0)
                    .color(self.theme.on_surface()),
            ]
            .width(Length::Fill),
            slider(
                0.0..=1.0,
                self.slider_value,
                Message::SliderChanged,
                slider::Style::new(&self.theme)
            )
            .step(0.01_f32)
            .size(size)
            .width(Length::Fill),
        ]
        .spacing(4.0)
        .width(400.0)
        .into()
    }

    fn badge_chip(&self, label_text: Option<&'static str>) -> Element<'_, Message> {
        let icon = iced_m3::widget::icon(*HOME, 24.0).font(icons_outlined());
        let badge_style = iced_m3::widget::badge::Style::new(&self.theme);

        let base: Element<'_, Message> = match label_text {
            Some(label) => iced_m3::widget::badge(badge_style, icon)
                .label(label)
                .label_font(text_bold())
                .bounds_mode(iced_m3::widget::badge::BoundsMode::Symmetrical)
                .into(),
            None => iced_m3::widget::badge(badge_style, icon).into(),
        };

        container(base)
            .style(|_| {
                container::Style::default()
                    .background(self.theme.secondary_container())
                    .border(iced::Border::default().rounded(f32::MAX))
            })
            .center_x(56.0)
            .center_y(32.0)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => {
                // Pressing a menu item closes every drop-down.
                self.drop_down_opened = false;
                self.drop_down_sub_menu_opened = false;
            }
            Message::GoTo(tab) => self.tab = tab,
            Message::BarSelect(i) => self.bar_selected = i,
            Message::ToggleExpanded => self.expanded = !self.expanded,
            Message::ToggleTheme => {
                self.theme.mode = match self.theme.mode {
                    Mode::Dark => Mode::Light,
                    Mode::Light => Mode::Dark,
                };
            }
            Message::SearchChanged(value) => self.search = value,
            Message::EmailChanged(value) => self.email = value,
            Message::PasswordChanged(value) => self.password = value,
            Message::SliderChanged(value) => self.slider_value = value,
            Message::ToggleSwitch(i) => self.switches[i] = !self.switches[i],
            Message::SelectGroup(i) => self.group_selected = i,
            Message::ToggleSelected(i) => self.selected_buttons[i] = !self.selected_buttons[i],
            Message::SetCardMode(mode) => self.card_mode = mode,
            Message::SelectShape(kind) => self.selected_shape = kind,
            Message::ToggleDropDown => {
                self.drop_down_sub_menu_opened = false;
                self.drop_down_opened = !self.drop_down_opened;
            }
            Message::ToggleDropDownSubMenu => {
                self.drop_down_sub_menu_opened = !self.drop_down_sub_menu_opened;
            }
            Message::OpenDialog => {
                self.dialog_open = true;
                self.dialog_result = None;
            }
            Message::CloseDialog(accepted) => {
                self.dialog_open = false;
                self.dialog_result = Some(if accepted {
                    "Last action: accepted"
                } else {
                    "Last action: dismissed"
                });
            }
        }
        Task::none()
    }
}

impl Tab {
    const DEFAULT_TABS: [Tab; 8] = [
        Tab::Buttons,
        Tab::Inputs,
        Tab::Feedback,
        Tab::Cards,
        Tab::Shapes,
        Tab::Dialogs,
        Tab::Navbar,
        Tab::Theme,
    ];
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
    .window(iced::window::Settings {
        size: Size::new(900.0, 900.0),
        min_size: Some(Size::new(900.0, 900.0)),
        ..Default::default()
    })
    .default_font(Font::with_name(fonts::NAME))
    .run()
}
