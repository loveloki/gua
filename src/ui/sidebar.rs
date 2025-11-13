use crate::state::app_state::AppState;
use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Window};
use gpui_component::{Icon, Side, sidebar::*};

/**
 * 应用侧边栏
 */
pub struct AppSideBar;

impl AppSideBar {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for AppSideBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let collapsed = AppState::state(cx).sidebar_collapsed;

        Sidebar::new(Side::Left)
            .collapsed(collapsed)
            .collapsible(true)
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("起卦")
                            .icon(Icon::empty().path("icons/pencil-line.svg"))
                            .on_click(|_, _, _| println!("Dashboard clicked")),
                    )
                    .child(
                        SidebarMenuItem::new("历史")
                            .icon(Icon::empty().path("icons/history.svg"))
                            .on_click(|_, _, _| println!("Settings clicked")),
                    ),
            )
    }
}
