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
        // 计算步骤
        let mut steps: Vec<GuaResultStep> = vec![];

        // 1. 将 num1 取余数
        // 计算上卦和下卦
        let shang_num = ichang_mod(num1, 8);
        let xia_num = ichang_mod(num2, 8);

        steps.push(GuaResultStep {
            description: format!("计算上卦和下卦"),
            origin: format!("数字一：{num1} 数字2：{num2}"),
            result: format!("上卦为 {num1} % 8 = {shang_num}, 下卦： {num2} % 8 = {xia_num}"),
        });

        // 计算本卦
        let shang_gua = Gua8::from_num(shang_num);
        let xia_gua = Gua8::from_num(xia_num);
        let ben_gua = Gua64::new(shang_gua, xia_gua);

        steps.push(GuaResultStep {
            description: format!("计算本卦"),
            origin: format!(
                "上卦：{}({shang_num}) 下卦：{}({xia_num})",
                shang_gua.name(),
                xia_gua.name(),
            ),
            result: format!("本卦：{}", ben_gua.name()),
        });

        // 将两个数字相加取余
        // 余数即变化的位置
        let bian_index = ichang_mod(num1 + num2, 6);

        // 变卦
        let mut bian_gua = ben_gua.clone();
        bian_gua.change(bian_index);

        steps.push(GuaResultStep {
            description: format!("计算变卦"),
            origin: format!(
                "本卦：{} 变数：({num1} + {num2}) % 6 = {bian_index}",
                ben_gua.name(),
            ),
            result: format!("变卦：{}", bian_gua.name()),
        });

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

    /// 计算过程
    pub steps: Vec<GuaResultStep>,
}

#[derive(Clone)]
pub struct GuaResultStep {
    /// 原始值
    pub origin: String,
    /// 操作描述
    pub description: String,
    /// 结果
    pub result: String,
}

impl GuaResult {
    pub fn new(ben_gua: Gua64, bian_gua: Gua64) -> Self {
        let date = OffsetDateTime::now_local().unwrap();
        let steps = vec![];

        GuaResult {
            date,
            ben_gua,
            bian_gua,
            steps,
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

#[cfg(test)]
mod tests {
    use crate::gua::basic::{Gua8, Gua64};

    use super::BaGuaCalculator;

    #[test]
    /**
     * 测试 calculate_from_two_numbers
     */
    fn test_calculate_from_two_numbers() {
        let r1 = BaGuaCalculator::calculate_from_two_numbers(128, 33);
        assert_eq!(r1.ben_gua, Gua64::泰);
        assert_eq!(r1.bian_gua, Gua64::需);

        let r2 = BaGuaCalculator::calculate_from_two_numbers(63, 49);
        assert_eq!(r2.ben_gua, Gua64::大畜);
        assert_eq!(r2.bian_gua, Gua64::大有);
    }
}
