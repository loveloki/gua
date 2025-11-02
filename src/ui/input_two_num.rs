use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, div,
};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
    input::{InputState, TextInput},
};

use crate::gua::ba_gua::BaGuaCalculator;

/**
 * 输入两个数字来计算卦象
 */

pub struct InputTwoNum {
    input1_state: Entity<InputState>,
    input2_state: Entity<InputState>,
    result_text: SharedString, // 八卦的结果
}

impl InputTwoNum {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn calc_result(&mut self, cx: &mut Context<Self>) {
        let value1: u16 = self.input1_state.read(cx).value().parse().unwrap_or(0);
        let value2: u16 = self.input2_state.read(cx).value().parse().unwrap_or(0);

        let ba_gua_result = BaGuaCalculator::calculate_from_two_numbers(value1, value2);

        self.result_text = ba_gua_result.display();

        cx.notify();
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input1_state = cx.new(|cx| InputState::new(window, cx).placeholder("输入数字1"));
        let input2_state = cx.new(|cx| InputState::new(window, cx).placeholder("输入数字2"));

        Self {
            input1_state,
            input2_state,
            result_text: SharedString::default(),
        }
    }
}

impl Render for InputTwoNum {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        div()
            .w_80()
            .p_2()
            .v_flex()
            .gap_2()
            .child(
                Button::new("quit")
                    .danger()
                    .outline()
                    .label("退出")
                    .on_click(|_, _, cx| cx.quit()),
            )
            .child(TextInput::new(&self.input1_state))
            .child(TextInput::new(&self.input2_state))
            .child(self.result_text.clone())
            .child(
                Button::new("calc")
                    .label("开始计算")
                    .primary()
                    .on_click(move |_, _, cx| {
                        cx.update_entity(
                            &entity,
                            |input: &mut InputTwoNum, context: &mut Context<InputTwoNum>| {
                                input.calc_result(context)
                            },
                        );
                    }),
            )
    }
}
