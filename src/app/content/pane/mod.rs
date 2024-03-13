use yew::prelude::*;

use crate::app::{content::pane::menu_views::*, ApplicationContext};
mod menu_views;

#[derive(PartialEq, Properties)]
pub struct PaneProps {}

#[function_component]
pub fn Pane(_props: &PaneProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();

    if let Some(active_tab) = ctx.active_tab {
        let menu = *active_tab.current_menu.borrow();
        match menu {
            pitou_core::frontend::AppMenu::Home => html! { <HomeView /> },
            pitou_core::frontend::AppMenu::Explorer => html! { <ExplorerView /> },
            pitou_core::frontend::AppMenu::Trash => html! { <TrashView /> },
            pitou_core::frontend::AppMenu::Favorites => html! {},
            pitou_core::frontend::AppMenu::Search => html! {},
            pitou_core::frontend::AppMenu::Locked => html! {},
            pitou_core::frontend::AppMenu::Recents => html! {},
            pitou_core::frontend::AppMenu::Cloud => html! {},
            pitou_core::frontend::AppMenu::Settings => html! {},
        }
    } else {
        html! {}
    }
}
