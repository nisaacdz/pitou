use std::{cell::RefCell, rc::Rc};

use pitou_core::{frontend::*, PitouFilePath};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
mod content;
mod title_bar;

use content::*;
use title_bar::TitleBar;

use crate::app::reusables::NoArg;

pub mod reusables;

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

#[derive(Clone)]
pub struct ApplicationContext {
    pub gen_ctx: Rc<GenCtx>,
    pub active_tab: Rc<TabCtx>, 
}

impl PartialEq for ApplicationContext {
    fn eq(&self, other: &Self) -> bool {
        // TODO to change this to correct implementation
        false
    }
}

impl ApplicationContext {
    fn new(gen_ctx: Rc<GenCtx>, active_tab: Rc<TabCtx>) -> Self {
        Self { gen_ctx, active_tab }
    }
}

impl AllTabsCtx {
    pub fn add_tab(mut self, default_dir: PitouFilePath) -> Self {
        let mut all_tabs = self.all_tabs.borrow_mut();
        let next_idx = all_tabs.len();
        all_tabs.push(Rc::new(TabCtx::new_with(default_dir)));
        std::mem::drop(all_tabs);
        self.active_tab = next_idx;
        self
    }

    pub fn change_tab(mut self, idx: usize) -> Self {
        self.active_tab = idx;
        self
    }

    pub fn remove_tab(mut self, idx: usize) -> Self {
        let mut all_tabs = self.all_tabs.borrow_mut();
        if all_tabs.len() <= 1 { return Self::default() }
        all_tabs.remove(idx);
        std::mem::drop(all_tabs);
        if idx <= self.active_tab {
            if self.active_tab != 0 {
                self.active_tab -= 1;
            }
        }
        self
    }

    fn current_tab(&self) -> Rc<TabCtx> {
        self.all_tabs.borrow()[self.active_tab].clone()
    }

    fn change_menu(self, menu: AppMenu) -> Self {
        let current_tab = self.current_tab();
        *current_tab.current_menu.borrow_mut() = menu;
        self
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
    let tabs_ctx = use_state(|| AllTabsCtx::default());
    
    let genr_ctx = use_state(|| Rc::new(GenCtx::default()));
    
    let add_tab = {
        let tabs_ctx = tabs_ctx.clone();
        move |()| {
            let tabs_ctx = tabs_ctx.clone();
            spawn_local(async move {
                let default_dir = tauri_sys::tauri::invoke("default_folder", &NoArg).await.unwrap();
                tabs_ctx.set((*tabs_ctx).clone().add_tab(default_dir))
            })
        }
    };

    let rem_tab = {
        let tabs_ctx = tabs_ctx.clone();
        move |idx| tabs_ctx.set((*tabs_ctx).clone().remove_tab(idx))
    };

    let change_tab = {
        let tabs_ctx = tabs_ctx.clone();
        move |idx| tabs_ctx.set((*tabs_ctx).clone().change_tab(idx))
    };

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
    --secondary-background-color: {background2};
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

    let onswitchmenu = {
        let tabs_ctx = tabs_ctx.clone();
        move |menu| {
            tabs_ctx.set((*tabs_ctx).clone().change_menu(menu))
        }
    };

    let active_tab = tabs_ctx.all_tabs.borrow()[tabs_ctx.active_tab].clone();

    html! {
            <main {style}>
                <ContextProvider<ApplicationContext> context={ApplicationContext::new((*genr_ctx).clone(), active_tab)}>
                    <TitleBar tabs_ctx = { (*tabs_ctx).clone() } {onclose} {ontogglemaximize} {onminimize} {add_tab} {rem_tab} {change_tab} />
                    <Content { onswitchmenu } />
                </ContextProvider<ApplicationContext>>
            </ main>
    }
}
