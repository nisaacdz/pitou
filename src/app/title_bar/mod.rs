use pitou_core::{frontend::*, *};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

use super::{reusables::*, AllTabsCtx};

#[derive(Properties)]
pub struct TitleBarProps {
    pub tabs_ctx: AllTabsCtx,
    pub onclose: Callback<()>,
    pub ontogglemaximize: Callback<()>,
    pub onminimize: Callback<()>,
    pub add_tab: Callback<()>,
    pub rem_tab: Callback<usize>,
    pub change_tab: Callback<usize>,
}

impl PartialEq for TitleBarProps {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

#[function_component]
pub fn TitleBar(props: &TitleBarProps) -> Html {
    let onclose = props.onclose.clone();
    let ontogglemaximize = props.ontogglemaximize.clone();
    let onminimize = props.onminimize.clone();

    let add_tab = props.add_tab.clone();
    let rem_tab = props.rem_tab.clone();
    let change_tab = props.change_tab.clone();
    let tabs_ctx = props.tabs_ctx.clone();

    let tabbed_interface =
        html! { <TabbedInterface tabs_ctx = { tabs_ctx } {add_tab} {rem_tab} {change_tab} /> };

    html! {
        <div id="title-bar" data-tauri-drag-region = "true">
            <AppLogo />
            { tabbed_interface }
            <ControlBox {onclose} {onminimize} {ontogglemaximize} />
        </div>
    }
}

#[derive(Properties)]
struct TabbedInterfaceProps {
    tabs_ctx: AllTabsCtx,
    add_tab: Callback<()>,
    rem_tab: Callback<usize>,
    change_tab: Callback<usize>,
}

impl PartialEq for TabbedInterfaceProps {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

#[function_component]
fn TabbedInterface(props: &TabbedInterfaceProps) -> Html {
    let AllTabsCtx {
        all_tabs,
        active_tab,
    } = props.tabs_ctx.clone();
    let tabs_disp = all_tabs
        .borrow()
        .iter()
        .enumerate()
        .map(|(idx, ctx)| {
            let ctx = ctx.clone();
            let rem_tab = props.rem_tab.clone();
            let change_tab = props.change_tab.clone();
            if idx == active_tab {
                html! { <ActiveTab {idx} {ctx} {rem_tab} {change_tab} /> }
            } else {
                html! { <InactiveTab {idx} {ctx} {rem_tab} {change_tab} /> }
            }
        })
        .chain(Some(html! { <AddTab add_tab = {props.add_tab.clone()} /> }))
        .collect::<Html>();
    html! {
        <div id="tabs-container" data-tauri-drag-region = "true">
            <TabsShower />
            <div id="all-tabs" data-tauri-drag-region = "true">
                {tabs_disp}
            </div>
        </div>
    }
}

#[derive(Properties)]
struct TabProps {
    idx: usize,
    ctx: Rc<TabCtx>,
    rem_tab: Callback<usize>,
    change_tab: Callback<usize>,
}

impl PartialEq for TabProps {
    fn eq(&self, other: &Self) -> bool {
        (self.ctx.current_dir() == other.ctx.current_dir())
            && (self.ctx.current_menu == other.ctx.current_menu)
    }
}

#[function_component]
fn InactiveTab(props: &TabProps) -> Html {
    let onclose = {
        let rem_tab = props.rem_tab.clone();
        let idx = props.idx;
        move |e: MouseEvent| {
            e.stop_propagation();
            rem_tab.emit(idx)
        }
    };

    let onchange = {
        let change_tab = props.change_tab.clone();
        let idx = props.idx;
        move |_| change_tab.emit(idx)
    };
    let dir = props.ctx.current_dir();
    let name = dir.as_ref().map(|v| v.name()).unwrap_or_default();

    html! {
        <div class = "tab inactive" onclick = {onchange}>
            <div class="tab-logo">
                <TabLogo menu = { *props.ctx.current_menu.borrow() } />
            </div>
            <div class="tab-text">{ name }</div>
            <div class="tab-close" onclick = {onclose}>
                <svg class="tab-close-cross" viewBox="0 0 24 24" width="24" height="24">
                    <path d="M17.71 6.71a1 1 0 0 0-1.42 0L12 10.59l-4.29-4.3a1 1 0 0 0-1.42
                    1.42L10.59 12l-4.3 4.29a1 1 0 1 0 1.42 1.42L12 13.41l4.29 4.3a1 1 0 0 0 
                    1.42-1.42L13.41 12l4.3-4.29a1 1 0 0 0 0-1.42z"/>
                </svg>
            </div>
        </div>
    }
}

#[function_component]
fn ActiveTab(props: &TabProps) -> Html {
    let onclose = {
        let rem_tab = props.rem_tab.clone();
        let idx = props.idx;
        move |e: MouseEvent| {
            e.stop_propagation();
            rem_tab.emit(idx)
        }
    };
    let dir = props.ctx.current_dir();
    let name = dir.as_ref().map(|v| v.name()).unwrap_or_default();

    html! {
        <div class = "tab active">
            <div class="tab-logo" data-tauri-drag-region = "true">
                <TabLogo menu = { *props.ctx.current_menu.borrow() } />
            </div>
            <div class="tab-text" data-tauri-drag-region = "true">{ name }</div>
            <div class="tab-close" onclick={onclose}>
                <svg class="tab-close-cross" viewBox="0 0 24 24" width="24" height="24">
                    <path d="M17.71 6.71a1 1 0 0 0-1.42 0L12 10.59l-4.29-4.3a1 1 0 0 0-1.42
                    1.42L10.59 12l-4.3 4.29a1 1 0 1 0 1.42 1.42L12 13.41l4.29 4.3a1 1 0 0 0 
                    1.42-1.42L13.41 12l4.3-4.29a1 1 0 0 0 0-1.42z"/>
                </svg>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AddTabProps {
    add_tab: Callback<()>,
}

#[function_component]
fn AddTab(prop: &AddTabProps) -> Html {
    let onclick = {
        let add_tab = prop.add_tab.clone();
        move |_| add_tab.emit(())
    };
    html! {
        <div class = "tab add-tab" {onclick} >
            <svg id = "add-tab-plus" viewBox="0 0 23 23" width="24" height="24">
                <path d="M19 11h-6V5a1 1 0 0 0-2 0v6H5a1 1 0 0 0 0 2h6v6a1 1 0 0 0 2 0v-6h6a1 1 0 0 0 0-2z"/>
            </svg>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ControlBoxProps {
    onclose: Callback<()>,
    ontogglemaximize: Callback<()>,
    onminimize: Callback<()>,
}

#[function_component]
fn ControlBox(props: &ControlBoxProps) -> Html {
    let is_maximized = use_state_eq(|| true);

    {
        let is_maximized = is_maximized.clone();
        use_interval(
            move || {
                let is_maximized = is_maximized.clone();
                spawn_local(async move {
                    is_maximized.set(
                        tauri_sys::window::current_window()
                            .is_maximized()
                            .await
                            .unwrap_or(false),
                    )
                })
            },
            200,
        );
    }

    let onclose = {
        let onclose = props.onclose.clone();
        move |_| onclose.emit(())
    };

    let onresize = {
        let ontoggle = props.ontogglemaximize.clone();
        move |_| ontoggle.emit(())
    };

    let onminimize = {
        let onminimize = props.onminimize.clone();
        move |_| onminimize.emit(())
    };

    let maxi_or_restore = if *is_maximized {
        html! {
            <svg class="elem" width="24" height="24" viewBox="-6 -6 32 32">
                <g stroke="none" stroke-width="1" fill="none" fill-rule="evenodd" stroke-linecap="round" stroke-linejoin="round">
                    <g transform="translate(-969.000000, -748.000000)" id="Group" stroke="#000000" stroke-width="2">
                        <g transform="translate(967.000000, 746.000000)" id="Shape">
                            <path class="maxi-button-line" d="M3,15 L9,15 L9,21 M15,21 L15,15 L21,15 M21,9 L15,9 L15,3 M9,3 L9,9 L3,9">
                            </path>
                        </g>
                    </g>
                </g>
            </svg>
        }
    } else {
        html! {
            <svg class="elem" width="24" height="24" viewBox="-3 -4 32 32" fill="none">
                <path class="maxi-button-line" d="M8 2H4C2.89543 2 2 2.89543 2 4V8" stroke-width="2" stroke-linecap="round"/>
                <path class="maxi-button-line" d="M22 8L22 4C22 2.89543 21.1046 2 20 2H16" stroke-width="2" stroke-linecap="round"/>
                <path class="maxi-button-line" d="M16 22L20 22C21.1046 22 22 21.1046 22 20L22 16" stroke-width="2" stroke-linecap="round"/>
                <path class="maxi-button-line" d="M8 22L4 22C2.89543 22 2 21.1046 2 20V16" stroke-width="2" stroke-linecap="round"/>
            </svg>
        }
    };

    html! {
        <div id="control-box">
            <div class="control-button close-button" onclick={onclose}>
                <svg class="elem" id="close-button-cross" viewBox="0 0 24 24" width="24" height="24">
                    <path d="M17.71 6.71a1 1 0 0 0-1.42 0L12 10.59l-4.29-4.3a1 1 0 0 0-1.42 1.42L10.59
                    12l-4.3 4.29a1 1 0 1 0 1.42 1.42L12 13.41l4.29 4.3a1 1 0 0 0 1.42-1.42L13.41 12l4.3-4.29a1 
                    1 0 0 0 0-1.42z"/>
                </svg>
            </div>
            <div class="control-button maxi-button" onclick={onresize}>
                { maxi_or_restore }
            </div>
            <div class="control-button mini-button" onclick={onminimize}>
                <svg class="elem" id="mini-button-dash" viewBox="0 0 24 24" width="24" height="24">
                    <rect x="5" y="12" width="14" height="2"/>
                </svg>
            </div>
        </div>
    }
}

#[function_component]
fn AppLogo() -> Html {
    html! {
        <div id="app-logo" data-tauri-drag-region = "true">
            <img id="app-logo-img" src = "./public/pitou_logo.png" />
            <div id="app-logo-name">{ "pitou" }</div>
            <div class="app-logo-btn">
                <img src="./public/undo2.png"/>
            </div>
            <div class="app-logo-btn">
                <img src="./public/redo2.png"/>
            </div>
        </div>
    }
}

#[function_component]
fn TabsShower() -> Html {
    html! {
        <div id="tabs-shower">
            <DBChevronDownIcon id="tabs-shower-chevron" class=""/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabLogoProp {
    menu: AppMenu,
}

#[function_component]
pub fn TabLogo(prop: &TabLogoProp) -> Html {
    match prop.menu {
        AppMenu::Home => html! { <HomeIcon id="home-tab-logo" class="tab-logo-elem" /> },
        AppMenu::Explorer => {
            html! { <ExplorerIcon id="explorer-tab-logo" class="tab-logo-elem" /> }
        }
        AppMenu::Trash => html! { <TrashIcon id="trash-tab-logo" class="tab-logo-elem" /> },
        AppMenu::Favorites => {
            html! { <FavoritesIcon id="favorites-tab-logo" class="tab-logo-elem" /> }
        }
        AppMenu::Recents => html! { <RecentsIcon id="recents-tab-logo" class="tab-logo-elem" /> },
        AppMenu::Cloud => html! { <CloudIcon id="cloud-tab-logo" class="tab-logo-elem" /> },
        AppMenu::Settings => {
            html! { <SettingsIcon id="settings-tab-logo" class="tab-logo-elem" /> }
        }
        AppMenu::Locked => html! { <LockedIcon id="locked-tab-logo" class="tab-logo-elem" /> },
        AppMenu::Search => html! { <SearchIcon id="search-tab-logo" class="tab-logo-elem" /> },
    }
}
