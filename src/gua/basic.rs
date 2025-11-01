use super::ba_gua::BaGua;
use super::yao::*;

/**
 * 三爻卦
 * 三个爻为一个卦，共有八个，即八卦
 */
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Gua8(pub Yao, pub Yao, pub Yao);

impl Gua8 {
    /**
     * 乾
     */
    const QIÁN: Gua8 = Gua8(Yao::YANG, Yao::YANG, Yao::YANG);
    /**
     * 兑
     */
    const DUÌ: Gua8 = Gua8(Yao::YIN, Yao::YANG, Yao::YANG);
    /**
     * 离
     */
    const LÍ: Gua8 = Gua8(Yao::YANG, Yao::YIN, Yao::YANG);
    /**
     * 震
     */
    const ZHÈN: Gua8 = Gua8(Yao::YIN, Yao::YIN, Yao::YANG);

    /**
     * 坤
     */
    const KŪN: Gua8 = Gua8(Yao::YIN, Yao::YIN, Yao::YIN);

    /**
     * 艮
     */
    const GÈN: Gua8 = Gua8(Yao::YANG, Yao::YIN, Yao::YIN);

    /**
     * 坎
     */
    const KǍN: Gua8 = Gua8(Yao::YIN, Yao::YANG, Yao::YIN);

    /**
     * 巽
     */
    const XÙN: Gua8 = Gua8(Yao::YANG, Yao::YANG, Yao::YIN);

    /**
     * 解析，返回 BaGua
     */
    pub fn name(&self) -> BaGua {
        let name = match (self.0, self.1, self.2) {
            (Yao::YANG, Yao::YANG, Yao::YANG) => BaGua::Qián,
            (Yao::YIN, Yao::YANG, Yao::YANG) => BaGua::Duì,
            (Yao::YANG, Yao::YIN, Yao::YANG) => BaGua::Lí,
            (Yao::YIN, Yao::YIN, Yao::YANG) => BaGua::Zhèn,

            (Yao::YIN, Yao::YIN, Yao::YIN) => BaGua::Kūn,

            (Yao::YANG, Yao::YIN, Yao::YIN) => BaGua::Gèn,
            (Yao::YIN, Yao::YANG, Yao::YIN) => BaGua::Kǎn,
            (Yao::YANG, Yao::YANG, Yao::YIN) => BaGua::Xùn,
        };

        name
    }
}

/**
 * 六爻卦
 * 由 三爻卦 组合而成，共 64 种
 */
struct Gua64(pub Gua8, pub Gua8);

impl Gua64 {
    /**
     * 乾
     */
    const QIÁN: Gua64 = Gua64(Gua8::QIÁN, Gua8::QIÁN);
    /**
     * 坤
     */
    const KŪN: Gua64 = Gua64(Gua8::KŪN, Gua8::KŪN);
    /**
     * 屯
     */
    const ZHŪN: Gua64 = Gua64(Gua8::KǍN, Gua8::ZHÈN);

    /**
     * 蒙
     */
    const MÉNG: Gua64 = Gua64(Gua8::GÈN, Gua8::KǍN);

    /**
     * 需
     */
    const XŪ: Gua64 = Gua64(Gua8::KǍN, Gua8::QIÁN);

    /**
     * 讼
     */
    const SÒNG: Gua64 = Gua64(Gua8::QIÁN, Gua8::KǍN);

    /**
     * 师
     */
    const SHĪ: Gua64 = Gua64(Gua8::KŪN, Gua8::KǍN);

    /**
     * 比
     */
    const BǏ: Gua64 = Gua64(Gua8::KǍN, Gua8::KŪN);

    /**
     * 小畜
     */
    const XIǍO_CHÙ: Gua64 = Gua64(Gua8::XÙN, Gua8::QIÁN);

    /**
     * 履
     */
    const LǙ: Gua64 = Gua64(Gua8::QIÁN, Gua8::DUÌ);

    /**
     * 泰
     */
    const TÀI: Gua64 = Gua64(Gua8::KŪN, Gua8::QIÁN);

    /**
     * 否
     */
    const PǏ: Gua64 = Gua64(Gua8::QIÁN, Gua8::KŪN);

    /**
     * 同人
     */
    const TÓNG_RÉN: Gua64 = Gua64(Gua8::QIÁN, Gua8::LÍ);

    /**
     * 大有
     */
    const DÀ_YǑU: Gua64 = Gua64(Gua8::LÍ, Gua8::QIÁN);

    /**
     * 谦
     */
    const QIĀN: Gua64 = Gua64(Gua8::KŪN, Gua8::GÈN);

    /**
     * 豫
     */
    const YÙ: Gua64 = Gua64(Gua8::ZHÈN, Gua8::KŪN);

    /**
     * 随
     */
    const SUÍ: Gua64 = Gua64(Gua8::DUÌ, Gua8::ZHÈN);

    /**
     * 蛊
     */
    const GǓ: Gua64 = Gua64(Gua8::GÈN, Gua8::XÙN);

    /**
     * 临
     */
    const LÍN: Gua64 = Gua64(Gua8::KŪN, Gua8::DUÌ);

    /**
     * 观
     */
    const GUĀN: Gua64 = Gua64(Gua8::XÙN, Gua8::KŪN);

    /**
     * 噬嗑
     */
    const SHÌ_HÉ: Gua64 = Gua64(Gua8::LÍ, Gua8::ZHÈN);

    /**
     * 贲
     */
    const BÌ: Gua64 = Gua64(Gua8::GÈN, Gua8::LÍ);

    /**
     * 剥
     */
    const BŌ: Gua64 = Gua64(Gua8::GÈN, Gua8::KŪN);

