use iced::widget::{container, text};

use crate::app::ui::widget::Element;

#[derive(Debug, Default)]
pub struct Help;

#[derive(Debug, Clone)]
pub enum Message {}

impl Help {
	pub fn view<'a>(&'a self) -> Element<'a, Message> {
		container(text("Help")).into()
	}
}
