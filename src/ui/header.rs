use gpui::{
    App, AppContext, Context, Entity, ParentElement, Render, Styled, Window, div,
    prelude::FluentBuilder, px,
};
use gpui_component::{
    ActiveTheme, Icon, IconName,
    button::{Button, ButtonVariants},
    h_flex,
};

use crate::state::app_state::AppState;

/**
 * 应用的标题栏
 */
pub struct Header {}

impl Header {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for Header {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        h_flex().child(
            div()
                .bg(cx.theme().sidebar)
                .border_color(cx.theme().sidebar_border)
                .border_r_1()
                .w(px(255.))
                .pl_24()
                .when(AppState::state(cx).sidebar_collapsed, |this| {
                    this.w_12().bg(cx.theme().background).border_r_0()
                })
                .child(
                    Button::new("sidebar-toggle")
                        .ghost()
                        .icon(Icon::new(IconName::PanelLeftClose))
                        .on_click(|_, _, cx| toggle_sidebar(cx))
                        .when(AppState::state(cx).sidebar_collapsed, |b| {
                            b.icon(Icon::new(IconName::PanelLeftOpen))
                        }),
                ),
        )
    }
}

/**
 * 切换侧边栏状态
 */
fn toggle_sidebar(cx: &mut App) {
    let state = AppState::state_mut(cx);

    state.sidebar_collapsed = !state.sidebar_collapsed;
}
