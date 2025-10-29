use crate::SharedString;

#[derive(Debug, Clone)]
pub enum BaGua {
    Qian = 1,
    Dui = 2,
    Li = 3,
    Zhen = 4,
    Xun = 5,
    Kan = 6,
    Gen = 7,
    Kun = 0,
}

impl BaGua {
    pub fn name(&self) -> &'static str {
        match self {
            BaGua::Qian => "乾",
            BaGua::Dui => "兑",
            BaGua::Li => "离",
            BaGua::Zhen => "震",
            BaGua::Xun => "巽",
            BaGua::Kan => "坎",
            BaGua::Gen => "艮",
            BaGua::Kun => "坤",
        }
    }

    /// 根据数字计算八卦 (1-8 对应乾到坤)
    pub fn from_number(num: u8) -> Option<Self> {
        match num {
            1 => Some(BaGua::Qian),
            2 => Some(BaGua::Dui),
            3 => Some(BaGua::Li),
            4 => Some(BaGua::Zhen),
            5 => Some(BaGua::Xun),
            6 => Some(BaGua::Kan),
            7 => Some(BaGua::Gen),
            0 => Some(BaGua::Kun),
            _ => None,
        }
    }
}

pub struct BaGuaItem {
    one: Option<BaGua>,
    two: Option<BaGua>,
    three: Option<BaGua>,
}

impl BaGuaItem {
    pub fn display(&self) -> SharedString {
        let one = self.one.clone().unwrap().name();
        let two = self.two.clone().unwrap().name();
        let three = self.three.clone().unwrap().name();

        format!("结果为：上卦 {}, 下卦 {}，动爻 {}", one, two, three).into()
    }
}

/// 八卦计算器结构体
#[derive(Debug)]
pub struct BaGuaCalculator;

impl BaGuaCalculator {
    /// 根据两个输入数字计算八卦
    pub fn calculate_from_two_numbers(num1: u8, num2: u8) -> BaGuaItem {
        // 1. 将 num1 取余数
        let sum = num1 as u16 + num2 as u16;
        let result1 = num1 % 8;
        let result2 = num2 % 8;
        let result3 = (sum % 6) as u8;

        BaGuaItem {
            one: BaGua::from_number(result1),
            two: BaGua::from_number(result2),
            three: BaGua::from_number(result3),
        }
    }
}
