use std::rc::Rc;

use pitou_core::{*, frontend::*};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{content::pane::menu_views::*, reusables::{NoArg, NotYetImplementedPane}};
mod menu_views;

#[derive(Properties, PartialEq)]
pub struct PaneProps {
    pub onupdatedir: Callback<Option<Rc<PitouFile>>>,
    pub onupdatetheme: Callback<ColorTheme>,
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
            if pf.is_file() {
                spawn_local(async move { crate::app::cmds::open(pf).await.ok(); })
            } else if pf.is_link() {

            } else {
                ctx.active_tab.update_cur_menu(AppMenu::Explorer);
                onupdatedir.emit(Some(pf))
            }
        }
    };

    let menu = *ctx.active_tab.current_menu.borrow();
    match menu {
        AppMenu::Home => html! { <HomeView {onopen} /> },
        AppMenu::Explorer => html! { <ExplorerView {onopen} /> },
        AppMenu::Trash => html! { <TrashView /> },
        AppMenu::Favorites => html! { <NotYetImplementedPane/> },
        AppMenu::Search => html! { <NotYetImplementedPane/> },
        AppMenu::Locked => html! { <NotYetImplementedPane/> },
        AppMenu::Recents => html! { <NotYetImplementedPane/> },
        AppMenu::Cloud => html! { <NotYetImplementedPane/> },
        AppMenu::Settings => html! { <SettingsView onupdatetheme={props.onupdatetheme.clone()}/> },
    }
}
