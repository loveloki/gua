use gpui::*;
use gpui_component::{h_flex, v_flex};

use crate::{
    qigua::{core::QiGuaCore, liu_yao::LiuYao, time::Time, two_number::TwoNumber},
    ui::{home::Stage, sidebar::StageItem, stage::result::ResultView},
};

/**
 * 算卦和结果
 */
pub struct QiGua {
    two_number: Entity<TwoNumber>,
    time: Entity<Time>,
    liu_yao: Entity<LiuYao>,
    result: Entity<ResultView>,
}

impl QiGua {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let two_number = TwoNumber::view(window, cx);
        let liu_yao = LiuYao::view(window, cx);
        let time = Time::view(window, cx);
        let result = ResultView::view(window, cx);

        Self {
            two_number,
            result,
            time,
            liu_yao,
        }
    }

    /**
     * 标题
     */
    pub fn title(&self) -> impl IntoElement {
        div()
            .child(div().child("开启卜卦之旅").text_2xl())
            .child(div().child("选择您喜欢的起卦方式，探索易经的奥秘"))
    }
}

impl Render for QiGua {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p_2()
            .gap_2()
            .child(self.title())
            .child(
                h_flex()
                    .gap_2()
                    .child(self.two_number.clone())
                    .child(self.time.clone())
                    .child(self.liu_yao.clone()),
            )
            .child(self.result.clone())
    }
}

impl Stage for QiGua {
    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn get_id() -> StageItem {
        StageItem::QiGua
    }
}
