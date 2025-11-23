use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Window, div,
};
use gpui_component::{
    Disableable, StyledExt,
    button::{Button, ButtonVariants},
    h_flex,
    label::Label,
    select::{Select, SelectEvent, SelectState},
    v_flex,
};

use crate::qigua::core::QiGuaCore;

const NAME: &str = "六爻";

/// 六爻起卦
pub struct LiuYao {
    content: Entity<LiuYaoContent>,
}

impl LiuYao {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let content = LiuYaoContent::view(window, cx);

        Self { content }
    }
}

impl Render for LiuYao {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().p_2().child(self.content.clone())
    }
}

/// 输入六爻来计算卦象
pub struct LiuYaoContent {
    /// 上爻（六爻）
    sixth_yao: Entity<SingalYaoSelect>,
    /// 五爻
    fifth_yao: Entity<SingalYaoSelect>,
    /// 四爻
    fourth_yao: Entity<SingalYaoSelect>,
    /// 三爻
    third_yao: Entity<SingalYaoSelect>,
    /// 二爻
    second_yao: Entity<SingalYaoSelect>,
    /// 初爻（一爻）
    first_yao: Entity<SingalYaoSelect>,
}

impl LiuYaoContent {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            sixth_yao: SingalYaoSelect::view(window, cx, SharedString::new_static("上爻")),
            fifth_yao: SingalYaoSelect::view(window, cx, SharedString::new_static("五爻")),
            fourth_yao: SingalYaoSelect::view(window, cx, SharedString::new_static("四爻")),
            third_yao: SingalYaoSelect::view(window, cx, SharedString::new_static("三爻")),
            second_yao: SingalYaoSelect::view(window, cx, SharedString::new_static("二爻")),
            first_yao: SingalYaoSelect::view(window, cx, SharedString::new_static("初爻")),
        }
    }

    /// 选择爻象
    fn select_content(&mut self) -> impl IntoElement {
        v_flex().gap_2().children([
            self.sixth_yao.clone(),
            self.fifth_yao.clone(),
            self.fourth_yao.clone(),
            self.third_yao.clone(),
            self.second_yao.clone(),
            self.first_yao.clone(),
        ])
    }

    /// 是否可以执行计算
    fn is_can_calc(&self, cx: &mut Context<Self>) -> bool {
        self.sixth_yao.read(cx).is_selected
            && self.fifth_yao.read(cx).is_selected
            && self.fourth_yao.read(cx).is_selected
            && self.third_yao.read(cx).is_selected
            && self.second_yao.read(cx).is_selected
            && self.first_yao.read(cx).is_selected
    }
}

impl Render for LiuYaoContent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        div()
            .w_80()
            .p_2()
            .v_flex()
            .gap_2()
            .child(NAME)
            .child(self.select_content())
            .child(
                Button::new("calc")
                    .label("开始计算")
                    .primary()
                    .disabled(!self.is_can_calc(cx))
                    .on_click(move |_, _, cx| {
                        cx.update_entity(
                            &entity,
                            |input: &mut LiuYaoContent, context: &mut Context<LiuYaoContent>| {
                                input.calc_gua(context)
                            },
                        );
                    }),
            )
    }
}

impl QiGuaCore for LiuYaoContent {
    fn calc_gua(&mut self, cx: &mut Context<Self>) {}
    fn name() -> SharedString {
        NAME.into()
    }
}

/// 爻的类型
///
/// 用于六爻起卦，所以需要四种
enum LiuYaoType {
    /// 阴
    Yin,
    /// 阳
    Yang,
    /// 动阴
    DongYin,
    /// 动阳
    DongYang,
}

/// 单个爻的选择（以及随机生成）UI
struct SingalYaoSelect {
    /// 选择的爻
    select_state: Entity<SelectState<Vec<&'static str>>>,
    /// 是否选中了卦象
    is_selected: bool,
    /// 爻的名称前缀
    title_prefix: SharedString,
    _subscribe: Vec<Subscription>,
}

impl SingalYaoSelect {
    pub fn view(window: &mut Window, cx: &mut App, title_prefix: SharedString) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, title_prefix))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>, title_prefix: SharedString) -> Self {
        let select_state =
            cx.new(|cx| SelectState::new(vec!["阴", "阳", "动阴", "动阳"], None, window, cx));

        let _subscribe = vec![cx.subscribe(
            &select_state,
            |this, _, event: &SelectEvent<Vec<&'static str>>, _| match event {
                SelectEvent::Confirm(value) => match value {
                    None => this.is_selected = false,
                    Some(_) => this.is_selected = true,
                },
            },
        )];

        Self {
            select_state,
            is_selected: false,
            title_prefix,
            _subscribe,
        }
    }
}

impl Render for SingalYaoSelect {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .gap_1()
            .child(Label::new(self.title_prefix.clone()))
            .child(Select::new(&self.select_state).placeholder("选择爻象"))
    }
}
