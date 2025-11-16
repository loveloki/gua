use gpui::{Context, SharedString};

/**
 * 起卦需要实现的方法
 */
pub trait QiGuaCore: 'static + Sized {
    /**
     * 算卦名称
     */
    fn name() -> SharedString;
    /**
     * 进行算卦
     */
    fn calc_gua(&mut self, cx: &mut Context<Self>);
}
