use apply::Apply;
use iced::widget::button::StyleSheet;
use iced::widget::{self, button, text};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};

use crate::app::ui::theme::Theme;
use crate::app::ui::widget::Button;

pub fn icon_button<'a, Message: std::clone::Clone + 'a>(
	icon: Icon,
	on_press: Message,
	style: <Theme as StyleSheet>::Style,
) -> Button<'a, Message> {
	button(
		text(icon_to_char(icon).to_string())
			.font(ICON_FONT)
			.size(12.0)
			.apply(widget::container)
			.padding(5.0)
			.center_x()
			.center_y(),
	)
	.style(style)
	.on_press(on_press)
}
