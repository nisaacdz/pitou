use std::{cell::RefCell, rc::Rc};

use pitou_core::frontend::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
mod body;
mod title_bar;

use body::Body;
use title_bar::TitleBar;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Clone)]
pub struct AllTabsCtx {
    pub all_tabs: Rc<RefCell<Vec<Rc<TabCtx>>>>,
    pub active_tab: usize,
}

impl PartialEq for AllTabsCtx {
    fn eq(&self, other: &Self) -> bool {
        self.active_tab == other.active_tab
    }
}

impl AllTabsCtx {
    fn debug_version() -> Self {
        let all_tabs = Rc::new(RefCell::new(vec![Rc::new(TabCtx::default()); 6]));
        Self {
            active_tab: 3,
            all_tabs,
        }
    }
    fn default() -> Self {
        let all_tabs = Rc::new(RefCell::new(vec![Rc::new(TabCtx::default())]));
        Self {
            active_tab: 0,
            all_tabs,
        }
    }
}

#[function_component]
pub fn App() -> Html {
    let tabs_ctx = use_state(|| AllTabsCtx::debug_version());
    let genr_ctx = use_state(|| {
        let v = GenCtx::default();
        v
    });

    let active_tab = tabs_ctx.all_tabs.borrow()[tabs_ctx.active_tab].clone();

    let ColorTheme {
        background1,
        background2,
        foreground1,
        foreground2,
        spare1,
        spare2,
    } = (*genr_ctx).color_theme;

    let style = format! {r"
    --primary-background-color: {background1};
    --seconday-background-color: {background2};
    --primary-foreground-color: {foreground1};
    --secondary-foreground-color: {foreground2};
    --primary-spare-color: {spare1};
    --secondary-spare-color: {spare2};
    "};

    let onclose = {
        |()| {
            let wd = tauri_sys::window::current_window();
            spawn_local(async move {
                wd.close().await.unwrap()
            })
        }
    };
    let ontogglemaximize = {
        |()| {
            let wd = tauri_sys::window::current_window();
            spawn_local(async move {
                wd.toggle_maximize().await.unwrap()
            })
        }
    };
    let onminimize = {
        |()| {
            let wd = tauri_sys::window::current_window();
            spawn_local(async move {
                wd.minimize().await.unwrap()
            })
        }
    };

    html! {
        <main {style}>
            <TitleBar tabs_ctx = { (*tabs_ctx).clone() } {onclose} {ontogglemaximize} {onminimize}/>
            <Body {active_tab} />
        </main>
    }
}
