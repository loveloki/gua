use gpui::*;

use crate::ui::input_two_num::InputTwoNum;

pub struct HomeWindow {
    input_two_num: Entity<InputTwoNum>,
}

impl HomeWindow {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_two_num = InputTwoNum::view(window, cx);

        Self { input_two_num }
    }
}

impl Render for HomeWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .child(self.input_two_num.clone())
    }
}
