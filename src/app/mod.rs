use std::{cell::RefCell, rc::Rc};

use pitou_core::{frontend::*, *};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
mod cmds;
mod content;
mod events;
mod title_bar;

use content::*;
use title_bar::TitleBar;

pub mod reusables;

#[function_component]
pub fn App() -> Html {
    let tabs_ctx = use_state(|| Rc::new(AllTabsCtx::default()));

    let genr_ctx = use_state(|| Rc::new(RefCell::new(GenCtx::default())));

    let static_data = use_state(|| Rc::new(StaticData::new()));

    let ctx = {
        let active_tab = tabs_ctx.current_tab();
        ApplicationContext::new((*genr_ctx).clone(), active_tab, (*static_data).clone())
    };

    let add_tab = {
        let tabs_ctx = tabs_ctx.clone();
        move |()| {
            let new_tabs = (**tabs_ctx).clone().add_tab();
            tabs_ctx.set(Rc::new(new_tabs))
        }
    };

    let rem_tab = {
        let tabs_ctx = tabs_ctx.clone();
        move |idx| {
            let new_tabs = (**tabs_ctx)
                .clone()
                .remove_tab(idx)
                .unwrap_or(AllTabsCtx::default());
            tabs_ctx.set(Rc::new(new_tabs));
        }
    };

    let change_tab = {
        let tabs_ctx = tabs_ctx.clone();
        let ctx = ctx.clone();
        move |idx| {
            let new_tabs = (**tabs_ctx).clone().change_tab(idx);
            ctx.toggle_refresher_state();
            tabs_ctx.set(Rc::new(new_tabs))
        }
    };

    let onupdatedir = {
        let ctx = ctx.clone();
        let tabs_ctx = tabs_ctx.clone();
        let static_data = static_data.clone();
        move |file| {
            let new_tabs = (**tabs_ctx).clone();
            static_data.clear_all_selections();
            let cur_tab = new_tabs.current_tab();
            cur_tab.update_children(None);
            cur_tab.update_siblings(None);
            cur_tab.update_cur_dir(file);
            ctx.toggle_refresher_state();
            tabs_ctx.set(Rc::new(new_tabs))
        }
    };

    let navigate_folder = {
        let tabs_ctx = tabs_ctx.clone();
        let ctx = ctx.clone();
        move |forward| {
            if forward {
                tabs_ctx.current_tab().navigate_forward()
            } else {
                tabs_ctx.current_tab().navigate_backward()
            }
            ctx.toggle_refresher_state();
            tabs_ctx.set((*tabs_ctx).clone());
        }
    };

    let ColorTheme {
        background1,
        background2,
        foreground1,
        foreground2,
        spare1,
        spare2,
    } = (*genr_ctx).borrow().color_theme;

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
            let new_tabs = (**tabs_ctx).clone().change_menu(menu);
            tabs_ctx.set(Rc::new(new_tabs))
        }
    };

    let onupdatetheme = {
        let genr_ctx = genr_ctx.clone();
        let ctx = ctx.clone();
        move |newtheme| {
            ctx.update_color_theme(newtheme);
            genr_ctx.set((*genr_ctx).clone())
        }
    };

    let reload = {
        let genr_ctx = genr_ctx.clone();
        let ctx = ctx.clone();
        move |()| {
            ctx.toggle_refresher_state();
            ctx.static_data.clear_all_selections();
            genr_ctx.set((*genr_ctx).clone())
        }
    };

    let quietreload = {
        let genr_ctx = genr_ctx.clone();
        let ctx = ctx.clone();
        move |()| {
            ctx.toggle_refresher_state();
            genr_ctx.set((*genr_ctx).clone())
        }
    };

    html! {
        <main {style}>
            <ContextProvider<ApplicationContext> context={ctx}>
                <TitleBar tabs_ctx = { (*tabs_ctx).clone() } {onclose} {ontogglemaximize} {onminimize} {add_tab} {rem_tab} {change_tab} />
                <Content {onswitchmenu} {onupdatedir} {onupdatetheme} {navigate_folder} {reload} {quietreload}/>
            </ContextProvider<ApplicationContext>>
        </main>
    }
}
