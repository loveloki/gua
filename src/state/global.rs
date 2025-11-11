use gpui::{App, Global};

use crate::{
    assets::init_gua64_info,
    gua::{ba_gua::GuaResult, basic::Gua64Info},
};

pub struct GlobalState {
    /**
     * 算卦结果
     */
    pub result: Option<GuaResult>,
    /**
     * 64卦信息
     */
    pub gua64_info_list: Vec<Gua64Info>,
}

impl Global for GlobalState {}

impl GlobalState {
    pub fn init(cx: &mut App) {
        let gua64_info_list = init_gua64_info();

        cx.set_global::<GlobalState>(GlobalState {
            result: None,
            gua64_info_list,
        });
    }

    pub fn state(cx: &mut App) -> &Self {
        cx.global::<GlobalState>()
    }

    pub fn state_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<GlobalState>()
    }
}
