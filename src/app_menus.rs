use gpui::{App, Menu, MenuItem, actions};

actions!(baGua, [Quit, CloseWindow, ToggleSearch,]);

pub fn init(title: &'static str, cx: &mut App) {
    cx.on_action(quit);

    cx.set_menus(vec![Menu {
        name: title.into(),
        items: vec![MenuItem::action("Quit", Quit), MenuItem::separator()],
    }]);
}

pub fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}
