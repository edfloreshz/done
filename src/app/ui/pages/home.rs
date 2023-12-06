use iced::widget::{container, text};

use crate::app::ui::widget::Element;

#[derive(Debug, Default)]
pub struct Home;

#[derive(Debug, Clone)]
pub enum Message {}

impl Home {
	pub fn view<'a>(&'a self) -> Element<'a, Message> {
		container(text("Home")).into()
	}
}
