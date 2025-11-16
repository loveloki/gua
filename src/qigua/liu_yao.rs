use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, div,
};
use gpui_component::{
    Disableable, StyledExt,
    button::{Button, ButtonVariants},
};

use crate::qigua::core::QiGuaCore;

const NAME: &str = "六爻";

/**
 * 六爻起卦
 */
pub struct LiuYao {
    content: Entity<LiuYaoContent>,
}

impl LiuYao {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let content = LiuYaoContent::view(window, cx);

        Self { content }
    }
}

impl Render for LiuYao {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().p_2().child(self.content.clone())
    }
}

/**
 * 输入六爻来计算卦象
 */
pub struct LiuYaoContent {}

impl LiuYaoContent {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for LiuYaoContent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        div()
            .w_80()
            .p_2()
            .v_flex()
            .gap_2()
            .child(NAME)
            .child("暂未实现！")
            .child(
                Button::new("calc")
                    .label("开始计算")
                    .primary()
                    .disabled(true)
                    .on_click(move |_, _, cx| {
                        cx.update_entity(
                            &entity,
                            |input: &mut LiuYaoContent, context: &mut Context<LiuYaoContent>| {
                                input.calc_gua(context)
                            },
                        );
                    }),
            )
    }
}

impl QiGuaCore for LiuYaoContent {
    fn calc_gua(&mut self, cx: &mut Context<Self>) {}
    fn name() -> SharedString {
        NAME.into()
    }
}
