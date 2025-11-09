use gpui::*;

use super::tabs::Tabs;

pub struct HomeWindow {
    tabs: Entity<Tabs>,
}

impl HomeWindow {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let tabs = Tabs::view(window, cx);

        Self { tabs }
    }
}

impl Render for HomeWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child(self.tabs.clone())
    }
}
