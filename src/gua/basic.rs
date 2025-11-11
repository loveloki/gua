use gpui::SharedString;

use super::yao::*;

/**
 * 三爻卦
 * 三个爻为一个卦，共有八个，即八卦
 */
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Gua8 {
    pub yao1: Yao,
    pub yao2: Yao,
    pub yao3: Yao,
}

impl Gua8 {
    /**
     * 乾
     */
    const 乾: Gua8 = Gua8::new(Yao::阳, Yao::阳, Yao::阳);
    /**
     * 兑
     */
    const 兑: Gua8 = Gua8::new(Yao::阴, Yao::阳, Yao::阳);
    /**
     * 离
     */
    const 离: Gua8 = Gua8::new(Yao::阳, Yao::阴, Yao::阳);
    /**
     * 震
     */
    const 震: Gua8 = Gua8::new(Yao::阴, Yao::阴, Yao::阳);

    /**
     * 巽
     */
    const 巽: Gua8 = Gua8::new(Yao::阳, Yao::阳, Yao::阴);

    /**
     * 坎
     */
    const 坎: Gua8 = Gua8::new(Yao::阴, Yao::阳, Yao::阴);

    /**
     * 艮
     */
    const 艮: Gua8 = Gua8::new(Yao::阳, Yao::阴, Yao::阴);

    /**
     * 坤
     */
    const 坤: Gua8 = Gua8::new(Yao::阴, Yao::阴, Yao::阴);

    pub const fn new(yao1: Yao, yao2: Yao, yao3: Yao) -> Self {
        Self { yao1, yao2, yao3 }
    }

    /**
     * 解析，返回 BaGua
     */
    pub fn name(&self) -> SharedString {
        let name = match (self.yao1, self.yao2, self.yao3) {
            (Yao::阳, Yao::阳, Yao::阳) => SharedString::new("乾"),
            (Yao::阴, Yao::阳, Yao::阳) => SharedString::new("兑"),
            (Yao::阳, Yao::阴, Yao::阳) => SharedString::new("离"),
            (Yao::阴, Yao::阴, Yao::阳) => SharedString::new("震"),
            (Yao::阳, Yao::阳, Yao::阴) => SharedString::new("巽"),
            (Yao::阴, Yao::阳, Yao::阴) => SharedString::new("坎"),
            (Yao::阳, Yao::阴, Yao::阴) => SharedString::new("艮"),
            (Yao::阴, Yao::阴, Yao::阴) => SharedString::new("坤"),
        };

        name
    }

    /**
     * 解析数字，返回 BaGua
     * 便于取余计算，所以坤用的是 0 而不是 8
     */
    pub fn from_num(num: u8) -> Self {
        let result = match num {
            1 => Gua8::乾,
            2 => Gua8::兑,
            3 => Gua8::离,
            4 => Gua8::震,
            5 => Gua8::巽,
            6 => Gua8::坎,
            7 => Gua8::艮,
            0 => Gua8::坤,
            _ => !unreachable!(),
        };

        result
    }

    /**
     * 翻转指定位置的爻
     */
    pub fn reverse(&mut self, index: u8) {
        match index {
            0 => self.yao1.reverse(),
            1 => self.yao2.reverse(),
            2 => self.yao3.reverse(),
            _ => !unreachable!(),
        }
    }
}

/**
 * 六爻卦
 * 由 三爻卦 组合而成，共 64 种
 */
#[derive(Clone)]
pub struct Gua64 {
    shang: Gua8,
    xia: Gua8,
    name: SharedString,
}

impl Gua64 {
    /**
     * 乾
     */
    const 乾: Gua64 = Gua64::new(Gua8::乾, Gua8::乾);
    /**
     * 坤
     */
    const 坤: Gua64 = Gua64::new(Gua8::坤, Gua8::坤);
    /**
     * 屯
     */
    const 屯: Gua64 = Gua64::new(Gua8::坎, Gua8::震);

    /**
     * 蒙
     */
    const 蒙: Gua64 = Gua64::new(Gua8::艮, Gua8::坎);

    /**
     * 需
     */
    const 需: Gua64 = Gua64::new(Gua8::坎, Gua8::乾);

    /**
     * 讼
     */
    const 讼: Gua64 = Gua64::new(Gua8::乾, Gua8::坎);

    /**
     * 师
     */
    const 师: Gua64 = Gua64::new(Gua8::坤, Gua8::坎);

    /**
     * 比
     */
    const 比: Gua64 = Gua64::new(Gua8::坎, Gua8::坤);

    /**
     * 小畜
     */
    const 小畜: Gua64 = Gua64::new(Gua8::巽, Gua8::乾);

