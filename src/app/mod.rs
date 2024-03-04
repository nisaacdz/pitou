use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use pitou_core::frontend::*;
mod title_bar;
mod body;

use title_bar::TitleBar;
use body::Body;

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
    pub active_tab: Rc<TabCtx>,
}

impl PartialEq for AllTabsCtx {
    fn eq(&self, other: &Self) -> bool {
        self.active_tab == other.active_tab
    }
}

impl AllTabsCtx {
    fn default() -> Self {
        let active_tab = Rc::new(TabCtx::default());
        let all_tabs = Rc::new(RefCell::new(vec![active_tab.clone()]));
        Self { active_tab, all_tabs }
    }
}


#[function_component]
pub fn App() -> Html {
    let tabs_ctx = use_state(|| AllTabsCtx::default());
    let genr_ctx = use_state(|| GenCtx::new());

    html! {
        <>
            <TitleBar tabs_ctx = { (*tabs_ctx).clone() } />
            <Body active_tab = { tabs_ctx.active_tab.clone() } />
        </>
    }
}


