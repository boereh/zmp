use gpui_kit::assets::AllAssets;
use gpui_kit::base::v_flex;
use gpui_kit::*;

mod macros;
mod titlebar;

struct ZMP;

impl Render for ZMP {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(cx.new(|_| titlebar::Titlebar::new()))
    }
}

fn main() {
    application().with_assets(AllAssets).run(|cx| {
        init(cx);

        open_window(
            WindowOptions {
                titlebar: None,
                ..WindowOptions::default()
            },
            cx,
            |_, cx| cx.new(|_| ZMP),
        )
        .expect("Failed to open window");
    });
}
