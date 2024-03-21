use pitou_core::*;
use yew::prelude::*;

use crate::app::{reusables::*, ApplicationContext};

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
    let class = if matches!(*ctx.active_tab.current_menu.borrow(), AppMenu::Explorer) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Explorer)
    };
    html! {
        <div id="explorer-menu" {class} {onclick}>
            <ExplorerIcon id="explorer-menu-elem" class="menu-item-elem" />
        </div>
    }
}

#[function_component]
pub fn SettingsMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(*ctx.active_tab.current_menu.borrow(), AppMenu::Settings) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Settings)
    };
    html! {
        <div id="settings-menu" {class} {onclick}>
            <SettingsIcon id="settings-menu-elem" class="menu-item-elem" />
        </div>
    }
}

#[function_component]
pub fn FavoritesMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(*ctx.active_tab.current_menu.borrow(), AppMenu::Favorites) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Favorites)
    };
    html! {
        <div id="favorites-menu" {class} {onclick}>
            <FavoritesIcon id="favorites-menu-elem" class="menu-item-elem" />
        </div>
    }
}

#[function_component]
fn RecentsMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(*ctx.active_tab.current_menu.borrow(), AppMenu::Recents) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Recents)
    };
    html! {
        <div id="recents-menu" {class} {onclick}>
            <RecentsIcon id="recents-menu-elem" class="menu-item-elem" />
        </div>
    }
}

#[function_component]
fn TrashMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(*ctx.active_tab.current_menu.borrow(), AppMenu::Trash) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Trash)
    };
    html! {
        <div id="trash-menu" {class} {onclick}>
            <TrashIcon id="recycle-menu-elem" class="menu-item-elem"/>
        </div>
    }
}

#[function_component]
fn LockedMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(*ctx.active_tab.current_menu.borrow(), AppMenu::Locked) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Locked)
    };
    html! {
        <div id="locked-menu" {class} {onclick}>
            <LockedIcon id="locked-menu-elem" class="menu-item-elem"/>
        </div>
    }
}

#[function_component]
fn SearchMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(*ctx.active_tab.current_menu.borrow(), AppMenu::Search) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Search)
    };
    html! {
        <div id="search-menu" {class} {onclick}>
            <SearchIcon id="search-menu-elem" class="menu-item-elem"/>
        </div>
    }
}

#[function_component]
fn CloudMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(*ctx.active_tab.current_menu.borrow(), AppMenu::Cloud) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Cloud)
    };
    html! {
        <div id="cloud-menu" {class} {onclick}>
            <CloudIcon id="cloud-menu-elem" class="menu-item-elem"/>
        </div>
    }
}

#[function_component]
fn HomeMenu(props: &EachMenuProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let class = if matches!(*ctx.active_tab.current_menu.borrow(), AppMenu::Home) {
        "menu-item active-menu"
    } else {
        "menu-item"
    };
    let onclick = {
        let onactivate = props.onactivate.clone();
        move |_| onactivate.emit(AppMenu::Home)
    };
    html! {
        <div id="home-menu" {class} {onclick}>
            <HomeIcon id="home-menu-elem" class="menu-item-elem" />
        </div>
    }
}
