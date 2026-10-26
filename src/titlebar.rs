use gpui_kit::{
    base::h_flex,
    component::{ActiveTheme, Icon},
    prelude::*,
    *,
};

use crate::phosphor;

pub struct Titlebar {
    menu_enabled: bool,
}

impl Titlebar {
    pub fn new() -> Self {
        Self {
            menu_enabled: false,
        }
    }

    pub fn window_buttons(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> Div {
        let minimize_btn = div()
            .id("quit-window")
            .size_10()
            .flex()
            .items_center()
            .justify_center()
            .hover(|this| this.bg(white().opacity(0.5)))
            .child(phosphor!(minus).size_3p5())
            .window_control_area(WindowControlArea::Min);

        let zoom_btn = div()
            .id("quit-window")
            .child(
                if window.is_maximized() {
                    phosphor!(copy)
                } else {
                    phosphor!(square)
                }
                .size_3p5(),
            )
            .size_10()
            .flex()
            .items_center()
            .justify_center()
            .hover(|this| this.bg(white().opacity(0.5)))
            .window_control_area(WindowControlArea::Max);

        let quit_btn = div()
            .id("quit-window")
            .size_10()
            .flex()
            .items_center()
            .justify_center()
            .hover(|this| this.bg(red()).text_color(white()))
            .child(phosphor!(x).size_4())
            .window_control_area(WindowControlArea::Close);

        h_flex().child(minimize_btn).child(zoom_btn).child(quit_btn)
    }

    pub fn menu(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> Div {
        if self.menu_enabled {
            div().child("pl")
        } else {
            h_flex()
                .text_sm()
                .child(div().id("openmenu").child(phosphor!(list)))
                .child(div().child("Home"))
        }
    }
}

impl Render for Titlebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        h_flex()
            .justify_between()
            .bg(theme.title_bar)
            .border_b_1()
            .border_color(theme.title_bar_border)
            .h_10()
            .child(self.menu(window, cx))
            .child(
                div()
                    .flex_grow_1()
                    .h_10()
                    .window_control_area(WindowControlArea::Drag),
            )
            .child(self.window_buttons(window, cx))
    }
}
