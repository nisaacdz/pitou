use yew::prelude::*;
use std::rc::Rc;
use pitou_core::frontend::*;

mod ribbon;
mod menus;
mod status;
mod pane;

use ribbon::*;
use menus::*;
use status::*;
use pane::*;

#[derive(PartialEq, Properties)]
pub struct ContentProps {
    pub active_tab: Rc<TabCtx>,
}

#[function_component]
pub fn Content(prop: &ContentProps) -> Html {
    html! {
        <div id = "content">
            <Ribbon />
            <Menus />
            <Status />
            <Pane />
        </div>
    }
}