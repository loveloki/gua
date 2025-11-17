use chinese_lunisolar_calendar::{LunisolarDate, SolarDate};
use chrono::{Datelike, Local, NaiveDate};
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, div,
};
use gpui_component::{
    Disableable, StyledExt,
    button::{Button, ButtonVariants},
    date_picker::{DatePicker, DatePickerEvent, DatePickerState},
};

use crate::qigua::core::QiGuaCore;

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
    date_picker_value: Option<NaiveDate>,
}

impl TimeContent {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let date_picker = cx.new(|cx| {
            let mut picker = DatePickerState::new(window, cx).number_of_months(1);
            picker.set_date(Local::now().naive_local().date(), window, cx);

            picker
        });
        let _subscriptions = vec![cx.subscribe(&date_picker, |this, _, ev, _| match ev {
            DatePickerEvent::Change(date) => {
                this.date_picker_value = date.start();
            }
        })];

        Self {
            date_picker,
            date_picker_value: None,
        }
    }

    fn data_picker_content(&self) -> impl IntoElement {
        DatePicker::new(&self.date_picker)
    }
}

impl Render for TimeContent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();

        let mut content = div()
            .w_80()
            .p_2()
            .v_flex()
            .gap_2()
            .child(NAME)
            .child(self.data_picker_content());

        if let Some(select_date) = self.date_picker_value {
            let year = select_date.year();
            let month = select_date.month();
            let day = select_date.day();

            let lunisolar_date =
                LunisolarDate::from_solar_date(SolarDate::from_ymd(year, month, day).unwrap()).unwrap();

            content = content
                .child(format!("公历: {}-{}-{}", year, month, day))
                .child(format!("农历: {}", lunisolar_date));
        }

        content.child(
            Button::new("calc")
                .label("开始计算")
                .primary()
                .disabled(self.date_picker_value.is_none())
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
    fn calc_gua(&mut self, cx: &mut Context<Self>) {}
    fn name() -> SharedString {
        NAME.into()
    }
}
