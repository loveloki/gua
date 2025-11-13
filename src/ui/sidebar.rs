use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Window, prelude::FluentBuilder};
use gpui_component::{Icon, IconName, Side, sidebar::*};

#[derive(PartialEq)]
enum Item {
    QiGua,
    History,
}

/**
 * 应用侧边栏
 */
pub struct AppSideBar {
    collapsed: bool,
    active_item: Item,
}

impl AppSideBar {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            collapsed: false,
            active_item: Item::QiGua,
        }
    }
}

impl Render for AppSideBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new(Side::Left)
            .collapsed(self.collapsed)
            .collapsible(true)
            .child(
                SidebarGroup::new("常用").child(
                    SidebarMenu::new()
                        .child(
                            SidebarMenuItem::new("起卦")
                                .icon(Icon::empty().path("icons/pencil-line.svg"))
                                .active(self.active_item == Item::QiGua)
                                .on_click(
                                    cx.listener(|this, _, _, _| this.active_item = Item::QiGua),
                                ),
                        )
                        .child(
                            SidebarMenuItem::new("历史")
                                .icon(Icon::empty().path("icons/history.svg"))
                                .active(self.active_item == Item::History)
                                .on_click(
                                    cx.listener(|this, _, _, _| this.active_item = Item::History),
                                ),
                        ),
                ),
            )
            .footer(
                SidebarMenuItem::new("折叠")
                    .icon(Icon::new(IconName::PanelLeftClose))
                    .on_click(cx.listener(|this, _, _, _| this.collapsed = !this.collapsed))
                    .when(self.collapsed, |b| {
                        b.icon(Icon::new(IconName::PanelLeftOpen))
                    }),
            )
    }
}
