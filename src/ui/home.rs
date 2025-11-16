use gpui::{prelude::FluentBuilder, *};
use gpui_component::{h_flex, v_flex};

use crate::ui::{
    header::AppHeader,
    sidebar::{AppSideBar, StageItem},
    stage::{history::History, library::Library, qi_gua::QiGua},
};

pub struct HomeWindow {
    sidebar: Entity<AppSideBar>,
    header: Entity<AppHeader>,
    stages: Vec<Entity<StageContainer>>,
}

impl HomeWindow {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let stages = vec![
            StageContainer::view::<QiGua>(window, cx),
            StageContainer::view::<Library>(window, cx),
            StageContainer::view::<History>(window, cx),
        ];

        let sidebar = AppSideBar::view(window, cx, StageItem::QiGua);
        let header = AppHeader::view(window, cx);

        Self {
            sidebar,
            stages,
            header,
        }
    }
}

impl Render for HomeWindow {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_stage = self.sidebar.read(cx).active_stage.clone();

        let active_stages: Vec<_> = self
            .stages
            .iter()
            .filter_map(|stage| {
                let is_eq = stage.read(cx).id == Some(active_stage.clone());

                if is_eq { Some(stage.clone()) } else { None }
            })
            .collect();

        let active_stage = active_stages.get(0).unwrap();

        v_flex().h_full().child(self.header.clone()).child(
            h_flex()
                .flex_1()
                .child(div().h_full().child(self.sidebar.clone()))
                .child(
                    div()
                        .flex_1()
                        .h_full()
                        .child(active_stage.clone())
                        .into_any_element(),
                ),
        )
    }
}

pub trait Stage {
    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render>;
    fn get_id() -> StageItem;
}

struct StageContainer {
    pub id: Option<StageItem>,
    stage: Option<AnyView>,
}

impl StageContainer {
    pub fn view<S: Stage>(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let stage = S::new_view(window, cx);
        let id = S::get_id();

        cx.new(|cx| {
            let mut container = Self::new(window, cx);

            container.stage = Some(stage.into());
            container.id = Some(id.clone());

            container
        })
    }

    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            id: None,
            stage: None,
        }
    }
}

impl Render for StageContainer {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("stage-container")
            .size_full()
            .overflow_y_scroll()
            .when_some(self.stage.clone(), |this, stage| this.child(stage))
    }
}
