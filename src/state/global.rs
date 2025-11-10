use gpui::{App, Global};

use crate::gua::ba_gua::GuaResult;

pub struct GlobalState {
    /**
     * 算卦结果
     */
    pub result: Option<GuaResult>,
}

impl Global for GlobalState {}

impl GlobalState {
    pub fn init(cx: &mut App) {
        cx.set_global::<GlobalState>(GlobalState { result: None });
    }

    pub fn state(cx: &mut App) -> &Self {
        cx.global::<GlobalState>()
    }

    pub fn state_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<GlobalState>()
    }
}
