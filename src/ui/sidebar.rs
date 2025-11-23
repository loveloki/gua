use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Window, prelude::FluentBuilder};
use gpui_component::{Icon, IconName, Side, sidebar::*};

#[derive(PartialEq, Eq, Clone)]
pub enum StageItem {
    QiGua,
    History,
    Library,
    About,
}

/// 应用侧边栏
pub struct AppSideBar {
    collapsed: bool,
    pub active_stage: StageItem,
}

impl AppSideBar {
    pub fn view(window: &mut Window, cx: &mut App, active_stage: StageItem) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, active_stage))
    }

    fn new(_: &mut Window, _: &mut Context<Self>, active_stage: StageItem) -> Self {
        Self {
            collapsed: false,
            active_stage,
        }
    }
}

impl Render for AppSideBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new(Side::Left)
            .collapsed(self.collapsed)
            .collapsible(true)
            .children([
                SidebarGroup::new("常用").child(
                    SidebarMenu::new()
                        .child(
                            SidebarMenuItem::new("起卦")
                                .icon(Icon::empty().path("icons/pencil-line.svg"))
                                .active(self.active_stage == StageItem::QiGua)
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.active_stage = StageItem::QiGua;
                                })),
                        )
                        .child(
                            SidebarMenuItem::new("历史")
                                .icon(Icon::empty().path("icons/history.svg"))
                                .active(self.active_stage == StageItem::History)
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.active_stage = StageItem::History;
                                })),
                        )
                        .child(
                            SidebarMenuItem::new("资源")
                                .icon(Icon::empty().path("icons/library.svg"))
                                .active(self.active_stage == StageItem::Library)
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.active_stage = StageItem::Library;
                                })),
                        ),
                ),
                SidebarGroup::new("设置").child(
                    SidebarMenu::new().child(
                        SidebarMenuItem::new("关于")
                            .icon(Icon::empty().path("icons/info.svg"))
                            .active(self.active_stage == StageItem::About)
                            .on_click(cx.listener(|this, _, _, _| {
                                this.active_stage = StageItem::About;
                            })),
                    ),
                ),
            ])
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
