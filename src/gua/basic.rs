use super::ba_gua::BaGua;
use super::yao::*;

/**
 * 卦由爻组成，三个爻为一个基础卦
 * 基础卦共有八个，即八卦
 */
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BasicGua(pub Yao, pub Yao, pub Yao);

impl BasicGua {
    /**
     * 解析，返回 BaGua
     */
    pub fn name(&self) -> BaGua {
        let name = match (self.0, self.1, self.2) {
            (Yao::YANG, Yao::YANG, Yao::YANG) => BaGua::Qian,
            (Yao::YIN, Yao::YANG, Yao::YANG) => BaGua::Dui,
            (Yao::YANG, Yao::YIN, Yao::YANG) => BaGua::Li,
            (Yao::YIN, Yao::YIN, Yao::YANG) => BaGua::Zhen,

            (Yao::YIN, Yao::YIN, Yao::YIN) => BaGua::Kun,

            (Yao::YANG, Yao::YIN, Yao::YIN) => BaGua::Gen,
            (Yao::YIN, Yao::YANG, Yao::YIN) => BaGua::Kan,
            (Yao::YANG, Yao::YANG, Yao::YIN) => BaGua::Xun,
        };

        name
    }
}

/**
 * 基础八卦
 */
const QIAN: BasicGua = BasicGua(Yao::YANG, Yao::YANG, Yao::YANG); // 乾
const DUI: BasicGua = BasicGua(Yao::YIN, Yao::YANG, Yao::YANG); // 兑
const LI: BasicGua = BasicGua(Yao::YANG, Yao::YIN, Yao::YANG); // 离
const ZHEN: BasicGua = BasicGua(Yao::YIN, Yao::YIN, Yao::YANG); // 震

const KUN: BasicGua = BasicGua(Yao::YIN, Yao::YIN, Yao::YIN); // 坤

const GEN: BasicGua = BasicGua(Yao::YANG, Yao::YIN, Yao::YIN); // 艮
const KAN: BasicGua = BasicGua(Yao::YIN, Yao::YANG, Yao::YIN); // 坎
const XUN: BasicGua = BasicGua(Yao::YANG, Yao::YANG, Yao::YIN); // 巽
