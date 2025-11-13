use gpui::{App, Global};

pub struct AppState {}

impl Global for AppState {}

impl AppState {
    pub fn init(cx: &mut App) {
        cx.set_global::<AppState>(AppState {});
    }

    pub fn state(cx: &mut App) -> &Self {
        cx.global::<AppState>()
    }

    pub fn state_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<AppState>()
    }
}
