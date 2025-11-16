use gpui::{App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Window, div};

use crate::ui::update::Update;

/**
 * 标题栏
 */
pub struct AppHeader {
    update: Entity<Update>,
}

impl AppHeader {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let update = Update::view(window, cx);

        Self { update }
    }
}

impl Render for AppHeader {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.update.clone())
    }
}