    /**
     * 履
     */
    const 履: Gua64 = Gua64::new(Gua8::乾, Gua8::兑);

    /**
     * 泰
     */
    const 泰: Gua64 = Gua64::new(Gua8::坤, Gua8::乾);

    /**
     * 否
     */
    const 否: Gua64 = Gua64::new(Gua8::乾, Gua8::坤);

    /**
     * 同人
     */
    const 同人: Gua64 = Gua64::new(Gua8::乾, Gua8::离);

    /**
     * 大有
     */
    const 大有: Gua64 = Gua64::new(Gua8::离, Gua8::乾);

    /**
     * 谦
     */
    const 谦: Gua64 = Gua64::new(Gua8::坤, Gua8::艮);

    /**
     * 豫
     */
    const 豫: Gua64 = Gua64::new(Gua8::震, Gua8::坤);

    /**
     * 随
     */
    const 随: Gua64 = Gua64::new(Gua8::兑, Gua8::震);

    /**
     * 蛊
     */
    const 蛊: Gua64 = Gua64::new(Gua8::艮, Gua8::巽);

    /**
     * 临
     */
    const 临: Gua64 = Gua64::new(Gua8::坤, Gua8::兑);

    /**
     * 观
     */
    const 观: Gua64 = Gua64::new(Gua8::巽, Gua8::坤);

    /**
     * 噬嗑
     */
    const 噬嗑: Gua64 = Gua64::new(Gua8::离, Gua8::震);

    /**
     * 贲
     */
    const 贲: Gua64 = Gua64::new(Gua8::艮, Gua8::离);

    /**
     * 剥
     */
    const 剥: Gua64 = Gua64::new(Gua8::艮, Gua8::坤);

    /**
     * 复
     */
    const 复: Gua64 = Gua64::new(Gua8::坤, Gua8::震);

    /**
     * 无妄
     */
    const 无妄: Gua64 = Gua64::new(Gua8::乾, Gua8::震);

    /**
     * 大畜
     */
    const 大畜: Gua64 = Gua64::new(Gua8::艮, Gua8::乾);

    /**
     * 颐
     */
    const 颐: Gua64 = Gua64::new(Gua8::艮, Gua8::震);

    /**
     * 大过
     */
    const 大过: Gua64 = Gua64::new(Gua8::兑, Gua8::巽);

    /**
     * 坎
     */
    const 坎: Gua64 = Gua64::new(Gua8::坎, Gua8::坎);

    /**
     * 离
     */
    const 离: Gua64 = Gua64::new(Gua8::离, Gua8::离);

    /**
     * 咸
     */
    const 咸: Gua64 = Gua64::new(Gua8::兑, Gua8::艮);

    /**
     * 恒
     */
    const 恒: Gua64 = Gua64::new(Gua8::震, Gua8::巽);

    /**
     * 遯
     */
    const 遯: Gua64 = Gua64::new(Gua8::乾, Gua8::艮);

    /**
     * 大壮
     */
    const 大壮: Gua64 = Gua64::new(Gua8::震, Gua8::乾);

    /**
     * 晋
     */
    const 晋: Gua64 = Gua64::new(Gua8::离, Gua8::坤);

    /**
     * 明夷
     */
    const 明夷: Gua64 = Gua64::new(Gua8::坤, Gua8::离);

    /**
     * 家人
     */
    const 家人: Gua64 = Gua64::new(Gua8::巽, Gua8::离);

    /**
     * 睽
     */
    const 睽: Gua64 = Gua64::new(Gua8::离, Gua8::兑);

    /**
     * 蹇
     */
    const 蹇: Gua64 = Gua64::new(Gua8::坎, Gua8::艮);

    /**
     * 解
     */
    const 解: Gua64 = Gua64::new(Gua8::震, Gua8::坎);

    /**
     * 损
     */
    const 损: Gua64 = Gua64::new(Gua8::艮, Gua8::兑);

    /**
     * 益
     */
    const 益: Gua64 = Gua64::new(Gua8::巽, Gua8::震);

    /**
     * 夬
     */
    const 夬: Gua64 = Gua64::new(Gua8::兑, Gua8::乾);

    /**
     * 姤
     */
    const 姤: Gua64 = Gua64::new(Gua8::乾, Gua8::巽);

    /**
     * 萃
     */
    const 萃: Gua64 = Gua64::new(Gua8::兑, Gua8::坤);

    /**
     * 升
     */
    const 升: Gua64 = Gua64::new(Gua8::坤, Gua8::巽);

    /**
     * 困
     */
    const 困: Gua64 = Gua64::new(Gua8::兑, Gua8::坎);

