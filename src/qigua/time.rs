use chinese_lunisolar_calendar::{EarthlyBranch, LunisolarDate, SolarDate};
use chrono::{Datelike, Local, NaiveDateTime, Timelike};
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Window, div,
};
use gpui_component::{
    StyledExt,
    button::{Button, ButtonVariants},
    date_picker::{DatePicker, DatePickerEvent, DatePickerState},
};

use crate::{
    gua::ba_gua::{BaGuaCalculator, GuaResult},
    qigua::core::QiGuaCore,
    state::global::GlobalState,
};

const NAME: &str = "时间";

/**
 * 时间起卦
 */
pub struct Time {
    content: Entity<TimeContent>,
}

impl Time {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let content = TimeContent::view(window, cx);

        Self { content }
    }
}

impl Render for Time {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().p_2().child(self.content.clone())
    }
}

/**
 * 输入时间来计算卦象
 */
pub struct TimeContent {
    date_picker: Entity<DatePickerState>,
    select_time: NaiveDateTime,
    _subscriptions: Vec<Subscription>,
}

impl TimeContent {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let select_time = Local::now().naive_local();

        let date_picker = cx.new(|cx| {
            let mut picker = DatePickerState::new(window, cx);
            picker.set_date(select_time.date(), window, cx);

            picker
        });

        let _subscriptions = vec![cx.subscribe(&date_picker, |this, _, ev, _| match ev {
            DatePickerEvent::Change(date) => {
                if let Some(d) = date.start() {
                    let year = d.year();
                    let month = d.month();
                    let day = d.day();

                    // 更新选择的日期
                    this.select_time = this
                        .select_time
                        .with_year(year)
                        .and_then(|t| t.with_month(month))
                        .and_then(|t| t.with_day(day))
                        .unwrap();
                }
            }
        })];

        Self {
            date_picker,
            select_time,
            _subscriptions,
        }
    }

    fn data_picker_content(&self) -> impl IntoElement {
        DatePicker::new(&self.date_picker).number_of_months(1)
    }
}

impl Render for TimeContent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        let year = self.select_time.year();
        let month = self.select_time.month();
        let day = self.select_time.day();

        let shi_chen = hour_to_shi_che(self.select_time.hour());

        let lunisolar_date = LunisolarDate::from_solar_date(
            SolarDate::from_ymd(year as u16, month as u8, day as u8).unwrap(),
        )
        .unwrap();

        div()
            .w_80()
            .p_2()
            .v_flex()
            .gap_2()
            .child(NAME)
            .child(self.data_picker_content())
            .child(format!("公历: {}-{}-{}", year, month, day))
            .child(format!("农历: {}", lunisolar_date))
            .child(format!("时辰: {}", shi_chen))
            .child(
                Button::new("calc")
                    .label("开始计算")
                    .primary()
                    .on_click(move |_, _, cx| {
                        cx.update_entity(
                            &entity,
                            |input: &mut TimeContent, context: &mut Context<TimeContent>| {
                                input.calc_gua(context)
                            },
                        );
                    }),
            )
    }
}

impl QiGuaCore for TimeContent {
    fn calc_gua(&mut self, cx: &mut Context<Self>) {
        let date = self.select_time;

        let year = date.year();
        let month = date.month();
        let day = date.day();

        let lunisolar_date = LunisolarDate::from_solar_date(
            SolarDate::from_ymd(year as u16, month as u8, day as u8).unwrap(),
        )
        .unwrap();

        let shi_chen = hour_to_shi_che(self.select_time.hour());
        let ba_gua_result = time_to_gua(lunisolar_date, shi_chen);

        let gua_result = GlobalState::state_mut(cx);
        gua_result.result = Some(ba_gua_result.clone());

        cx.notify();
    }
    fn name() -> SharedString {
        NAME.into()
    }
}

/**
 * 根据时间计算卦象
 */
fn time_to_gua(lunisolar_date: LunisolarDate, shi_chen: EarthlyBranch) -> GuaResult {
    let day = lunisolar_date.to_lunar_day().to_u8();
    let month = lunisolar_date.to_lunar_month().to_u8_raw();
    let year = lunisolar_date.to_lunar_year().to_earthly_branch().ordinal();

    let num1 = (year + month + day) as u16;
    let num2 = num1 + shi_chen.ordinal() as u16;

    BaGuaCalculator::calculate_from_two_numbers(num1, num2)
}

/**
 * 根据小时获取时辰
 */
fn hour_to_shi_che(hour: u32) -> EarthlyBranch {
    match hour {
        23 | 0 => EarthlyBranch::First,     // 子时: 23:00-01:00
        1 | 2 => EarthlyBranch::Second,     // 丑时: 01:00-03:00
        3 | 4 => EarthlyBranch::Third,      // 寅时: 03:00-05:00
        5 | 6 => EarthlyBranch::Fourth,     // 卯时: 05:00-07:00
        7 | 8 => EarthlyBranch::Fifth,      // 辰时: 07:00-09:00
        9 | 10 => EarthlyBranch::Sixth,     // 巳时: 09:00-11:00
        11 | 12 => EarthlyBranch::Seventh,  // 午时: 11:00-13:00
        13 | 14 => EarthlyBranch::Eighth,   // 未时: 13:00-15:00
        15 | 16 => EarthlyBranch::Ninth,    // 申时: 15:00-17:00
        17 | 18 => EarthlyBranch::Tenth,    // 酉时: 17:00-19:00
        19 | 20 => EarthlyBranch::Eleventh, // 戌时: 19:00-21:00
        21 | 22 => EarthlyBranch::Twelfth,  // 亥时: 21:00-23:00
        _ => unreachable!("hour must be 0-23"),
    }
}
