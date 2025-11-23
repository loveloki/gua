use crate::gua::basic::Gua64Info;
use anyhow::{Ok, anyhow};
use gpui::AssetSource;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("无法加载指定路径的资源： \"{path}\""))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

/// 初始化资源
pub fn init_gua64_info() -> Vec<Gua64Info> {
    let gua64_json_str = include_str!("../assets/gua64.json");

    let gua64_info: Vec<Gua64Info> = serde_json::from_str(gua64_json_str).unwrap();

    gua64_info
}
