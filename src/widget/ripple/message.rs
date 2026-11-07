use iced::Point;

use super::model::Pointer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Message {
    /// The mouse cursor entered (`true`) or left (`false`) the bounds.
    Hover(bool),
    /// `at` is relative to the top-left corner of the bounds.
    Press {
        pointer: Pointer,
        at: Point,
    },
    Release {
        pointer: Pointer,
    },
    Click,
    Cancel {
        pointer: Pointer,
    },
    ContextMenu,
    Tick,
}
