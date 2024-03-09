use std::rc::Rc;

use pitou_core::frontend::TabCtx;
use yew::prelude::*;

use crate::app::content::pane::menu_views::explorer::ExplorerView;
mod menu_views;

#[derive(PartialEq, Properties)]
pub struct PaneProp {
    pub ctx: Rc<TabCtx>,
}

#[function_component]
pub fn Pane(prop: &PaneProp) -> Html {
    match prop.ctx.current_menu {
        pitou_core::frontend::AppMenu::Home => todo!(),
        pitou_core::frontend::AppMenu::Explorer => html! { <ExplorerView /> },
        pitou_core::frontend::AppMenu::Trash => todo!(),
        pitou_core::frontend::AppMenu::Favorites => todo!(),
        pitou_core::frontend::AppMenu::Search => todo!(),
        pitou_core::frontend::AppMenu::Locked => todo!(),
        pitou_core::frontend::AppMenu::Recents => todo!(),
        pitou_core::frontend::AppMenu::Cloud => todo!(),
        pitou_core::frontend::AppMenu::Settings => todo!(),
    }
}