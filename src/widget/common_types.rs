use iced_widget::text;

use crate::widget::hybrid_icon::Icon;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Badge<'a> {
    pub label: Option<text::Fragment<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BadgeIcon<'a> {
    pub icon: Icon<'a>,
    pub badge: Option<Badge<'a>>,
}
