use std::usize;

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    tab::{Tab, TabBar},
    v_flex,
};
use rand::Rng;

use super::result::ResultPanel;
use crate::{state::global::GlobalState, ui::input_two_num::InputTwoNumPanel};

/**
 * 切换算卦和结果
 */
pub struct Tabs {
    active_tab: usize,
    input_two_num_panel: Entity<InputTwoNumPanel>,
    result_panel: Entity<ResultPanel>,
    random_gua_info_index: usize,
}

/**
 * 从 64 卦中获取随机的卦象信息
 */
fn get_random_gua_index(cx: &mut App, exclude: usize) -> usize {
    let gua64_info_list = GlobalState::state(cx).gua64_info_list.len();
    let mut random_index = rand::rng().random_range(0..gua64_info_list);

    while random_index == exclude {
        random_index = rand::rng().random_range(0..gua64_info_list);
    }

    random_index
}

impl Tabs {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_two_num_panel = InputTwoNumPanel::view(window, cx);
        let result_panel = ResultPanel::view(window, cx);
        let random_gua_info_index = get_random_gua_index(cx, usize::MAX);

        Self {
            active_tab: 0,
            input_two_num_panel,
            result_panel,
            random_gua_info_index,
        }
    }

    fn render_tab_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_tab {
            0 => div()
                .flex()
                .flex_col()
                .child(
                    div().flex().gap_1().children([
                        Button::new("quit")
                            .danger()
                            .outline()
                            .label("退出")
                            .on_click(|_, _, cx| cx.quit()),
                        Button::new("random-gua")
                            .outline()
                            .label("随机展示一卦的信息")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.random_gua_info_index =
                                    get_random_gua_index(cx, this.random_gua_info_index)
                            })),
                    ]),
                )
                .gap_3()
                .child({
                    let gua64_info_list = GlobalState::state(cx).gua64_info_list.clone();
                    let gua64_info = gua64_info_list
                        .get(self.random_gua_info_index)
                        .unwrap()
                        .clone();

                    cx.new(|_| gua64_info)
                }),
            1 => div().child(self.input_two_num_panel.clone()),
            2 => div().child(self.result_panel.clone()),
            _ => div(),
        }
    }
}

impl Render for Tabs {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(
                TabBar::new("tabs")
                    .selected_index(self.active_tab)
                    .on_click(cx.listener(|view, index, _, cx| {
                        view.active_tab = *index;
                        cx.notify();
                    }))
                    .child(Tab::new("主页"))
                    .child(Tab::new("算卦1"))
                    .child(Tab::new("结果")),
            )
            .child(div().flex_1().p_4().child(self.render_tab_content(cx)))
    }
}
