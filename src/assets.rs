use crate::gua::basic::Gua64Info;

/**
 * 初始化资源
 */
pub fn init_gua64_info() -> Vec<Gua64Info> {
    let gua64_json_str = include_str!("../assets/gua64.json");

    let gua64_info: Vec<Gua64Info> = serde_json::from_str(gua64_json_str).unwrap();

    gua64_info
}
