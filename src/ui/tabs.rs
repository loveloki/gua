use gpui::*;
use gpui_component::{
    tab::{Tab, TabBar},
    v_flex,
};

use crate::ui::input_two_num::InputTwoNumPanel;

/**
 * 展示的页面
 */
enum DisplayWindow {
    Home = 0,
    InputTwoNumber,
    Result,
}

/**
 * 切换算卦和结果
 */
pub struct Tabs {
    active_tab: usize,
    input_two_num_panel: Entity<InputTwoNumPanel>,
}

impl Tabs {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_two_num_panel = InputTwoNumPanel::view(window, cx);

        Self {
            active_tab: 0,
            input_two_num_panel,
        }
    }

    fn render_tab_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_tab {
            0 => div().child("主页"),
            1 => div().child(self.input_two_num_panel.clone()),
            2 => div().child("Result content"),
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
