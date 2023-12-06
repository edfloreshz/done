use iced::widget::{container, text};

use crate::app::ui::widget::Element;

#[derive(Debug, Default)]
pub struct Welcome;

#[derive(Debug, Clone)]
pub enum Message {}

impl Welcome {
	pub fn view<'a>(&'a self) -> Element<'a, Message> {
		container(text("Welcome!")).into()
	}
}
