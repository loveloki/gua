use gpui::*;
use gpui_component::h_flex;

use crate::ui::{
    home::Stage,
    sidebar::StageItem,
    stage::{input_two_num::InputTwoNum, result::ResultView},
};

/**
 * 算卦和结果
 */
pub struct QiGua {
    input_two_num: Entity<InputTwoNum>,
    result: Entity<ResultView>,
}

impl QiGua {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_two_num = InputTwoNum::view(window, cx);
        let result = ResultView::view(window, cx);

        Self {
            input_two_num,
            result,
        }
    }
}

impl Render for QiGua {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .p_2()
            .gap_2()
            .child(self.input_two_num.clone())
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
