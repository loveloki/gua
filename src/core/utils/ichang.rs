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
    use super::ichang_mod;

    #[test]
    /// 测试 ichang_mod 函数
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