    /**
     * 井
     */
    const 井: Gua64 = Gua64::new(Gua8::坎, Gua8::巽);

    /**
     * 革
     */
    const 革: Gua64 = Gua64::new(Gua8::兑, Gua8::离);

    /**
     * 鼎
     */
    const 鼎: Gua64 = Gua64::new(Gua8::离, Gua8::巽);

    /**
     * 震
     */
    const 震: Gua64 = Gua64::new(Gua8::震, Gua8::震);

    /**
     * 艮
     */
    const 艮: Gua64 = Gua64::new(Gua8::艮, Gua8::艮);

    /**
     * 渐
     */
    const 渐: Gua64 = Gua64::new(Gua8::巽, Gua8::艮);

    /**
     * 归妹
     */
    const 归妹: Gua64 = Gua64::new(Gua8::震, Gua8::兑);

    /**
     * 丰
     */
    const 丰: Gua64 = Gua64::new(Gua8::震, Gua8::离);

    /**
     * 旅
     */
    const 旅: Gua64 = Gua64::new(Gua8::离, Gua8::艮);

    /**
     * 巽
     */
    const 巽: Gua64 = Gua64::new(Gua8::巽, Gua8::巽);

    /**
     * 兑
     */
    const 兑: Gua64 = Gua64::new(Gua8::兑, Gua8::兑);

    /**
     * 涣
     */
    const 涣: Gua64 = Gua64::new(Gua8::巽, Gua8::坎);

    /**
     * 节
     */
    const 节: Gua64 = Gua64::new(Gua8::坎, Gua8::兑);

    /**
     * 中孚
     */
    const 中孚: Gua64 = Gua64::new(Gua8::巽, Gua8::兑);

    /**
     * 小过
     */
    const 小过: Gua64 = Gua64::new(Gua8::震, Gua8::艮);

    /**
     * 既济
     */
    const 既济: Gua64 = Gua64::new(Gua8::坎, Gua8::离);

    /**
     * 未济
     */
    const 未济: Gua64 = Gua64::new(Gua8::离, Gua8::坎);

    pub const fn new(shang: Gua8, xia: Gua8) -> Self {
        let name = Self::name(shang, xia);

        Self { shang, xia, name }
    }

