use gpui::{Context, IntoElement, Render, SharedString, Window};
use gpui_component::description_list::{DescriptionItem, DescriptionList};
use serde::{Deserialize, Serialize};

use super::yao::*;

/**
 * 64卦信息结构体
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gua64Info {
    pub id: String,
    pub name: String,
    pub gua_ci: String,
    pub tuan_ci: String,
    pub da_xiang: String,
    pub yao_ci: Vec<String>,
    pub xiao_xiang: Vec<String>,
    pub symbol: String,
}

impl Render for Gua64Info {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let id = self.id.clone();
        let name = self.name.clone();
        let gua_ci = self.gua_ci.clone();
        let tuan_ci = self.tuan_ci.clone();
        let da_xiang = self.da_xiang.clone();
        let yao_ci = self.yao_ci.clone();
        let xiao_xiang = self.xiao_xiang.clone();
        let symbol = self.symbol.clone();

        DescriptionList::new().columns(1).children([
            DescriptionItem::new("二进制").value(id).span(1),
            DescriptionItem::new("名称").value(name).span(1),
            DescriptionItem::new("卦辞").value(gua_ci).span(1),
            DescriptionItem::new("彖辞").value(tuan_ci).span(1),
            DescriptionItem::new("大象").value(da_xiang).span(1),
            DescriptionItem::new("爻辞")
                .value(yao_ci.join(", "))
                .span(1),
            DescriptionItem::new("小象")
                .value(xiao_xiang.join(", "))
                .span(1),
            DescriptionItem::new("符号").value(symbol).span(1),
        ])
    }
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
    /**
     * 乾
     */
    pub const 乾: Gua8 = Gua8::new(Yao::阳, Yao::阳, Yao::阳);
    /**
     * 兑
     */
    pub const 兑: Gua8 = Gua8::new(Yao::阳, Yao::阳, Yao::阴);
    /**
     * 离
     */
    pub const 离: Gua8 = Gua8::new(Yao::阳, Yao::阴, Yao::阳);
    /**
     * 震
     */
    pub const 震: Gua8 = Gua8::new(Yao::阳, Yao::阴, Yao::阴);

    /**
     * 巽
     */
    pub const 巽: Gua8 = Gua8::new(Yao::阴, Yao::阳, Yao::阳);

    /**
     * 坎
     */
    pub const 坎: Gua8 = Gua8::new(Yao::阴, Yao::阳, Yao::阴);

    /**
     * 艮
     */
    pub const 艮: Gua8 = Gua8::new(Yao::阴, Yao::阴, Yao::阳);

    /**
     * 坤
     */
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

    /**
     * 解析，返回 BaGua
     */
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

    /**
     * 解析数字，返回 BaGua
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

/// 64 卦爻的顺序
/// 注意爻的顺序是从下往上
enum Gua64_Yao_Index {
    /// 一爻
    First = 1,
    /// 二爻
    Second,
    /// 三爻
    Third,
    /// 四爻
    Fourth,
    /// 五爻
    Fifth,
    /// 六爻
    Sixth,
}

/**
 * 六爻卦
 * 由 三爻卦 组合而成，共 64 种
 */
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gua64 {
    /**
     * 上卦
     */
    shang: Gua8,
    /**
     * 下卦
     */
    xia: Gua8,
    /**
     * 名称
     */
    name: SharedString,
    // /**
    //  * 释义等信息
    //  */
    // info: GuaInfo,
}

impl Gua64 {
    /**
     * 乾
     */
    pub const 乾: Gua64 = Gua64::new(Gua8::乾, Gua8::乾);
    /**
     * 坤
     */
    pub const 坤: Gua64 = Gua64::new(Gua8::坤, Gua8::坤);
    /**
     * 屯
     */
    pub const 屯: Gua64 = Gua64::new(Gua8::坎, Gua8::震);

    /**
     * 蒙
     */
    pub const 蒙: Gua64 = Gua64::new(Gua8::艮, Gua8::坎);

    /**
     * 需
     */
    pub const 需: Gua64 = Gua64::new(Gua8::坎, Gua8::乾);

    /**
     * 讼
     */
    pub const 讼: Gua64 = Gua64::new(Gua8::乾, Gua8::坎);

    /**
     * 师
     */
    pub const 师: Gua64 = Gua64::new(Gua8::坤, Gua8::坎);

    /**
     * 比
     */
    pub const 比: Gua64 = Gua64::new(Gua8::坎, Gua8::坤);

    /**
     * 小畜
     */
    pub const 小畜: Gua64 = Gua64::new(Gua8::巽, Gua8::乾);

    /**
     * 履
     */
    pub const 履: Gua64 = Gua64::new(Gua8::乾, Gua8::兑);

    /**
     * 泰
     */
    pub const 泰: Gua64 = Gua64::new(Gua8::坤, Gua8::乾);

    /**
     * 否
     */
    pub const 否: Gua64 = Gua64::new(Gua8::乾, Gua8::坤);

    /**
     * 同人
     */
    pub const 同人: Gua64 = Gua64::new(Gua8::乾, Gua8::离);

