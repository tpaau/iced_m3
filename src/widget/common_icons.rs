use std::sync::LazyLock;

use iced::advanced::svg;

const CHECK_BYTES: &[u8] = include_bytes!("../../resources/svg/check.svg");
pub(crate) static CHECK: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(CHECK_BYTES));

const CLOSE_BYTES: &[u8] = include_bytes!("../../resources/svg/close.svg");
pub(crate) static CLOSE: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(CLOSE_BYTES));

const MENU_BYTES: &[u8] = include_bytes!("../../resources/svg/menu.svg");
pub(crate) static MENU: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(MENU_BYTES));

const MENU_OPEN_BYTES: &[u8] = include_bytes!("../../resources/svg/menu_open.svg");
pub(crate) static MENU_OPEN: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(MENU_OPEN_BYTES));

const ADD_BYTES: &[u8] = include_bytes!("../../resources/svg/add.svg");
pub(crate) static ADD: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(ADD_BYTES));

const ARROW_RIGHT_BYTES: &[u8] = include_bytes!("../../resources/svg/arrow_right.svg");
pub(crate) static ARROW_RIGHT: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(ARROW_RIGHT_BYTES));
