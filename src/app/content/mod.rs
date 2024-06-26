use std::rc::Rc;

use pitou_core::*;
use yew::prelude::*;

pub mod menus;
pub mod pane;
pub mod ribbon;
pub mod status;

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
    pub reload: Callback<()>,
    pub quietreload: Callback<()>,
}

#[function_component]
pub fn Content(props: &ContentProps) -> Html {
    html! {
        <div id = "content">
            <Ribbon navigate_folder={props.navigate_folder.clone()} reload={props.reload.clone()} quietreload={props.quietreload.clone()} onupdatedir={props.onupdatedir.clone()}/>
            <Menus onswitchmenu = { props.onswitchmenu.clone() } />
            <Status />
            <Pane onupdatedir={props.onupdatedir.clone()} onupdatetheme={props.onupdatetheme.clone()} reload={props.reload.clone()} quietreload={props.quietreload.clone()} />
        </div>
    }
}
