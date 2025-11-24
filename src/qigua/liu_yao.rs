use crate::{
    gua::{
        ba_gua::{GuaResult, GuaResultStep},
        basic::{Gua8, Gua64, Gua64YaoIndex},
        yao::Yao,
    },
    qigua::core::QiGuaCore,
    state::global::GlobalState,
};
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Window, div,
};
use gpui_component::{
    Disableable, Icon, StyledExt,
    button::{Button, ButtonVariants},
    h_flex,
    label::Label,
    select::{Select, SelectEvent, SelectState},
    v_flex,
};
use std::str::FromStr;
use strum::{Display, EnumString, IntoStaticStr};

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
    fn calc_gua(&mut self, cx: &mut Context<Self>) {
        let mut steps: Vec<GuaResultStep> = vec![];

        let first = self.first_yao.read(cx).yao.clone().unwrap();
        let second = self.second_yao.read(cx).yao.clone().unwrap();
        let third = self.third_yao.read(cx).yao.clone().unwrap();
        let fourth = self.fourth_yao.read(cx).yao.clone().unwrap();
        let fifth = self.fifth_yao.read(cx).yao.clone().unwrap();
        let sixth = self.sixth_yao.read(cx).yao.clone().unwrap();

        // 计算本卦
        let (ben_gua, steps2) = ben_gua(first, second, third, fourth, fifth, sixth);

        steps.append(&mut steps2.clone());

        // 计算变卦
        // 需要检查每一个爻，是否是动爻，对动幺进行翻转
        let bian_gua = bian_gua(&ben_gua, first, second, third, fourth, fifth, sixth);

        match bian_gua.clone() {
            None => {
                steps.push(GuaResultStep {
                    description: format!("计算变卦").into(),
                    origin: format!("本卦：{}", ben_gua.display()).into(),
                    result: format!("不存在动爻，没有变卦").into(),
                });
            }
            Some(bian_gua) => {
                steps.push(GuaResultStep {
                    description: format!("计算变卦").into(),
                    origin: format!("本卦：{}", ben_gua.display()).into(),
                    result: format!("变卦：{}", bian_gua.display()).into(),
                });
            }
        }

        // 计算互卦
        // 互卦
        let hu_gua = ben_gua.hu_gua();

        steps.push(GuaResultStep {
            description: format!("计算互卦").into(),
            origin: format!("本卦：{}", ben_gua.display(),).into(),
            result: format!("互卦：{}", hu_gua.display()).into(),
        });

        let mut ba_gua_result = GuaResult::new(ben_gua, bian_gua, hu_gua);
        ba_gua_result.steps = steps;

        let gua_result = GlobalState::state_mut(cx);
        gua_result.result = Some(ba_gua_result.clone());

        cx.notify();
    }

    fn name() -> SharedString {
        NAME.into()
    }
}

/// 爻的类型
///
/// 用于六爻起卦，所以需要四种
#[derive(Debug, PartialEq, EnumString, Display, Clone, Copy, IntoStaticStr)]
enum LiuYaoType {
    /// 阴
    #[strum(serialize = "阴")]
    阴,
    /// 阳
    #[strum(serialize = "阳")]
    阳,
    /// 动阴
    #[strum(serialize = "动阴")]
    动阴,
    /// 动阳
    #[strum(serialize = "动阳")]
    动阳,
}

impl LiuYaoType {
    /// 转换为 Yao
    ///
    /// * `阴` 和 `动阴` 转为 `阴`
    /// * `阳` 和 `动阳` 转为 `阳`
    pub const fn yao(self) -> Yao {
        match self {
            LiuYaoType::阴 | LiuYaoType::动阴 => Yao::阴,
            LiuYaoType::阳 | LiuYaoType::动阳 => Yao::阳,
        }
    }

    /// 是否是动幺
    pub fn is_dong(self) -> bool {
        match self {
            LiuYaoType::动阴 | LiuYaoType::动阳 => true,
            _ => false,
        }
    }

    /// 从三个 yao 转换
    ///
    /// # 注意
    /// * 哪种爻数量多，最终就是哪种爻
    /// * 三个爻一样，就是动爻
    pub fn from_three_yao(yao1: Yao, yao2: Yao, yao3: Yao) -> Self {
        match (yao1, yao2, yao3) {
            (Yao::阳, Yao::阳, Yao::阳) => LiuYaoType::动阳,
            (Yao::阴, Yao::阴, Yao::阴) => LiuYaoType::动阴,
            (Yao::阳, Yao::阴, Yao::阴) => LiuYaoType::阴,
            (Yao::阴, Yao::阳, Yao::阴) => LiuYaoType::阴,
            (Yao::阴, Yao::阴, Yao::阳) => LiuYaoType::阴,
            (Yao::阴, Yao::阳, Yao::阳) => LiuYaoType::阳,
            (Yao::阳, Yao::阴, Yao::阳) => LiuYaoType::阳,
            (Yao::阳, Yao::阳, Yao::阴) => LiuYaoType::阳,
        }
    }

    /// 随机生成 LiuYaoType
    pub fn random() -> Self {
        Self::from_three_yao(Yao::random_yao(), Yao::random_yao(), Yao::random_yao())
    }
}

