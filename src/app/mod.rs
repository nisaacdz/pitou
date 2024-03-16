use std::{cell::RefCell, rc::Rc};

use pitou_core::frontend::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
mod content;
mod title_bar;

use content::*;
use title_bar::TitleBar;

pub mod reusables;

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
    fn eq(&self, _other: &Self) -> bool {
        // TODO to change this to correct implementation
        false
    }
}

impl ApplicationContext {
    fn new(gen_ctx: Rc<GenCtx>, active_tab: Rc<TabCtx>) -> Self {
        Self {
            gen_ctx,
            active_tab,
        }
    }
}

impl AllTabsCtx {
    pub fn default() -> Self {
        let active_tab = Rc::new(TabCtx::default());
        let all_tabs = Rc::new(RefCell::new(vec![active_tab]));
        Self {
            all_tabs,
            active_tab: 0,
        }
    }

    pub fn add_tab(mut self) -> Self {
        let mut all_tabs = self.all_tabs.borrow_mut();
        let next_idx = all_tabs.len();
        all_tabs.push(Rc::new(TabCtx::default()));
        std::mem::drop(all_tabs);
        self.active_tab = next_idx;
        self
    }

    pub fn change_tab(mut self, idx: usize) -> Self {
        self.active_tab = idx;
        self
    }

    pub fn remove_tab(mut self, idx: usize) -> Option<Self> {
        let mut all_tabs = self.all_tabs.borrow_mut();
        if all_tabs.len() <= 1 {
            return None;
        }
        all_tabs.remove(idx);
        std::mem::drop(all_tabs);
        if idx <= self.active_tab {
            if self.active_tab != 0 {
                self.active_tab -= 1;
            }
        }
        Some(self)
    }

    fn current_tab(&self) -> Rc<TabCtx> {
        self.all_tabs.borrow()[self.active_tab].clone()
    }

    fn change_menu(self, menu: AppMenu) -> Self {
        let current_tab = self.current_tab();
        *current_tab.current_menu.borrow_mut() = menu;
        self
    }
}

#[function_component]
pub fn App() -> Html {
    let tabs_ctx = use_state(|| AllTabsCtx::default());

    let genr_ctx = use_state(|| Rc::new(GenCtx::default()));

    let add_tab = {
        let tabs_ctx = tabs_ctx.clone();
        move |()| {
            let new_tabs = (*tabs_ctx).clone().add_tab();
            tabs_ctx.set(new_tabs)
        }
    };

    let rem_tab = {
        let tabs_ctx = tabs_ctx.clone();
        move |idx| {
            let new_tabs = (*tabs_ctx)
                .clone()
                .remove_tab(idx)
                .unwrap_or(AllTabsCtx::default());
            tabs_ctx.set(new_tabs);
        }
    };

    let change_tab = {
        let tabs_ctx = tabs_ctx.clone();
        move |idx| {
            let new_tabs = (*tabs_ctx).clone().change_tab(idx);
            tabs_ctx.set(new_tabs)
        }
    };

    let onupdatedir = {
        let tabs_ctx = tabs_ctx.clone();
        move |file| {
            let new_tabs = (*tabs_ctx).clone();
            new_tabs.current_tab().update_cur_dir(file);
            tabs_ctx.set(new_tabs)
        }
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
            spawn_local(async move { wd.close().await.unwrap() })
        }
    };
    let ontogglemaximize = {
        |()| {
            let wd = tauri_sys::window::current_window();
            spawn_local(async move { wd.toggle_maximize().await.unwrap() })
        }
    };
    let onminimize = {
        |()| {
            let wd = tauri_sys::window::current_window();
            spawn_local(async move { wd.minimize().await.unwrap() })
        }
    };

    let onswitchmenu = {
        let tabs_ctx = tabs_ctx.clone();
        move |menu| {
            let new_tabs = (*tabs_ctx).clone().change_menu(menu);
            tabs_ctx.set(new_tabs)
        }
    };

    let active_tab = tabs_ctx.all_tabs.borrow()[tabs_ctx.active_tab].clone();

    html! {
        <main {style}>
            <ContextProvider<ApplicationContext> context={ApplicationContext::new((*genr_ctx).clone(), active_tab)}>
                <TitleBar tabs_ctx = { (*tabs_ctx).clone() } {onclose} {ontogglemaximize} {onminimize} {add_tab} {rem_tab} {change_tab} />
                <Content {onswitchmenu} {onupdatedir}/>
            </ContextProvider<ApplicationContext>>
        </main>
    }
}