    /**
     * 根据上下卦获得卦象名称
     */
    pub const fn name(shang: Gua8, xia: Gua8) -> SharedString {
        match (shang, xia) {
            (Gua8::乾, Gua8::乾) => SharedString::new_static("乾"),
            (Gua8::坤, Gua8::坤) => SharedString::new_static("坤"),
            (Gua8::坎, Gua8::震) => SharedString::new_static("屯"),
            (Gua8::艮, Gua8::坎) => SharedString::new_static("蒙"),
            (Gua8::坎, Gua8::乾) => SharedString::new_static("需"),
            (Gua8::乾, Gua8::坎) => SharedString::new_static("讼"),
            (Gua8::坤, Gua8::坎) => SharedString::new_static("师"),
            (Gua8::坎, Gua8::坤) => SharedString::new_static("比"),
            (Gua8::巽, Gua8::乾) => SharedString::new_static("小畜"),
            (Gua8::乾, Gua8::兑) => SharedString::new_static("履"),
            (Gua8::坤, Gua8::乾) => SharedString::new_static("泰"),
            (Gua8::乾, Gua8::坤) => SharedString::new_static("否"),
            (Gua8::乾, Gua8::离) => SharedString::new_static("同人"),
            (Gua8::离, Gua8::乾) => SharedString::new_static("大有"),
            (Gua8::坤, Gua8::艮) => SharedString::new_static("谦"),
            (Gua8::震, Gua8::坤) => SharedString::new_static("豫"),
            (Gua8::兑, Gua8::震) => SharedString::new_static("随"),
            (Gua8::艮, Gua8::巽) => SharedString::new_static("蛊"),
            (Gua8::坤, Gua8::兑) => SharedString::new_static("临"),
            (Gua8::巽, Gua8::坤) => SharedString::new_static("观"),
            (Gua8::离, Gua8::震) => SharedString::new_static("噬嗑"),
            (Gua8::艮, Gua8::离) => SharedString::new_static("贲"),
            (Gua8::艮, Gua8::坤) => SharedString::new_static("剥"),
            (Gua8::坤, Gua8::震) => SharedString::new_static("复"),
            (Gua8::乾, Gua8::震) => SharedString::new_static("无妄"),
            (Gua8::艮, Gua8::乾) => SharedString::new_static("大畜"),
            (Gua8::艮, Gua8::震) => SharedString::new_static("颐"),
            (Gua8::兑, Gua8::巽) => SharedString::new_static("大过"),
            (Gua8::坎, Gua8::坎) => SharedString::new_static("坎"),
            (Gua8::离, Gua8::离) => SharedString::new_static("离"),
            (Gua8::兑, Gua8::艮) => SharedString::new_static("咸"),
            (Gua8::震, Gua8::巽) => SharedString::new_static("恒"),
            (Gua8::乾, Gua8::艮) => SharedString::new_static("遯"),
            (Gua8::震, Gua8::乾) => SharedString::new_static("大壮"),
            (Gua8::离, Gua8::坤) => SharedString::new_static("晋"),
            (Gua8::坤, Gua8::离) => SharedString::new_static("明夷"),
            (Gua8::巽, Gua8::离) => SharedString::new_static("家人"),
            (Gua8::离, Gua8::兑) => SharedString::new_static("睽"),
            (Gua8::坎, Gua8::艮) => SharedString::new_static("蹇"),
            (Gua8::震, Gua8::坎) => SharedString::new_static("解"),
            (Gua8::艮, Gua8::兑) => SharedString::new_static("损"),
            (Gua8::巽, Gua8::震) => SharedString::new_static("益"),
            (Gua8::兑, Gua8::乾) => SharedString::new_static("夬"),
            (Gua8::乾, Gua8::巽) => SharedString::new_static("姤"),
            (Gua8::兑, Gua8::坤) => SharedString::new_static("萃"),
            (Gua8::坤, Gua8::巽) => SharedString::new_static("升"),
            (Gua8::兑, Gua8::坎) => SharedString::new_static("困"),
            (Gua8::坎, Gua8::巽) => SharedString::new_static("井"),
            (Gua8::兑, Gua8::离) => SharedString::new_static("革"),
            (Gua8::离, Gua8::巽) => SharedString::new_static("鼎"),
            (Gua8::震, Gua8::震) => SharedString::new_static("震"),
            (Gua8::艮, Gua8::艮) => SharedString::new_static("艮"),
            (Gua8::巽, Gua8::艮) => SharedString::new_static("渐"),
            (Gua8::震, Gua8::兑) => SharedString::new_static("归妹"),
            (Gua8::震, Gua8::离) => SharedString::new_static("丰"),
            (Gua8::离, Gua8::艮) => SharedString::new_static("旅"),
            (Gua8::巽, Gua8::巽) => SharedString::new_static("巽"),
            (Gua8::兑, Gua8::兑) => SharedString::new_static("兑"),
            (Gua8::巽, Gua8::坎) => SharedString::new_static("涣"),
            (Gua8::坎, Gua8::兑) => SharedString::new_static("节"),
            (Gua8::巽, Gua8::兑) => SharedString::new_static("中孚"),
            (Gua8::震, Gua8::艮) => SharedString::new_static("小过"),
            (Gua8::坎, Gua8::离) => SharedString::new_static("既济"),
            (Gua8::离, Gua8::坎) => SharedString::new_static("未济"),
        }
    }

    pub fn display(&self) -> SharedString {
        let shang = self.shang.name();
        let xia = self.xia.name();
        let name = self.name.clone();

        let display = format!("{}（上卦：{}，下卦：{}）", name, shang, xia);

        SharedString::new(display)
    }

    /**
     * 进行变卦
     */
    pub fn change(&mut self, num: u8) {
        match num {
            0 | 1 | 2 => self.shang.reverse(num),
            _ => self.xia.reverse(num - 3),
        }

        self.name = Self::name(self.shang, self.xia);
    }
}

#[cfg(test)]
mod tests {
    use super::{Gua8, Gua64, Yao};

    #[test]
    /**
     * gua8 变卦功能测试
     */
    fn test_gua8_change() {
        let mut gua = Gua8::new(Yao::阳, Yao::阳, Yao::阳);

        // 乾卦
        assert_eq!(gua, Gua8::乾);

        // 兑卦
        gua.reverse(0);
        assert_eq!(gua, Gua8::兑);

        // 震卦
        gua.reverse(1);
        assert_eq!(gua, Gua8::震);

        // 坤卦
        gua.reverse(2);
        assert_eq!(gua, Gua8::坤);

        // 艮卦
        gua.reverse(0);
        assert_eq!(gua, Gua8::艮);

        // 巽卦
        gua.reverse(1);
        assert_eq!(gua, Gua8::巽);

        // 坎卦
        gua.reverse(0);
        assert_eq!(gua, Gua8::坎);

        // 离卦
        gua.reverse(0);
        gua.reverse(1);
        gua.reverse(2);
        assert_eq!(gua, Gua8::离);
    }
}
