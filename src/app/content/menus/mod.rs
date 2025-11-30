use pitou_core::*;
use yew::prelude::*;

use crate::app::{reusables::MatIcon, ApplicationContext};

#[derive(Properties, PartialEq)]
pub struct EachMenuProps {
    pub onactivate: Callback<AppMenu>,
}

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    pub onswitchmenu: Callback<AppMenu>,
}

#[function_component]
pub fn Menus(props: &MenuProps) -> Html {
    html! {
        <div id="side-menus">
            <HomeMenu onactivate = { props.onswitchmenu.clone() } />
            <ExplorerMenu onactivate = { props.onswitchmenu.clone() } />
            <LockedMenu onactivate = { props.onswitchmenu.clone() } />
            <FavoritesMenu onactivate = { props.onswitchmenu.clone() } />
            <RecentsMenu onactivate = { props.onswitchmenu.clone() } />
            <SearchMenu onactivate = { props.onswitchmenu.clone() } />
            <CloudMenu onactivate = { props.onswitchmenu.clone() } />
            <SettingsMenu onactivate = { props.onswitchmenu.clone() } />
            <TrashMenu onactivate = { props.onswitchmenu.clone() } />
        </div>
    }
}

#[function_component]
pub fn ExplorerMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(ctx.current_menu(), AppMenu::Explorer) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Explorer)
    };
    html! {
        <div id="explorer-menu" {class} {onclick} title="explorer">
            <MatIcon name="folder_open" class="menu-item-icon" />
        </div>
    }
}

#[function_component]
pub fn SettingsMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(ctx.current_menu(), AppMenu::Settings) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Settings)
    };
    html! {
        <div id="settings-menu" {class} {onclick} title="settings">
            <MatIcon name="settings" class="menu-item-icon" />
        </div>
    }
}

#[function_component]
pub fn FavoritesMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(ctx.current_menu(), AppMenu::Favorites) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Favorites)
    };
    html! {
        <div id="favorites-menu" {class} {onclick} title="pinned files">
            <MatIcon name="star" class="menu-item-icon" />
        </div>
    }
}

#[function_component]
fn RecentsMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(ctx.current_menu(), AppMenu::Recents) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Recents)
    };
    html! {
        <div id="recents-menu" {class} {onclick} title="recent files">
            <MatIcon name="history" class="menu-item-icon" />
        </div>
    }
}

#[function_component]
fn TrashMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(ctx.current_menu(), AppMenu::Trash) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Trash)
    };
    html! {
        <div id="trash-menu" {class} {onclick} title="recycle bin">
            <MatIcon name="delete" class="menu-item-icon" />
        </div>
    }
}

#[function_component]
fn LockedMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(ctx.current_menu(), AppMenu::Locked) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Locked)
    };
    html! {
        <div id="locked-menu" {class} {onclick} title="locked files">
            <MatIcon name="lock" class="menu-item-icon" />
        </div>
    }
}

#[function_component]
fn SearchMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(ctx.current_menu(), AppMenu::Search) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Search)
    };
    html! {
        <div id="search-menu" {class} {onclick} title="advanced search">
            <MatIcon name="search" class="menu-item-icon" />
        </div>
    }
}

#[function_component]
fn CloudMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(ctx.current_menu(), AppMenu::Cloud) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Cloud)
    };
    html! {
        <div id="cloud-menu" {class} {onclick} title="cloud storage">
            <MatIcon name="cloud" class="menu-item-icon" />
        </div>
    }
}

#[function_component]
fn HomeMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(ctx.current_menu(), AppMenu::Home) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Home)
    };
    html! {
        <div id="home-menu" {class} {onclick} title="home">
            <MatIcon name="home" class="menu-item-icon" />
        </div>
    }
}
