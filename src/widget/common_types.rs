use iced_widget::text;

pub enum OnPress<'a, Message>
where
    Message: Clone,
{
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<'a, Message> OnPress<'a, Message>
where
    Message: Clone,
{
    pub fn resolve(&self) -> Message {
        match self {
            Self::Direct(message) => message.clone(),
            Self::Closure(callback) => callback(),
        }
    }
}

pub struct Badge<'a> {
    pub label: Option<text::Fragment<'a>>,
}

pub struct BadgeIcon<'a> {
    pub icon: text::Fragment<'a>,
    pub badge: Option<Badge<'a>>,
}