    /**
     * 大有
     */
    pub const 大有: Gua64 = Gua64::new(Gua8::离, Gua8::乾);

    /**
     * 谦
     */
    pub const 谦: Gua64 = Gua64::new(Gua8::坤, Gua8::艮);

    /**
     * 豫
     */
    pub const 豫: Gua64 = Gua64::new(Gua8::震, Gua8::坤);

    /**
     * 随
     */
    pub const 随: Gua64 = Gua64::new(Gua8::兑, Gua8::震);

    /**
     * 蛊
     */
    pub const 蛊: Gua64 = Gua64::new(Gua8::艮, Gua8::巽);

    /**
     * 临
     */
    pub const 临: Gua64 = Gua64::new(Gua8::坤, Gua8::兑);

    /**
     * 观
     */
    pub const 观: Gua64 = Gua64::new(Gua8::巽, Gua8::坤);

    /**
     * 噬嗑
     */
    pub const 噬嗑: Gua64 = Gua64::new(Gua8::离, Gua8::震);

    /**
     * 贲
     */
    pub const 贲: Gua64 = Gua64::new(Gua8::艮, Gua8::离);

    /**
     * 剥
     */
    pub const 剥: Gua64 = Gua64::new(Gua8::艮, Gua8::坤);

    /**
     * 复
     */
    pub const 复: Gua64 = Gua64::new(Gua8::坤, Gua8::震);

    /**
     * 无妄
     */
    pub const 无妄: Gua64 = Gua64::new(Gua8::乾, Gua8::震);

    /**
     * 大畜
     */
    pub const 大畜: Gua64 = Gua64::new(Gua8::艮, Gua8::乾);

    /**
     * 颐
     */
    pub const 颐: Gua64 = Gua64::new(Gua8::艮, Gua8::震);

    /**
     * 大过
     */
    pub const 大过: Gua64 = Gua64::new(Gua8::兑, Gua8::巽);

    /**
     * 坎
     */
    pub const 坎: Gua64 = Gua64::new(Gua8::坎, Gua8::坎);

    /**
     * 离
     */
    pub const 离: Gua64 = Gua64::new(Gua8::离, Gua8::离);

    /**
     * 咸
     */
    pub const 咸: Gua64 = Gua64::new(Gua8::兑, Gua8::艮);

    /**
     * 恒
     */
    pub const 恒: Gua64 = Gua64::new(Gua8::震, Gua8::巽);

    /**
     * 遯
     */
    pub const 遯: Gua64 = Gua64::new(Gua8::乾, Gua8::艮);

    /**
     * 大壮
     */
    pub const 大壮: Gua64 = Gua64::new(Gua8::震, Gua8::乾);

    /**
     * 晋
     */
    pub const 晋: Gua64 = Gua64::new(Gua8::离, Gua8::坤);

    /**
     * 明夷
     */
    pub const 明夷: Gua64 = Gua64::new(Gua8::坤, Gua8::离);

    /**
     * 家人
     */
    pub const 家人: Gua64 = Gua64::new(Gua8::巽, Gua8::离);

    /**
     * 睽
     */
    pub const 睽: Gua64 = Gua64::new(Gua8::离, Gua8::兑);

    /**
     * 蹇
     */
    pub const 蹇: Gua64 = Gua64::new(Gua8::坎, Gua8::艮);

    /**
     * 解
     */
    pub const 解: Gua64 = Gua64::new(Gua8::震, Gua8::坎);

    /**
     * 损
     */
    pub const 损: Gua64 = Gua64::new(Gua8::艮, Gua8::兑);

    /**
     * 益
     */
    pub const 益: Gua64 = Gua64::new(Gua8::巽, Gua8::震);

    /**
     * 夬
     */
    pub const 夬: Gua64 = Gua64::new(Gua8::兑, Gua8::乾);

    /**
     * 姤
     */
    pub const 姤: Gua64 = Gua64::new(Gua8::乾, Gua8::巽);

    /**
     * 萃
     */
    pub const 萃: Gua64 = Gua64::new(Gua8::兑, Gua8::坤);

    /**
     * 升
     */
    pub const 升: Gua64 = Gua64::new(Gua8::坤, Gua8::巽);

    /**
     * 困
     */
    pub const 困: Gua64 = Gua64::new(Gua8::兑, Gua8::坎);

    /**
     * 井
     */
    pub const 井: Gua64 = Gua64::new(Gua8::坎, Gua8::巽);

    /**
     * 革
     */
    pub const 革: Gua64 = Gua64::new(Gua8::兑, Gua8::离);

    /**
     * 鼎
     */
    pub const 鼎: Gua64 = Gua64::new(Gua8::离, Gua8::巽);

    /**
     * 震
     */
    pub const 震: Gua64 = Gua64::new(Gua8::震, Gua8::震);

    /**
     * 艮
     */
    pub const 艮: Gua64 = Gua64::new(Gua8::艮, Gua8::艮);

    /**
     * 渐
     */
    pub const 渐: Gua64 = Gua64::new(Gua8::巽, Gua8::艮);

