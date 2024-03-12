use yew::prelude::*;
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
    pub onswitchmenu: Callback<AppMenu>,
}

#[function_component]
pub fn Content(props: &ContentProps) -> Html {
    html! {
        <div id = "content">
            <Ribbon />
            <Menus onswitchmenu = { props.onswitchmenu.clone() } />
            <Status />
            <Pane />
        </div>
    }
}