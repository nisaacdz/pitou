use std::rc::Rc;

use pitou_core::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{content::pane::menu_views::*, reusables::NoArg, ApplicationContext};
mod menu_views;

#[derive(Properties, PartialEq)]
pub struct PaneProps {
    pub onupdatedir: Callback<Option<Rc<PitouFile>>>,
}

#[function_component]
pub fn Pane(props: &PaneProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();

    if ctx.active_tab.current_dir().is_none() {
        let onupdatedir = props.onupdatedir.clone();
        spawn_local(async move {
            let res = tauri_sys::tauri::invoke("default_folder", &NoArg)
                .await
                .map(|v| Rc::new(v))
                .ok();
            onupdatedir.emit(res)
        });
        return html! {};
    }

    let onopen = {
        let onupdatedir = props.onupdatedir.clone();
        let ctx = ctx.clone();
        move |pf: Rc<PitouFile>| {
            ctx.active_tab.update_cur_menu(AppMenu::Explorer);
            if pf.is_file() {
            } else if pf.is_link() {
            } else {
                onupdatedir.emit(Some(pf))
            }
        }
    };

    let menu = *ctx.active_tab.current_menu.borrow();
    match menu {
        AppMenu::Home => html! { <HomeView {onopen} /> },
        AppMenu::Explorer => html! { <ExplorerView {onopen} /> },
        AppMenu::Trash => html! { <TrashView /> },
        AppMenu::Favorites => html! {},
        AppMenu::Search => html! {},
        AppMenu::Locked => html! {},
        AppMenu::Recents => html! {},
        AppMenu::Cloud => html! {},
        AppMenu::Settings => html! {},
    }
}
