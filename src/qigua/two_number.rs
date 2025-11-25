use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, div,
};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
    input::{Input, InputState},
};

use crate::{core::ba_gua::BaGuaCalculator, qigua::core::QiGuaCore, state::global::GlobalState};

const NAME: &str = "两个数字";

/// 两个数字起卦
pub struct TwoNumber {
    content: Entity<InputTwoNumContent>,
}

impl TwoNumber {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let content = InputTwoNumContent::view(window, cx);

        Self { content }
    }
}

impl Render for TwoNumber {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().p_2().child(self.content.clone())
    }
}

/// 输入两个数字来计算卦象
pub struct InputTwoNumContent {
    input1_state: Entity<InputState>,
    input2_state: Entity<InputState>,
}

impl InputTwoNumContent {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input1_state = cx.new(|cx| InputState::new(window, cx).placeholder("输入数字1"));
        let input2_state = cx.new(|cx| InputState::new(window, cx).placeholder("输入数字2"));

        Self {
            input1_state,
            input2_state,
        }
    }
}

impl Render for InputTwoNumContent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        div()
            .w_80()
            .p_2()
            .v_flex()
            .gap_2()
            .child(NAME)
            .child(Input::new(&self.input1_state))
            .child(Input::new(&self.input2_state))
            .child(
                Button::new("calc")
                    .label("开始计算")
                    .primary()
                    .on_click(move |_, _, cx| {
                        cx.update_entity(
                            &entity,
                            |input: &mut InputTwoNumContent,
                             context: &mut Context<InputTwoNumContent>| {
                                input.calc_gua(context)
                            },
                        );
                    }),
            )
    }
}

impl QiGuaCore for InputTwoNumContent {
    fn calc_gua(&mut self, cx: &mut Context<Self>) {
        let shang_num: u16 = self.input1_state.read(cx).value().parse().unwrap_or(0);
        let xia_num: u16 = self.input2_state.read(cx).value().parse().unwrap_or(0);

        let ba_gua_result =
            BaGuaCalculator::calculate_from_two_numbers(shang_num, xia_num, shang_num + xia_num);

        let gua_result = GlobalState::state_mut(cx);
        gua_result.result = Some(ba_gua_result.clone());

        cx.notify();
    }
    fn name() -> SharedString {
        NAME.into()
    }
}