    /**
     * 复
     */
    const FÙ: Gua64 = Gua64(Gua8::KŪN, Gua8::ZHÈN);

    /**
     * 无妄
     */
    const WÚ_WÀNG: Gua64 = Gua64(Gua8::QIÁN, Gua8::ZHÈN);

    /**
     * 大畜
     */
    const DÀ_CHÙ: Gua64 = Gua64(Gua8::GÈN, Gua8::QIÁN);

    /**
     * 颐
     */
    const YÍ: Gua64 = Gua64(Gua8::GÈN, Gua8::ZHÈN);

    /**
     * 大过
     */
    const DÀ_GUÒ: Gua64 = Gua64(Gua8::DUÌ, Gua8::XÙN);

    /**
     * 坎
     */
    const KǍN: Gua64 = Gua64(Gua8::KǍN, Gua8::KǍN);

    /**
     * 离
     */
    const LÍ: Gua64 = Gua64(Gua8::LÍ, Gua8::LÍ);

    /**
     * 咸
     */
    const XIÁN: Gua64 = Gua64(Gua8::DUÌ, Gua8::GÈN);

    /**
     * 恒
     */
    const HÉNG: Gua64 = Gua64(Gua8::ZHÈN, Gua8::XÙN);

    /**
     * 遯
     */
    const DÙN: Gua64 = Gua64(Gua8::QIÁN, Gua8::GÈN);

    /**
     * 大壮
     */
    const DÀ_ZHUÀNG: Gua64 = Gua64(Gua8::ZHÈN, Gua8::QIÁN);

    /**
     * 晋
     */
    const JÌN: Gua64 = Gua64(Gua8::LÍ, Gua8::KŪN);

    /**
     * 明夷
     */
    const MÍNG_YÍ: Gua64 = Gua64(Gua8::KŪN, Gua8::LÍ);

    /**
     * 家人
     */
    const JIĀ_RÉN: Gua64 = Gua64(Gua8::XÙN, Gua8::LÍ);

    /**
     * 睽
     */
    const KUÍ: Gua64 = Gua64(Gua8::LÍ, Gua8::DUÌ);

    /**
     * 蹇
     */
    const JIǍN: Gua64 = Gua64(Gua8::KǍN, Gua8::GÈN);

    /**
     * 解
     */
    const XIÈ: Gua64 = Gua64(Gua8::ZHÈN, Gua8::KǍN);

    /**
     * 损
     */
    const SǓN: Gua64 = Gua64(Gua8::GÈN, Gua8::DUÌ);

    /**
     * 益
     */
    const YÌ: Gua64 = Gua64(Gua8::XÙN, Gua8::ZHÈN);

    /**
     * 夬
     */
    const GUÀI: Gua64 = Gua64(Gua8::DUÌ, Gua8::QIÁN);

    /**
     * 姤
     */
    const GÒU: Gua64 = Gua64(Gua8::QIÁN, Gua8::XÙN);

    /**
     * 萃
     */
    const CUÌ: Gua64 = Gua64(Gua8::DUÌ, Gua8::KŪN);

    /**
     * 升
     */
    const SHĒNG: Gua64 = Gua64(Gua8::KŪN, Gua8::XÙN);

    /**
     * 困
     */
    const KÙN: Gua64 = Gua64(Gua8::DUÌ, Gua8::KǍN);

    /**
     * 井
     */
    const JǏNG: Gua64 = Gua64(Gua8::KǍN, Gua8::XÙN);

    /**
     * 革
     */
    const GÉ: Gua64 = Gua64(Gua8::DUÌ, Gua8::LÍ);

    /**
     * 鼎
     */
    const DǏNG: Gua64 = Gua64(Gua8::LÍ, Gua8::XÙN);

    /**
     * 震
     */
    const ZHÈN: Gua64 = Gua64(Gua8::ZHÈN, Gua8::ZHÈN);

    /**
     * 艮
     */
    const GÈN: Gua64 = Gua64(Gua8::GÈN, Gua8::GÈN);

    /**
     * 渐
     */
    const JIÀN: Gua64 = Gua64(Gua8::XÙN, Gua8::GÈN);

    /**
     * 归妹
     */
    const GUĪ_MÈI: Gua64 = Gua64(Gua8::ZHÈN, Gua8::DUÌ);

    /**
     * 丰
     */
    const FĒNG: Gua64 = Gua64(Gua8::ZHÈN, Gua8::LÍ);

    /**
     * 旅
     */
    const LǙ_56: Gua64 = Gua64(Gua8::LÍ, Gua8::GÈN);

    /**
     * 巽
     */
    const XÙN: Gua64 = Gua64(Gua8::XÙN, Gua8::XÙN);

    /**
     * 兑
     */
    const DUÌ: Gua64 = Gua64(Gua8::DUÌ, Gua8::DUÌ);

    /**
     * 涣
     */
    const HUÀN: Gua64 = Gua64(Gua8::XÙN, Gua8::KǍN);

    /**
     * 节
     */
    const JIÉ: Gua64 = Gua64(Gua8::KǍN, Gua8::DUÌ);

    /**
     * 中孚
     */
    const ZHŌNG_FÚ: Gua64 = Gua64(Gua8::XÙN, Gua8::DUÌ);

    /**
     * 小过
     */
    const XIǍO_GUÒ: Gua64 = Gua64(Gua8::ZHÈN, Gua8::GÈN);

    /**
     * 既济
     */
    const JÌ_JÌ: Gua64 = Gua64(Gua8::KǍN, Gua8::LÍ);

    /**
     * 未济
     */
    const WÈI_JÌ: Gua64 = Gua64(Gua8::LÍ, Gua8::KǍN);
}
