use iced::Element;

pub mod drop_down_menu;

pub fn drop_down_menu<'a, Message, Theme, Renderer>(
    trigger: impl Fn(bool) -> Element<'a, Message, Theme, Renderer> + 'a,
    menu: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
    placement: drop_down_menu::Placement,
) -> drop_down_menu::DropDownMenu<'a, Message, Theme, Renderer> {
    drop_down_menu::DropDownMenu::<'a, Message, Theme, Renderer>::new(trigger, menu, placement)
}
