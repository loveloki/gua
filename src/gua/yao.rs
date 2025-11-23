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

    /// 翻转
    pub fn reverse(&mut self) {
        self.status = !self.status;
    }
}
