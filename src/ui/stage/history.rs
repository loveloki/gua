use gpui::*;

use crate::ui::{home::Stage, sidebar::StageItem};

/**
 * 历史记录
 */
pub struct History {}

impl History {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for History {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child("暂未实现")
    }
}

impl Stage for History {
    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn get_id() -> StageItem {
        StageItem::History
    }
}
