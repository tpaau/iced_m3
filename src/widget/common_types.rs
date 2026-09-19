use iced_widget::text;

pub enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

pub struct Badge<'a> {
    pub label: Option<text::Fragment<'a>>,
}

// TODO: SVG icons
pub struct Icon<'a> {
    pub icon: text::Fragment<'a>,
    pub badge: Option<Badge<'a>>,
}
