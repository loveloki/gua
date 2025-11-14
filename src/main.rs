use gpui::{AppContext, Application, TitlebarOptions, WindowOptions};
use gpui_component::Root;

use crate::{assets::Assets, state::global::GlobalState, ui::home::HomeWindow};

mod app_menus;
mod assets;
mod gua;
mod state;
mod ui;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);
        GlobalState::init(cx);

        app_menus::init("简单的八卦计算器", cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    show: true,
                    titlebar: Some(TitlebarOptions {
                        title: Some("简单的自己算卦".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let view = HomeWindow::view(window, cx);
                    // This first level on the window, should be a Root.
                    cx.new(|cx| Root::new(view.into(), window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
