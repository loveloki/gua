use gpui::*;
use gpui_component::{Icon, IconName, alert::Alert, text::TextView};

use crate::auto_update::{AutoUpdater, CURRENT_VERSION};

/**
 * 算卦和结果
 */
pub struct Update {}

impl Update {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {}
    }

    /**
     * 提示更新 UI
     */
    fn tip_content(
        &mut self,
        info: (SemanticVersion, String),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let (version, download_url) = info;
        let md_text = format!(
            "当前版本：：{CURRENT_VERSION}\n 发现新版本：{version}，[点击下载]({download_url})"
        );
        let message = TextView::markdown("update-markdown", md_text, window, cx);

        div().child(
            Alert::info("update-alert", message)
                .banner()
                .icon(Icon::new(IconName::Info)),
        )
    }
}

impl Render for Update {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let updater = AutoUpdater::get(cx)
            .map(|item| {
                let s = item.read(cx);

                s
            })
            .unwrap();

        let update_info = updater.update_info();

        match update_info {
            None => div(),
            Some(update_info) => self.tip_content(update_info, window, cx),
        }
    }
}
