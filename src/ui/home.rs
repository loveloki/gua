use crate::gua::ba_gua::BaGuaCalculator;
use gpui::*;
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
    input::{InputEvent, InputState, TextInput},
};

pub struct HomeWindow {
    input1_state: Entity<InputState>,
    input2_state: Entity<InputState>,
    display_text: SharedString, // 八卦的结果
    _subscriptions1: Vec<Subscription>,
    _subscriptions2: Vec<Subscription>,
}

impl HomeWindow {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input1_state = cx.new(|cx| InputState::new(window, cx).placeholder("输入数字1"));
        let input2_state = cx.new(|cx| InputState::new(window, cx).placeholder("输入数字2"));

        let _subscriptions1 = vec![cx.subscribe_in(&input1_state, window, {
            let input1_state = input1_state.clone();
            let input2_state = input2_state.clone();

            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let value1: u16 = input1_state.read(cx).value().parse().unwrap_or(0);
                    let value2: u16 = input2_state.read(cx).value().parse().unwrap_or(0);

                    let ba_gua_result = BaGuaCalculator::calculate_from_two_numbers(value1, value2);

                    this.display_text = ba_gua_result.display();

                    cx.notify();
                }
                _ => {}
            }
        })];

        let _subscriptions2 = vec![cx.subscribe_in(&input2_state, window, {
            let input1_state = input1_state.clone();
            let input2_state = input2_state.clone();

            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let value1: u16 = input1_state.read(cx).value().parse().unwrap_or(0);
                    let value2: u16 = input2_state.read(cx).value().parse().unwrap_or(0);

                    let ba_gua_result = BaGuaCalculator::calculate_from_two_numbers(value1, value2);

                    this.display_text = ba_gua_result.display();

                    cx.notify();
                }
                _ => {}
            }
        })];

        Self {
            input1_state,
            input2_state,
            display_text: SharedString::default(),
            _subscriptions1: _subscriptions1,
            _subscriptions2: _subscriptions2,
        }
    }
}

impl Render for HomeWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .min_w_32()
            .p_2()
            .v_flex()
            .gap_2()
            .child(
                Button::new("quit")
                    .primary()
                    .label("退出")
                    .on_click(|_, _, cx| cx.quit()),
            )
            .child(TextInput::new(&self.input1_state))
            .child(TextInput::new(&self.input2_state))
            .child(self.display_text.clone())
    }
}
