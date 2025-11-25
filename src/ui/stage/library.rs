use gpui::*;
use gpui_component::button::Button;
use rand::Rng;

use crate::{
    core::basic::{Gua64, Gua64Info},
    state::global::GlobalState,
    ui::{home::Stage, sidebar::StageItem},
};

/// 查询基础内容
pub struct Library {
    /// 当前卦象的索引
    gua_index: usize,
    /// 六十四卦信息列表
    gua64: Vec<Gua64Info>,
}

impl Library {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let gua64 = GlobalState::state(cx).gua64_info_list.clone();

        Self {
            gua_index: 0,
            gua64,
        }
    }

    /// 随机展示一个卦象
    fn change_random(&mut self) {
        let mut random_index = rand::rng().random_range(0..self.gua64.len());

        while random_index == self.gua_index {
            random_index = rand::rng().random_range(0..self.gua64.len());
        }

        self.gua_index = random_index;
    }
}

impl Render for Library {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p_2()
            .child(
                div().flex().gap_1().child(
                    Button::new("random-gua")
                        .outline()
                        .label("随机来一卦")
                        .on_click(cx.listener(|this, _, _, _| {
                            this.change_random();
                        })),
                ),
            )
            .gap_3()
            .child({
                let gua_info = self.gua64.get(self.gua_index).unwrap().clone();

                cx.new(|_| gua_info)
            })
    }
}

impl Stage for Library {
    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn get_id() -> StageItem {
        StageItem::Library
    }
}
