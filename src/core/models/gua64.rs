use crate::core::{models::Gua8YaoIndex, utils::ichang_mod};

use super::{Gua8, Yao};
use gpui::SharedString;

/// 64 卦爻的顺序
///
/// 注意爻的顺序是从下往上
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gua64YaoIndex {
    /// 初爻（一爻）
    First = 1,
    /// 二爻
    Second,
    /// 三爻
    Third,
    /// 四爻
    Fourth,
    /// 五爻
    Fifth,
    /// 上爻（六爻）
    Sixth,
}

impl From<u16> for Gua64YaoIndex {
    fn from(value: u16) -> Self {
        let bian_index = ichang_mod(value, 6);

        match bian_index {
            1 => Gua64YaoIndex::First,
            2 => Gua64YaoIndex::Second,
            3 => Gua64YaoIndex::Third,
            4 => Gua64YaoIndex::Fourth,
            5 => Gua64YaoIndex::Fifth,
            6 => Gua64YaoIndex::Sixth,
            _ => unreachable!(),
        }
    }
}

/**
 * 六爻卦
 * 由 三爻卦 组合而成，共 64 种
 */
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gua64 {
    /// 上卦
    shang: Gua8,
    /// 下卦
    xia: Gua8,
}

impl Gua64 {
    /// 乾
    pub const 乾: Gua64 = Gua64::new(Gua8::乾, Gua8::乾);
    /// 坤
    pub const 坤: Gua64 = Gua64::new(Gua8::坤, Gua8::坤);
    /// 屯
    pub const 屯: Gua64 = Gua64::new(Gua8::坎, Gua8::震);

    /// 蒙
    pub const 蒙: Gua64 = Gua64::new(Gua8::艮, Gua8::坎);

    /// 需
    pub const 需: Gua64 = Gua64::new(Gua8::坎, Gua8::乾);

    /// 讼
    pub const 讼: Gua64 = Gua64::new(Gua8::乾, Gua8::坎);

    /// 师
    pub const 师: Gua64 = Gua64::new(Gua8::坤, Gua8::坎);

    /// 比
    pub const 比: Gua64 = Gua64::new(Gua8::坎, Gua8::坤);

    /// 小畜
    pub const 小畜: Gua64 = Gua64::new(Gua8::巽, Gua8::乾);

    /// 履
    pub const 履: Gua64 = Gua64::new(Gua8::乾, Gua8::兑);

    /// 泰
    pub const 泰: Gua64 = Gua64::new(Gua8::坤, Gua8::乾);

    /// 否
    pub const 否: Gua64 = Gua64::new(Gua8::乾, Gua8::坤);

    /// 同人
    pub const 同人: Gua64 = Gua64::new(Gua8::乾, Gua8::离);

    /// 大有
    pub const 大有: Gua64 = Gua64::new(Gua8::离, Gua8::乾);

    /// 谦
    pub const 谦: Gua64 = Gua64::new(Gua8::坤, Gua8::艮);

    /// 豫
    pub const 豫: Gua64 = Gua64::new(Gua8::震, Gua8::坤);

    /// 随
    pub const 随: Gua64 = Gua64::new(Gua8::兑, Gua8::震);

    /// 蛊
    pub const 蛊: Gua64 = Gua64::new(Gua8::艮, Gua8::巽);

    /// 临
    pub const 临: Gua64 = Gua64::new(Gua8::坤, Gua8::兑);

    /// 观
    pub const 观: Gua64 = Gua64::new(Gua8::巽, Gua8::坤);

    /// 噬嗑
    pub const 噬嗑: Gua64 = Gua64::new(Gua8::离, Gua8::震);

    /// 贲
    pub const 贲: Gua64 = Gua64::new(Gua8::艮, Gua8::离);

    /// 剥
    pub const 剥: Gua64 = Gua64::new(Gua8::艮, Gua8::坤);

    /// 复
    pub const 复: Gua64 = Gua64::new(Gua8::坤, Gua8::震);

    /// 无妄
    pub const 无妄: Gua64 = Gua64::new(Gua8::乾, Gua8::震);

    /// 大畜
    pub const 大畜: Gua64 = Gua64::new(Gua8::艮, Gua8::乾);

    /// 颐
    pub const 颐: Gua64 = Gua64::new(Gua8::艮, Gua8::震);

    /// 大过
    pub const 大过: Gua64 = Gua64::new(Gua8::兑, Gua8::巽);

    /// 坎
    pub const 坎: Gua64 = Gua64::new(Gua8::坎, Gua8::坎);

    /// 离
    pub const 离: Gua64 = Gua64::new(Gua8::离, Gua8::离);

    /// 咸
    pub const 咸: Gua64 = Gua64::new(Gua8::兑, Gua8::艮);

    /// 恒
    pub const 恒: Gua64 = Gua64::new(Gua8::震, Gua8::巽);

    /// 遯
    pub const 遯: Gua64 = Gua64::new(Gua8::乾, Gua8::艮);

    /// 大壮
    pub const 大壮: Gua64 = Gua64::new(Gua8::震, Gua8::乾);

