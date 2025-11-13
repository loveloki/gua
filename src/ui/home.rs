use gpui::{prelude::FluentBuilder, *};
use gpui_component::{h_flex, v_flex};

use crate::ui::{
    panel::story::Story,
    sidebar::{AppSideBar, PanelItem},
};

use super::tabs::Tabs;

pub struct HomeWindow {
    sidebar: Entity<AppSideBar>,
    panels: Vec<Entity<PanelContainer>>,
    active_panel: PanelItem,
}

impl HomeWindow {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let panels = vec![
            PanelContainer::view::<Tabs>(window, cx),
            PanelContainer::view::<Story>(window, cx),
        ];

        let active_panel = PanelItem::QiGua;
        let sidebar = AppSideBar::view(window, cx, active_panel.clone()).set_on_active_change(
            Box::new(cx.listener(|this, item: PanelItem, _, _| {
                this.active_panel = item;
            })),
        );

        Self {
            sidebar,
            panels,
            active_panel,
        }
    }
}

impl Render for HomeWindow {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_panels: Vec<_> = self
            .panels
            .iter()
            .filter_map(|panel| {
                let is_eq = panel.read(cx).id == Some(self.active_panel.clone());

                if is_eq { Some(panel.clone()) } else { None }
            })
            .collect();

        let active_panel = active_panels.get(0).unwrap();

        v_flex().h_full().child(
            h_flex()
                .flex_1()
                .child(div().h_full().child(self.sidebar.clone()))
                .child(
                    div()
                        .flex_1()
                        .h_full()
                        .child(active_panel.clone())
                        .into_any_element(),
                ),
        )
    }
}

pub trait AppPanel {
    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render>;
    fn get_id() -> PanelItem;
}

struct PanelContainer {
    pub id: Option<PanelItem>,
    panel: Option<AnyView>,
}

impl PanelContainer {
    pub fn view<P: AppPanel>(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let panel = P::new_view(window, cx);
        let id = panel.get_id();

        cx.new(|cx| {
            let mut container = Self::new(window, cx);

            container.panel = Some(panel.into());
            container.id = Some(id.clone());

            container
        })
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            id: None,
            panel: None,
        }
    }
}

impl Render for PanelContainer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("panel-container")
            .size_full()
            .overflow_y_scroll()
            .when_some(self.panel.clone(), |this, panel| this.child(panel))
    }
}
