use gpui::{App, Global};

pub struct AppState {
    /**
     * 侧边栏是否折叠
     */
    pub sidebar_collapsed: bool,
}

impl Global for AppState {}

impl AppState {
    pub fn init(cx: &mut App) {
        cx.set_global::<AppState>(AppState {
            sidebar_collapsed: false,
        });
    }

    pub fn state(cx: &mut App) -> &Self {
        cx.global::<AppState>()
    }

    pub fn state_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<AppState>()
    }
}
