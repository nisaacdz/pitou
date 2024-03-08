use yew::prelude::*;

use crate::app::reusables::*;

#[function_component]
pub fn Menus() -> Html {
    html! {
        <div id="side-menus">
            <HomeMenu />
            <ExplorerMenu />
            <LockedMenu />
            <FavoritesMenu />
            <RecentsMenu />
            <SearchMenu />
            <CloudMenu />
            <SettingsMenu />
            <TrashMenu />
        </div>
    }
}

#[function_component]
pub fn ExplorerMenu() -> Html {
    html! {
        <div id="explorer-menu" class="menu-item">
            <ExplorerIcon id="explorer-menu-elem" class="menu-item-elem" />
        </div>
    }
}

#[function_component]
pub fn SettingsMenu() -> Html {
    html! {
        <div id="settings-menu" class="menu-item">
            <SettingsIcon id="settings-menu-elem" class="menu-item-elem" />
        </div>
    }
}

#[function_component]
pub fn FavoritesMenu() -> Html {
    html! {
        <div id="favorites-menu" class="menu-item">
            <FavoritesIcon id="favorites-menu-elem" class="menu-item-elem" />
        </div>
    }
}

#[function_component]
fn RecentsMenu() -> Html {
    html! {
        <div id="recents-menu" class="menu-item">
            <RecentsIcon id="recents-menu-elem" class="menu-item-elem" />
        </div>
    }
}

#[function_component]
fn TrashMenu() -> Html {
    html! {
        <div id="recycle-menu" class="menu-item">
            <TrashIcon id="recycle-menu-elem" class="menu-item-elem"/>
        </div>
    }
}

#[function_component]
fn LockedMenu() -> Html {
    html! {
        <div id="locked-menu" class="menu-item">
            <LockedIcon id="locked-menu-elem" class="menu-item-elem"/>
        </div>
    }
}

#[function_component]
fn SearchMenu() -> Html {
    html! {
        <div id="search-menu" class="menu-item">
            <SearchIcon id="search-menu-elem" class="menu-item-elem"/>
        </div>
    }
}

#[function_component]
fn CloudMenu() -> Html {
    html! {
        <div id="cloud-menu" class="menu-item">
            <CloudIcon id="cloud-menu-elem" class="menu-item-elem"/>
        </div>
    }
}

#[function_component]
fn HomeMenu() -> Html {
    html! { 
        <div id="home-menu" class="menu-item">
            <HomeIcon id="home-menu-elem" class="menu-item-elem" />
        </div>
    }
}