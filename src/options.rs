use gpui::*;

pub fn window_options() -> WindowOptions {
    WindowOptions {
        bounds: WindowBounds::Fixed(Bounds {
            origin: Default::default(),
            size: size(px(600.), px(700.)).into(),
        }),
        titlebar: Some(TitlebarOptions {
            title: Some("Done".into()),
            appears_transparent: true,
            ..Default::default()
        }),
        ..Default::default()
    }
}
