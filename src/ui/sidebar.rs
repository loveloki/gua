use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Window, prelude::FluentBuilder};
use gpui_component::{Icon, IconName, Side, sidebar::*};

#[derive(PartialEq, Eq, Clone)]
pub enum PanelItem {
    QiGua,
    History,
}

/**
 * 应用侧边栏
 */
pub struct AppSideBar {
    collapsed: bool,
    active_panel: PanelItem,
    on_active_change: Option<Box<dyn Fn(PanelItem)>>,
}

impl AppSideBar {
    pub fn view(window: &mut Window, cx: &mut App, active_panel: PanelItem) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, active_panel))
    }

    fn new(_: &mut Window, _: &mut Context<Self>, active_panel: PanelItem) -> Self {
        Self {
            collapsed: false,
            active_panel,
            on_active_change: None,
        }
    }

    pub fn set_on_active_change(mut self, f: Box<dyn Fn(PanelItem)>) -> Self {
        self.on_active_change = Some(f);
        self
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
                                .active(self.active_panel == PanelItem::QiGua)
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.active_panel = PanelItem::QiGua;

                                    if let Some(ref f) = this.on_active_change {
                                        f(PanelItem::QiGua);
                                    }
                                })),
                        )
                        .child(
                            SidebarMenuItem::new("历史")
                                .icon(Icon::empty().path("icons/history.svg"))
                                .active(self.active_panel == PanelItem::History)
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.active_panel = PanelItem::History;

                                    if let Some(ref f) = this.on_active_change {
                                        f(PanelItem::History);
                                    }
                                })),
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
