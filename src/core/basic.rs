use gpui::{Context, IntoElement, Render, Window};
use gpui_component::description_list::{DescriptionItem, DescriptionList};
use serde::{Deserialize, Serialize};

/// 64卦信息结构体
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
