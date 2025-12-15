use rand::random_bool;

/// 爻，卦的基础单位，分为阴和阳
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Yao {
    pub status: bool,
}

impl Yao {
    pub const 阴: Yao = Yao { status: false };
    pub const 阳: Yao = Yao { status: true };

    pub fn new(is_yang: bool) -> Self {
        Yao { status: is_yang }
    }

    pub fn is_yang(&self) -> bool {
        self.status
    }

    pub fn is_yin(&self) -> bool {
        !self.status
    }

    pub fn name(&self) -> &'static str {
        if self.status { "阳" } else { "阴" }
    }

    /// 翻转
    pub fn reverse(&mut self) {
        self.status = !self.status;
    }

    /// 获取随机爻
    pub fn random_yao() -> Self {
        match random_bool(1.0 / 2.0) {
            true => Yao::阳,
            false => Yao::阴,
        }
    }
}
