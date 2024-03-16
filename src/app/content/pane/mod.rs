use std::rc::Rc;

use pitou_core::PitouFile;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::{content::pane::menu_views::*, reusables::NoArg, ApplicationContext};
mod menu_views;

#[derive(PartialEq, Properties)]
pub struct PaneProps {
    pub onupdatedir: Callback<Option<Rc<PitouFile>>>,
    pub onopen: Callback<Rc<PitouFile>>,
}

#[function_component]
pub fn Pane(props: &PaneProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();

    if ctx.active_tab.current_dir.borrow().is_none() {
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

    let menu = *ctx.active_tab.current_menu.borrow();
    match menu {
        pitou_core::frontend::AppMenu::Home => html! { <HomeView /> },
        pitou_core::frontend::AppMenu::Explorer => {
            html! { <ExplorerView onopen={props.onopen.clone()} /> }
        }
        pitou_core::frontend::AppMenu::Trash => html! { <TrashView /> },
        pitou_core::frontend::AppMenu::Favorites => html! {},
        pitou_core::frontend::AppMenu::Search => html! {},
        pitou_core::frontend::AppMenu::Locked => html! {},
        pitou_core::frontend::AppMenu::Recents => html! {},
        pitou_core::frontend::AppMenu::Cloud => html! {},
        pitou_core::frontend::AppMenu::Settings => html! {},
    }
}
