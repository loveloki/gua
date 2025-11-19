use std::fmt::Debug;

use gpui::{Context, IntoElement, ParentElement, Render, SharedString, Window, div};
use time::{OffsetDateTime, macros::format_description};

use crate::gua::basic::{Gua8, Gua64, ichang_mod};

/// 卦象计算器
#[derive(Debug)]
pub struct BaGuaCalculator;

impl BaGuaCalculator {
    /**
     * 根据两个输入数字计算卦象
     */
    pub fn calculate_from_two_numbers(num1: u16, num2: u16) -> GuaResult {
        // 1. 将 num1 取余数
        let shang_num = ichang_mod(num1, 8);
        let xia_num = ichang_mod(num2, 8);

        // 本卦
        let ben_gua = Gua64::new(Gua8::from_num(shang_num), Gua8::from_num(xia_num));

        // 将两个数字相加取余
        // 余数即变化的位置
        let bian_index = ichang_mod(num1 + num2, 6);

        // 变卦
        let mut bian_gua = ben_gua.clone();
        bian_gua.change(bian_index);

        GuaResult::new(ben_gua, bian_gua)
    }
}

/**
 * 算卦结果
 */
#[derive(Clone)]
pub struct GuaResult {
    /**
     * 本卦
     */
    pub ben_gua: Gua64,
    /**
     * 变卦
     */
    pub bian_gua: Gua64,
    /**
     * 算卦时间
     */
    pub date: OffsetDateTime,
}

impl GuaResult {
    pub fn new(ben_gua: Gua64, bian_gua: Gua64) -> Self {
        let date = OffsetDateTime::now_local().unwrap();

        GuaResult {
            date,
            ben_gua,
            bian_gua,
        }
    }

    pub fn display(&self) -> SharedString {
        let ben_gua = self.ben_gua.display();
        let bian_gua = self.bian_gua.display();

        let custom_format = format_description!(
            "[year] 年 [month padding:zero] 月 [day padding:zero] 日 [hour padding:zero] 时 [minute padding:zero] 分"
        );
        let parsed_date = self.date.format(custom_format).unwrap();

        format!(
            "本卦：{}\n变卦：{}\n 算卦时间：{}",
            ben_gua, bian_gua, parsed_date
        )
        .into()
    }
}

impl Render for GuaResult {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child(self.display())
    }
}