/// 单个爻的选择（以及随机生成）UI
struct SingalYaoSelect {
    /// 选择的爻
    select_state: Entity<SelectState<Vec<&'static str>>>,
    /// 是否选中了卦象
    is_selected: bool,
    /// 爻的名称前缀
    title_prefix: SharedString,
    /// 爻的类型
    yao: Option<LiuYaoType>,
    /// 订阅
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
                    None => {
                        this.is_selected = false;
                        this.yao = None;
                    }
                    Some(s) => {
                        this.is_selected = true;
                        this.yao = Some(LiuYaoType::from_str(s).unwrap());
                    }
                },
            },
        )];

        Self {
            select_state,
            is_selected: false,
            title_prefix,
            yao: None,
            _subscribe,
        }
    }
}

impl Render for SingalYaoSelect {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .gap_1()
            .child(Label::new(self.title_prefix.clone()))
            .child(Select::new(&self.select_state).placeholder("选择爻象"))
            .child(
                Button::new("random-yao")
                    .icon(Icon::empty().path("icons/dices.svg"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        let random = LiuYaoType::random();
                        this.select_state.update(cx, |state, cx| {
                            state.set_selected_value(&random.into(), window, cx);

                            this.is_selected = true;
                            this.yao = Some(random);
                        });
                    })),
            )
    }
}

/// 获取变卦
///
/// 当和本卦相同的时候返回 None
fn bian_gua(
    ben_gua: &Gua64,
    first: LiuYaoType,
    second: LiuYaoType,
    third: LiuYaoType,
    fourth: LiuYaoType,
    fifth: LiuYaoType,
    sixth: LiuYaoType,
) -> Option<Gua64> {
    let mut bian_gua = ben_gua.clone();

    let is_dong = first.is_dong();
    if is_dong {
        bian_gua.change(Gua64YaoIndex::First);
    }

    let is_dong = second.is_dong();
    if is_dong {
        bian_gua.change(Gua64YaoIndex::Second);
    }

    let is_dong = third.is_dong();
    if is_dong {
        bian_gua.change(Gua64YaoIndex::Third);
    }

    let is_dong = fourth.is_dong();
    if is_dong {
        bian_gua.change(Gua64YaoIndex::Fourth);
    }

    let is_dong = fifth.is_dong();
    if is_dong {
        bian_gua.change(Gua64YaoIndex::Fifth);
    }

    let is_dong = sixth.is_dong();
    if is_dong {
        bian_gua.change(Gua64YaoIndex::Sixth);
    }

    match ben_gua.eq(&bian_gua) {
        true => None,
        false => Some(bian_gua),
    }
}

/// 获取本卦
fn ben_gua(
    first: LiuYaoType,
    second: LiuYaoType,
    third: LiuYaoType,
    fourth: LiuYaoType,
    fifth: LiuYaoType,
    sixth: LiuYaoType,
) -> (Gua64, Vec<GuaResultStep>) {
    let mut steps: Vec<GuaResultStep> = vec![];

    let shang = Gua8::new(
        LiuYaoType::yao(fourth),
        LiuYaoType::yao(fifth),
        LiuYaoType::yao(sixth),
    );
    let xia = Gua8::new(
        LiuYaoType::yao(first),
        LiuYaoType::yao(second),
        LiuYaoType::yao(third),
    );

    steps.push(GuaResultStep {
        description: format!("计算上卦和下卦").into(),
        origin: format!(
            "上爻：{sixth}，五爻：{fifth}，四爻：{fourth}， 三爻: {third}, 二爻: {second}, 初爻: {first}",
        )
        .into(),
        result: format!("上卦：{}，下卦：{}", shang.name(), xia.name()).into(),
    });

    let ben_gua = Gua64::new(shang, xia);

    steps.push(GuaResultStep {
        description: format!("计算本卦").into(),
        origin: format!("上卦：{}，下卦：{}", shang.name(), xia.name()).into(),
        result: format!("本卦：{}", ben_gua.name()).into(),
    });

    (ben_gua, steps)
}

mod tests {
    use super::*;

    #[test]
    /// 测试六爻算卦结果
    fn test_liu_yao() {
        let first = LiuYaoType::阳;
        let second = LiuYaoType::阴;
        let third = LiuYaoType::动阳;
        let fourth = LiuYaoType::动阴;
        let fifth = LiuYaoType::阳;
        let sixth = LiuYaoType::阳;

        // 计算本卦
        let (ben_gua_result, _) = ben_gua(first, second, third, fourth, fifth, sixth);

        // 计算变卦
        let bian_gua_result = bian_gua(&ben_gua_result, first, second, third, fourth, fifth, sixth);

        let hu_gua_result = ben_gua_result.hu_gua();

        assert_eq!(ben_gua_result, Gua64::家人);
        assert_eq!(bian_gua_result.unwrap(), Gua64::无妄);
        assert_eq!(hu_gua_result, Gua64::未济);

        // 测试没有变卦的结果
        let third = LiuYaoType::阳;
        let fourth = LiuYaoType::阴;

        // 计算本卦
        let (ben_gua_result, _) = ben_gua(first, second, third, fourth, fifth, sixth);

        // 计算变卦
        let bian_gua_result = bian_gua(&ben_gua_result, first, second, third, fourth, fifth, sixth);

        let hu_gua_result = ben_gua_result.hu_gua();

        assert_eq!(ben_gua_result, Gua64::家人);
        assert_eq!(bian_gua_result.is_none(), true);
        assert_eq!(hu_gua_result, Gua64::未济);
    }
}
