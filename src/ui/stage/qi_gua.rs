use gpui::*;
use gpui_component::h_flex;

use crate::{
    qigua::two_number::TwoNumber,
    ui::{home::Stage, sidebar::StageItem, stage::result::ResultView},
};

/**
 * 算卦和结果
 */
pub struct QiGua {
    two_number: Entity<TwoNumber>,
    result: Entity<ResultView>,
}

impl QiGua {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let two_number = TwoNumber::view(window, cx);
        let result = ResultView::view(window, cx);

        Self { two_number, result }
    }
}

impl Render for QiGua {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .p_2()
            .gap_2()
            .child(self.two_number.clone())
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
