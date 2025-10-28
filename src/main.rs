mod app_menus;
mod ba_gua;

use ba_gua::BaGuaCalculator;
use gpui::*;
use gpui_component::{
    button::*,
    input::{InputEvent, InputState, TextInput},
    *,
};

pub struct MainWindow {
    number_input: Entity<NumberInput>,
}

impl Render for MainWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child(self.number_input.clone())
    }
}

pub struct NumberInput {
    input1_state: Entity<InputState>,
    input2_state: Entity<InputState>,
    display_text: SharedString, // 八卦的结果
    _subscriptions1: Vec<Subscription>,
    _subscriptions2: Vec<Subscription>,
}
impl NumberInput {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input1_state = cx.new(|cx| InputState::new(window, cx).placeholder("输入数字1"));
        let input2_state = cx.new(|cx| InputState::new(window, cx).placeholder("输入数字2"));

        let _subscriptions1 = vec![cx.subscribe_in(&input1_state, window, {
            let input1_state = input1_state.clone();
            let input2_state = input2_state.clone();

            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let value1: u8 = input1_state.read(cx).value().parse().unwrap_or(0);
                    let value2: u8 = input2_state.read(cx).value().parse().unwrap_or(0);

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
                    let value1: u8 = input1_state.read(cx).value().parse().unwrap_or(0);
                    let value2: u8 = input2_state.read(cx).value().parse().unwrap_or(0);

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

impl Render for NumberInput {
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

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        app_menus::init("简单的八卦计算器", cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let number_input = cx.new(|cx| NumberInput::new(window, cx));

                let view = cx.new(|_| MainWindow { number_input });
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view.into(), window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
