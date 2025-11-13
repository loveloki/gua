use gpui::*;
use gpui_component::{h_flex, v_flex};

use crate::ui::sidebar::AppSideBar;

use super::tabs::Tabs;

pub struct HomeWindow {
    tabs: Entity<Tabs>,
    sidebar: Entity<AppSideBar>,
}

impl HomeWindow {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let tabs = Tabs::view(window, cx);
        let sidebar = AppSideBar::view(window, cx);

        Self { tabs, sidebar }
    }
}

impl Render for HomeWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex().h_full().child(
            h_flex()
                .flex_1()
                .child(div().h_full().child(self.sidebar.clone()))
                .child(div().flex_1().h_full().child(self.tabs.clone())),
        )
    }
}