    /// 晋
    pub const 晋: Gua64 = Gua64::new(Gua8::离, Gua8::坤);

    /// 明夷
    pub const 明夷: Gua64 = Gua64::new(Gua8::坤, Gua8::离);

    /// 家人
    pub const 家人: Gua64 = Gua64::new(Gua8::巽, Gua8::离);

    /// 睽
    pub const 睽: Gua64 = Gua64::new(Gua8::离, Gua8::兑);

    /// 蹇
    pub const 蹇: Gua64 = Gua64::new(Gua8::坎, Gua8::艮);

    /// 解
    pub const 解: Gua64 = Gua64::new(Gua8::震, Gua8::坎);

    /// 损
    pub const 损: Gua64 = Gua64::new(Gua8::艮, Gua8::兑);

    /// 益
    pub const 益: Gua64 = Gua64::new(Gua8::巽, Gua8::震);

    /// 夬
    pub const 夬: Gua64 = Gua64::new(Gua8::兑, Gua8::乾);

    /// 姤
    pub const 姤: Gua64 = Gua64::new(Gua8::乾, Gua8::巽);

    /// 萃
    pub const 萃: Gua64 = Gua64::new(Gua8::兑, Gua8::坤);

    /// 升
    pub const 升: Gua64 = Gua64::new(Gua8::坤, Gua8::巽);

    /// 困
    pub const 困: Gua64 = Gua64::new(Gua8::兑, Gua8::坎);

    /// 井
    pub const 井: Gua64 = Gua64::new(Gua8::坎, Gua8::巽);

    /// 革
    pub const 革: Gua64 = Gua64::new(Gua8::兑, Gua8::离);

    /// 鼎
    pub const 鼎: Gua64 = Gua64::new(Gua8::离, Gua8::巽);

    /// 震
    pub const 震: Gua64 = Gua64::new(Gua8::震, Gua8::震);

    /// 艮
    pub const 艮: Gua64 = Gua64::new(Gua8::艮, Gua8::艮);

    /// 渐
    pub const 渐: Gua64 = Gua64::new(Gua8::巽, Gua8::艮);

    /// 归妹
    pub const 归妹: Gua64 = Gua64::new(Gua8::震, Gua8::兑);

    /// 丰
    pub const 丰: Gua64 = Gua64::new(Gua8::震, Gua8::离);

    /// 旅
    pub const 旅: Gua64 = Gua64::new(Gua8::离, Gua8::艮);

    /// 巽
    pub const 巽: Gua64 = Gua64::new(Gua8::巽, Gua8::巽);

    /// 兑
    pub const 兑: Gua64 = Gua64::new(Gua8::兑, Gua8::兑);

    /// 涣
    pub const 涣: Gua64 = Gua64::new(Gua8::巽, Gua8::坎);

    /// 节
    pub const 节: Gua64 = Gua64::new(Gua8::坎, Gua8::兑);

    /// 中孚
    pub const 中孚: Gua64 = Gua64::new(Gua8::巽, Gua8::兑);

    /// 小过
    pub const 小过: Gua64 = Gua64::new(Gua8::震, Gua8::艮);

    /// 既济
    pub const 既济: Gua64 = Gua64::new(Gua8::坎, Gua8::离);

    /// 未济
    pub const 未济: Gua64 = Gua64::new(Gua8::离, Gua8::坎);

    pub const fn new(shang: Gua8, xia: Gua8) -> Self {
        Self { shang, xia }
    }

    /// 根据索引获取爻
    /// 从下往上数，共六个
    pub const fn yao(&self, index: Gua64YaoIndex) -> Yao {
        match index {
            Gua64YaoIndex::First => self.xia.first_yao,
            Gua64YaoIndex::Second => self.xia.second_yao,
            Gua64YaoIndex::Third => self.xia.third_yao,
            Gua64YaoIndex::Fourth => self.shang.first_yao,
            Gua64YaoIndex::Fifth => self.shang.second_yao,
            Gua64YaoIndex::Sixth => self.shang.third_yao,
        }
    }

    /// 获取互卦
    ///
    /// # 注释
    /// * 三四五爻作为上卦
    /// * 二三四爻作为下卦
    pub const fn hu_gua(&self) -> Self {
        let shang = Gua8::new(
            self.yao(Gua64YaoIndex::Third),
            self.yao(Gua64YaoIndex::Fourth),
            self.yao(Gua64YaoIndex::Fifth),
        );

        let xia = Gua8::new(
            self.yao(Gua64YaoIndex::Second),
            self.yao(Gua64YaoIndex::Third),
            self.yao(Gua64YaoIndex::Fourth),
        );

        Self::new(shang, xia)
    }

