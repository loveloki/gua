use gpui::Context;

/**
 * 起卦需要实现的方法
 */
pub trait QiGuaCore: 'static + Sized {
    /**
     * 进行算卦
     */
    fn calc_gua(&mut self, cx: &mut Context<Self>);
}
