use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
    prelude::FluentBuilder,
};
use gpui_component::{
    Icon, IconName, Side,
    button::{Button, ButtonVariants},
    sidebar::*,
};

/**
 * 应用侧边栏
 */
pub struct AppSideBar {
    collapsed: bool,
}

impl AppSideBar {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self { collapsed: false }
    }
}

impl Render for AppSideBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new(Side::Left)
            .collapsed(self.collapsed)
            .collapsible(true)
            .header(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .w_full()
                    .child(
                        Button::new("close-app")
                            .rounded_full()
                            .danger()
                            .size_4()
                            .icon(IconName::Close)
                            .on_click(|_, _, cx| {
                                cx.quit();
                            })
                            .when(self.collapsed, |b| b.hidden()),
                    )
                    .child(
                        Button::new("sidebar-toggle")
                            .ghost()
                            .icon(Icon::new(IconName::PanelLeftClose))
                            .on_click(cx.listener(|this, _, _, _| {
                                this.collapsed = !this.collapsed;
                            }))
                            .when(self.collapsed, |b| {
                                b.icon(Icon::new(IconName::PanelLeftOpen))
                            }),
                    ),
            )
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
