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
    div().flex()
}

pub fn sidebar() -> impl IntoElement {
    div()
        .flex()
        .w_64()
        .bg(rgb(0xdcd8d8))
        .p_2()
        .text_color(rgb(0xffffff))
}

pub fn titlebar(title: String) -> impl IntoElement {
    div()
        .flex()
        .h_8()
        .w_full()
        .bg(rgb(0xdcd8d8))
        .justify_center()
        .items_center()
        .text_color(rgb(0xffffff))
        .child(title)
}