    /**
     * 归妹
     */
    pub const 归妹: Gua64 = Gua64::new(Gua8::震, Gua8::兑);

    /**
     * 丰
     */
    pub const 丰: Gua64 = Gua64::new(Gua8::震, Gua8::离);

    /**
     * 旅
     */
    pub const 旅: Gua64 = Gua64::new(Gua8::离, Gua8::艮);

    /**
     * 巽
     */
    pub const 巽: Gua64 = Gua64::new(Gua8::巽, Gua8::巽);

    /**
     * 兑
     */
    pub const 兑: Gua64 = Gua64::new(Gua8::兑, Gua8::兑);

    /**
     * 涣
     */
    pub const 涣: Gua64 = Gua64::new(Gua8::巽, Gua8::坎);

    /**
     * 节
     */
    pub const 节: Gua64 = Gua64::new(Gua8::坎, Gua8::兑);

    /**
     * 中孚
     */
    pub const 中孚: Gua64 = Gua64::new(Gua8::巽, Gua8::兑);

    /**
     * 小过
     */
    pub const 小过: Gua64 = Gua64::new(Gua8::震, Gua8::艮);

    /**
     * 既济
     */
    pub const 既济: Gua64 = Gua64::new(Gua8::坎, Gua8::离);

    /**
     * 未济
     */
    pub const 未济: Gua64 = Gua64::new(Gua8::离, Gua8::坎);

    pub const fn new(shang: Gua8, xia: Gua8) -> Self {
        let name = Self::parse_name(shang, xia);

        Self { shang, xia, name }
    }

    /// 根据索引获取爻
    /// 从下往上数，共六个
    pub const fn yao(&self, index: Gua64_Yao_Index) -> Yao {
        match index {
            Gua64_Yao_Index::First => self.xia.first_yao,
            Gua64_Yao_Index::Second => self.xia.second_yao,
            Gua64_Yao_Index::Third => self.xia.third_yao,
            Gua64_Yao_Index::Fourth => self.shang.first_yao,
            Gua64_Yao_Index::Fifth => self.shang.second_yao,
            Gua64_Yao_Index::Sixth => self.shang.third_yao,
        }
    }

    /// 获取互卦
    ///
    /// # 注释
    /// * 三四五爻作为上卦
    /// * 二三四爻作为下卦
    pub const fn hu_gua(&self) -> Self {
        let shang = Gua8::new(
            self.yao(Gua64_Yao_Index::Third),
            self.yao(Gua64_Yao_Index::Fourth),
            self.yao(Gua64_Yao_Index::Fifth),
        );

        let xia = Gua8::new(
            self.yao(Gua64_Yao_Index::Second),
            self.yao(Gua64_Yao_Index::Third),
            self.yao(Gua64_Yao_Index::Fourth),
        );

        Self::new(shang, xia)
    }

    /**
     * 根据上下卦获得卦象名称
     */
    pub const fn parse_name(shang: Gua8, xia: Gua8) -> SharedString {
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

    /// 获取卦名
    pub fn name(&self) -> SharedString {
        self.name.clone()
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
     * 传入的 num 应该为 1 到 6
     * 顺序为从下到上，即 1 是下卦的最后一个爻，6 是上卦的第一个爻
     */
    pub fn change(&mut self, num: u8) {
        match num {
            1 => self.xia.reverse(Gua8YaoIndex::First),
            2 => self.xia.reverse(Gua8YaoIndex::Second),
            3 => self.xia.reverse(Gua8YaoIndex::Third),
            4 => self.shang.reverse(Gua8YaoIndex::First),
            5 => self.shang.reverse(Gua8YaoIndex::Second),
            6 => self.shang.reverse(Gua8YaoIndex::Third),
            _ => !unreachable!("变卦的爻只能是 1 到 6"),
        }

        self.name = Self::parse_name(self.shang, self.xia);
    }
}

/// 梅花易数的“取余”逻辑
///
/// 遵循“除尽则为满”的原则：
/// - 如果能被整除（余数为 0），则结果为基数本身（base）。
/// - 否则，结果为余数。
///
/// # Arguments
///
/// * `number`: 被除数
/// * `base`: 基数
/// ```
pub fn ichang_mod(number: u16, base: u16) -> u8 {
    let a = number % base;

    match a {
        0 => base as u8,
        _ => a as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::{Gua8, Gua8YaoIndex, Yao, ichang_mod};

    #[test]
    /**
     * gua8 变卦功能测试
     */
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

    #[test]
    /**
     * 测试 ichang_mod 函数
     */
    fn test_ichang_mod() {
        assert_eq!(ichang_mod(0, 6), 6);
        assert_eq!(ichang_mod(1, 6), 1);
        assert_eq!(ichang_mod(2, 6), 2);
        assert_eq!(ichang_mod(3, 6), 3);
        assert_eq!(ichang_mod(4, 6), 4);
        assert_eq!(ichang_mod(5, 6), 5);
        assert_eq!(ichang_mod(6, 6), 6);
        assert_eq!(ichang_mod(7, 6), 1);
    }
}
