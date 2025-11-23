use gpui::*;
use gpui_component::{StyledExt, button::Button, text::TextView};

use crate::{
    auto_update::{AutoUpdater, CURRENT_VERSION},
    ui::{home::Stage, sidebar::StageItem},
};

const ABOUT_MARKDOWN: &str = "项目主页： [gua](https://github.com/loveloki/gua)";

const README_MARKDOWN: &str = include_str!("../../../README.MD");

/// 历史记录
pub struct About {}

impl About {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {}
    }

    /// 检查更新按钮
    fn check_update(&self) -> impl IntoElement {
        div().child(
            Button::new("manual-update-btn")
                .label("检查更新")
                .on_click(|_, window, cx| {
                    AutoUpdater::check(window, cx);
                }),
        )
    }

    /// 当前版本信息
    fn current_version(&self) -> impl IntoElement {
        div().child(format!("当前版本：{CURRENT_VERSION}"))
    }

    /// 项目信息
    fn repo_info(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().child(TextView::markdown("repo-info", ABOUT_MARKDOWN, window, cx))
    }

    /// README内容
    fn readme_info(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        TextView::markdown("repo-md", README_MARKDOWN, window, cx)
    }
}

impl Render for About {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .scrollable(Axis::Vertical)
            .p_2()
            .child(self.check_update())
            .child(self.current_version())
            .child(self.repo_info(window, cx))
            .child(self.readme_info(window, cx))
    }
}

impl Stage for About {
    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn get_id() -> StageItem {
        StageItem::About
    }
}
