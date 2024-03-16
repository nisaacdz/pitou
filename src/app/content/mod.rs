use std::rc::Rc;

use pitou_core::{frontend::*, PitouFile};
use yew::prelude::*;

mod menus;
mod pane;
mod ribbon;
mod status;

use menus::*;
use pane::*;
use ribbon::*;
use status::*;

#[derive(PartialEq, Properties)]
pub struct ContentProps {
    pub onswitchmenu: Callback<AppMenu>,
    pub onupdatedir: Callback<Option<Rc<PitouFile>>>,
}

#[function_component]
pub fn Content(props: &ContentProps) -> Html {
    html! {
        <div id = "content">
            <Ribbon />
            <Menus onswitchmenu = { props.onswitchmenu.clone() } />
            <Status />
            <Pane onupdatedir = { props.onupdatedir.clone() } />
        </div>
    }
}
