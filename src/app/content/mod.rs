use std::rc::Rc;

use pitou_core::*;
use yew::prelude::*;

mod menus;
mod pane;
mod ribbon;
mod status;

use menus::*;
use pane::*;
use ribbon::*;
use status::*;

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    pub onswitchmenu: Callback<AppMenu>,
    pub onupdatedir: Callback<Option<Rc<PitouFile>>>,
    pub onupdatetheme: Callback<ColorTheme>,
    pub navigate_folder: Callback<bool>,
}

#[function_component]
pub fn Content(props: &ContentProps) -> Html {
    html! {
        <div id = "content">
            <Ribbon navigate_folder = { props.navigate_folder.clone() }/>
            <Menus onswitchmenu = { props.onswitchmenu.clone() } />
            <Status />
            <Pane onupdatedir = { props.onupdatedir.clone() } onupdatetheme={ props.onupdatetheme.clone() }/>
        </div>
    }
}