    /// 获得卦象名称
    pub const fn name(&self) -> &'static str {
        match (self.shang, self.xia) {
            (Gua8::乾, Gua8::乾) => "乾",
            (Gua8::坤, Gua8::坤) => "坤",
            (Gua8::坎, Gua8::震) => "屯",
            (Gua8::艮, Gua8::坎) => "蒙",
            (Gua8::坎, Gua8::乾) => "需",
            (Gua8::乾, Gua8::坎) => "讼",
            (Gua8::坤, Gua8::坎) => "师",
            (Gua8::坎, Gua8::坤) => "比",
            (Gua8::巽, Gua8::乾) => "小畜",
            (Gua8::乾, Gua8::兑) => "履",
            (Gua8::坤, Gua8::乾) => "泰",
            (Gua8::乾, Gua8::坤) => "否",
            (Gua8::乾, Gua8::离) => "同人",
            (Gua8::离, Gua8::乾) => "大有",
            (Gua8::坤, Gua8::艮) => "谦",
            (Gua8::震, Gua8::坤) => "豫",
            (Gua8::兑, Gua8::震) => "随",
            (Gua8::艮, Gua8::巽) => "蛊",
            (Gua8::坤, Gua8::兑) => "临",
            (Gua8::巽, Gua8::坤) => "观",
            (Gua8::离, Gua8::震) => "噬嗑",
            (Gua8::艮, Gua8::离) => "贲",
            (Gua8::艮, Gua8::坤) => "剥",
            (Gua8::坤, Gua8::震) => "复",
            (Gua8::乾, Gua8::震) => "无妄",
            (Gua8::艮, Gua8::乾) => "大畜",
            (Gua8::艮, Gua8::震) => "颐",
            (Gua8::兑, Gua8::巽) => "大过",
            (Gua8::坎, Gua8::坎) => "坎",
            (Gua8::离, Gua8::离) => "离",
            (Gua8::兑, Gua8::艮) => "咸",
            (Gua8::震, Gua8::巽) => "恒",
            (Gua8::乾, Gua8::艮) => "遯",
            (Gua8::震, Gua8::乾) => "大壮",
            (Gua8::离, Gua8::坤) => "晋",
            (Gua8::坤, Gua8::离) => "明夷",
            (Gua8::巽, Gua8::离) => "家人",
            (Gua8::离, Gua8::兑) => "睽",
            (Gua8::坎, Gua8::艮) => "蹇",
            (Gua8::震, Gua8::坎) => "解",
            (Gua8::艮, Gua8::兑) => "损",
            (Gua8::巽, Gua8::震) => "益",
            (Gua8::兑, Gua8::乾) => "夬",
            (Gua8::乾, Gua8::巽) => "姤",
            (Gua8::兑, Gua8::坤) => "萃",
            (Gua8::坤, Gua8::巽) => "升",
            (Gua8::兑, Gua8::坎) => "困",
            (Gua8::坎, Gua8::巽) => "井",
            (Gua8::兑, Gua8::离) => "革",
            (Gua8::离, Gua8::巽) => "鼎",
            (Gua8::震, Gua8::震) => "震",
            (Gua8::艮, Gua8::艮) => "艮",
            (Gua8::巽, Gua8::艮) => "渐",
            (Gua8::震, Gua8::兑) => "归妹",
            (Gua8::震, Gua8::离) => "丰",
            (Gua8::离, Gua8::艮) => "旅",
            (Gua8::巽, Gua8::巽) => "巽",
            (Gua8::兑, Gua8::兑) => "兑",
            (Gua8::巽, Gua8::坎) => "涣",
            (Gua8::坎, Gua8::兑) => "节",
            (Gua8::巽, Gua8::兑) => "中孚",
            (Gua8::震, Gua8::艮) => "小过",
            (Gua8::坎, Gua8::离) => "既济",
            (Gua8::离, Gua8::坎) => "未济",
        }
    }

    /// 进行变卦
    ///
    /// * `index` - Gua64YaoIndex，要改变的爻
    pub fn change(&mut self, index: Gua64YaoIndex) {
        match index {
            Gua64YaoIndex::First => self.xia.reverse(Gua8YaoIndex::First),
            Gua64YaoIndex::Second => self.xia.reverse(Gua8YaoIndex::Second),
            Gua64YaoIndex::Third => self.xia.reverse(Gua8YaoIndex::Third),
            Gua64YaoIndex::Fourth => self.shang.reverse(Gua8YaoIndex::First),
            Gua64YaoIndex::Fifth => self.shang.reverse(Gua8YaoIndex::Second),
            Gua64YaoIndex::Sixth => self.shang.reverse(Gua8YaoIndex::Third),
        }
    }
}

pub struct Gua64Iterator {
    gua: Gua64,
    index: u8,
}

impl Iterator for Gua64Iterator {
    type Item = Yao;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(self.gua.yao(Gua64YaoIndex::First)),
            1 => Some(self.gua.yao(Gua64YaoIndex::Second)),
            2 => Some(self.gua.yao(Gua64YaoIndex::Third)),
            3 => Some(self.gua.yao(Gua64YaoIndex::Fourth)),
            4 => Some(self.gua.yao(Gua64YaoIndex::Fifth)),
            5 => Some(self.gua.yao(Gua64YaoIndex::Sixth)),
            _ => None,
        };

        if result.is_some() {
            self.index += 1;
        }

        result
    }
}

impl IntoIterator for Gua64 {
    type Item = Yao;
    type IntoIter = Gua64Iterator;

    fn into_iter(self) -> Self::IntoIter {
        Gua64Iterator {
            gua: self,
            index: 0,
        }
    }
}
