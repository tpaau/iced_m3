use iced_widget::text;

pub enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

pub struct Badge<'a> {
    pub label: Option<text::Fragment<'a>>,
}

pub struct BadgeIcon<'a> {
    pub icon: text::Fragment<'a>,
    pub badge: Option<Badge<'a>>,
}
