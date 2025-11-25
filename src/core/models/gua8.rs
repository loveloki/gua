use super::Yao;
use gpui::SharedString;

/// 八卦爻的顺序
/// 注意爻的顺序是从下往上
pub enum Gua8YaoIndex {
    /// 初爻（一爻）
    First = 1,
    /// 二爻
    Second,
    /// 三爻
    Third,
}

/// 八卦
/// 三个爻为一个卦，共有八个，即八卦
///
/// # 注意
/// 爻的顺序是从下到上的，所以初爻是最下面的
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Gua8 {
    /// 初爻，即一爻
    pub first_yao: Yao,
    /// 二爻
    pub second_yao: Yao,
    /// 三爻
    pub third_yao: Yao,
}

impl Gua8 {
    /// 乾
    pub const 乾: Gua8 = Gua8::new(Yao::阳, Yao::阳, Yao::阳);
    /// 兑
    pub const 兑: Gua8 = Gua8::new(Yao::阳, Yao::阳, Yao::阴);
    /// 离
    pub const 离: Gua8 = Gua8::new(Yao::阳, Yao::阴, Yao::阳);
    /// 震
    pub const 震: Gua8 = Gua8::new(Yao::阳, Yao::阴, Yao::阴);

    /// 巽
    pub const 巽: Gua8 = Gua8::new(Yao::阴, Yao::阳, Yao::阳);

    /// 坎
    pub const 坎: Gua8 = Gua8::new(Yao::阴, Yao::阳, Yao::阴);

    /// 艮
    pub const 艮: Gua8 = Gua8::new(Yao::阴, Yao::阴, Yao::阳);

    /// 坤
    pub const 坤: Gua8 = Gua8::new(Yao::阴, Yao::阴, Yao::阴);

    /// 根据三个爻创建新的八卦
    ///
    /// # 注意
    /// 爻的顺序是从下到上，所以初爻在最下面
    pub const fn new(first_yao: Yao, second_yao: Yao, third_yao: Yao) -> Self {
        Self {
            third_yao,
            second_yao,
            first_yao,
        }
    }

    /// 解析，返回 BaGua
    pub fn name(&self) -> SharedString {
        let name = match (self.first_yao, self.second_yao, self.third_yao) {
            (Yao::阳, Yao::阳, Yao::阳) => SharedString::new("乾"),
            (Yao::阳, Yao::阳, Yao::阴) => SharedString::new("兑"),
            (Yao::阳, Yao::阴, Yao::阳) => SharedString::new("离"),
            (Yao::阳, Yao::阴, Yao::阴) => SharedString::new("震"),
            (Yao::阴, Yao::阳, Yao::阳) => SharedString::new("巽"),
            (Yao::阴, Yao::阳, Yao::阴) => SharedString::new("坎"),
            (Yao::阴, Yao::阴, Yao::阳) => SharedString::new("艮"),
            (Yao::阴, Yao::阴, Yao::阴) => SharedString::new("坤"),
        };

        name
    }

    /// 解析数字，返回 BaGua
    pub fn from_num(num: u8) -> Self {
        let result = match num {
            1 => Gua8::乾,
            2 => Gua8::兑,
            3 => Gua8::离,
            4 => Gua8::震,
            5 => Gua8::巽,
            6 => Gua8::坎,
            7 => Gua8::艮,
            8 => Gua8::坤,
            _ => !unreachable!(),
        };

        result
    }

    ///翻转指定位置的爻
    pub fn reverse(&mut self, index: Gua8YaoIndex) {
        match index {
            Gua8YaoIndex::First => self.first_yao.reverse(),
            Gua8YaoIndex::Second => self.second_yao.reverse(),
            Gua8YaoIndex::Third => self.third_yao.reverse(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::models::{Gua8, Gua8YaoIndex, Yao};

    #[test]
    /// 测试变卦功能
    fn test_gua8_change() {
        let mut gua = Gua8::new(Yao::阳, Yao::阳, Yao::阳);

        // 乾卦
        assert_eq!(gua, Gua8::乾);

        // 兑卦
        gua.reverse(Gua8YaoIndex::Third);
        assert_eq!(gua, Gua8::兑);

        // 震卦
        gua.reverse(Gua8YaoIndex::Second);
        assert_eq!(gua, Gua8::震);

        // 坤卦
        gua.reverse(Gua8YaoIndex::First);
        assert_eq!(gua, Gua8::坤);

        // 艮卦
        gua.reverse(Gua8YaoIndex::Third);
        assert_eq!(gua, Gua8::艮);

        // 巽卦
        gua.reverse(Gua8YaoIndex::Second);
        assert_eq!(gua, Gua8::巽);

        // 坎卦
        gua.reverse(Gua8YaoIndex::Third);
        assert_eq!(gua, Gua8::坎);

        // 离卦
        gua.reverse(Gua8YaoIndex::Third);
        gua.reverse(Gua8YaoIndex::Second);
        gua.reverse(Gua8YaoIndex::First);
        assert_eq!(gua, Gua8::离);
    }
}
