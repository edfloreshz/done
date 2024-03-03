use gpui::*;

pub struct Done {
	pub title: SharedString,
}

impl Render for Done {
	fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
		div()
			.flex_col()
			.size_full()
			.bg(rgb(0xffffff))
			.text_color(rgb(0xffffff))
			.child(titlebar(self.title.to_string()))
			.child(div().flex().size_full().child(sidebar()).child(content()))
	}
}

pub fn content() -> impl IntoElement {
	div().flex().text_color(rgb(0x1e1e1e))
}

pub fn sidebar() -> impl IntoElement {
	div()
		.flex()
		.w_64()
		.p_2()
		.bg(rgb(0xdcd8d8))
		.text_color(rgb(0x1e1e1e))
}

pub fn titlebar(title: String) -> impl IntoElement {
	div()
		.flex()
		.h_8()
		.w_full()
		.bg(rgb(0xdcd8d8))
		.justify_center()
		.items_center()
		.text_color(rgb(0x1e1e1e))
		.text_sm()
		.border_color(rgb(0xc4c4c4))
		.border_b()
		.child(title)
}
