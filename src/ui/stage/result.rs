use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};

use crate::state::global::GlobalState;

/// 算卦结果
pub struct ResultView;

impl ResultView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    pub fn new(_: &mut Window, _: &mut App) -> Self {
        Self {}
    }
}

impl Render for ResultView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let gua_result = GlobalState::state(cx).result.clone();

        match gua_result {
            None => div().child("还没有进行算卦！".to_string()),
            Some(result) => div().p_2().child(cx.new(|_| result.clone())),
        }
    }
}
