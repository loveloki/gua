use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, px,
    rgb, rgba,
};
use gpui_component::h_flex;

use crate::{gua::ba_gua::GuaResultStep, state::global::GlobalState};

/// 算卦结果
pub struct ResultView;

impl ResultView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    pub fn new(_: &mut Window, _: &mut App) -> Self {
        Self {}
    }

    /// 获取步骤内容
    fn step_content_with_color(&self, steps: Vec<GuaResultStep>) -> impl IntoElement {
        let widths: Vec<f32> = vec![400., 300., 500.];

        div()
            .w(gpui::px(1200.0))
            .border_1()
            .border_color(rgb(0x2D2F34))
            .rounded_md()
            .overflow_hidden()
            .child(
                h_flex()
                    .w_full()
                    .border_b_1()
                    .border_color(rgb(0x2D2F34))
                    .bg(rgba(0xB3B1AD))
                    .child(div().w(px(widths[0])).p_2().child("原始值"))
                    .child(div().w(px(widths[1])).p_2().child("操作"))
                    .child(div().w(px(widths[2])).p_2().child("结果")),
            )
            .children(steps.iter().map(|s| {
                h_flex()
                    .child(div().w(px(widths[0])).p_2().child(s.origin.clone()))
                    .child(
                        div()
                            .w(px(widths[1]))
                            .p_2()
                            .bg(rgb(0xB3B1AD))
                            .child(s.description.clone()),
                    )
                    .child(div().w(px(widths[2])).p_2().child(s.result.clone()))
            }))
    }
}

impl Render for ResultView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let gua_result = GlobalState::state(cx).result.clone();

        match gua_result {
            None => div().child("还没有进行算卦！".to_string()),
            Some(result) => div()
                .p_2()
                .child(cx.new(|_| result.clone()))
                .child(self.step_content_with_color(result.steps)),
        }
    }
}
