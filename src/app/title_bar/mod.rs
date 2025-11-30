use pitou_core::{frontend::*, *};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::app::reusables::DBChevronDownIcon;

/// Material Icon component that uses Google Material Icons font
#[derive(Properties, PartialEq)]
pub struct MatIconProps {
    pub name: AttrValue,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn MatIcon(props: &MatIconProps) -> Html {
    let class = classes!("material-icons", props.class.clone());
    html! {
        <span {class}>{ &props.name }</span>
    }
}

#[derive(Properties)]
pub struct TitleBarProps {
    pub tabs_ctx: Rc<AllTabsCtx>,
    pub onclose: Callback<()>,
    pub ontogglemaximize: Callback<()>,
    pub onminimize: Callback<()>,
    pub add_tab: Callback<()>,
    pub rem_tab: Callback<usize>,
    pub change_tab: Callback<usize>,
}

impl PartialEq for TitleBarProps {
    fn eq(&self, _other: &Self) -> bool {
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

    html! {
        <div id="title-bar" data-tauri-drag-region = "true" data-tauri-titlebar="true">
            <AppLogo />
            <TabbedInterface tabs_ctx = {tabs_ctx} {add_tab} {rem_tab} {change_tab} />
            <ControlBox {onclose} {onminimize} {ontogglemaximize} />
        </div>
    }
}

#[derive(Properties)]
struct TabbedInterfaceProps {
    tabs_ctx: Rc<AllTabsCtx>,
    add_tab: Callback<()>,
    rem_tab: Callback<usize>,
    change_tab: Callback<usize>,
}

impl PartialEq for TabbedInterfaceProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[function_component]
fn TabbedInterface(props: &TabbedInterfaceProps) -> Html {
    let AllTabsCtx {
        all_tabs,
        active_tab,
    } = &*props.tabs_ctx;
    let tabs_disp = all_tabs
        .borrow()
        .iter()
        .enumerate()
        .map(|(idx, ctx)| {
            let ctx = ctx.clone();
            let rem = {
                let rem_tab = props.rem_tab.clone();
                move |()| rem_tab.emit(idx)
            };
            let set = {
                let change_tab = props.change_tab.clone();
                move |()| change_tab.emit(idx)
            };

            if idx == *active_tab {
                html! { <ActiveTab {ctx} {rem} {set} /> }
            } else {
                html! { <InactiveTab {ctx} {rem} {set} /> }
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
    ctx: Rc<TabCtx>,
    rem: Callback<()>,
    set: Callback<()>,
}

impl PartialEq for TabProps {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[function_component]
fn InactiveTab(props: &TabProps) -> Html {
    let onclose = {
        let rem = props.rem.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            rem.emit(())
        }
    };

    let onchange = {
        let set = props.set.clone();
        move |_| set.emit(())
    };

    let name = props.ctx.display_name();

    html! {
        <div class = "tab inactive" onclick = {onchange}>
            <div class="tab-logo">
                <TabLogo menu = { *props.ctx.current_menu.borrow() } />
            </div>
            <div class="tab-text">{ name }</div>
            <div class="tab-close" onclick = {onclose}>
                <MatIcon name="close" class="tab-close-icon" />
            </div>
        </div>
    }
}

#[function_component]
fn ActiveTab(props: &TabProps) -> Html {
    let onclose = {
        let rem = props.rem.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            rem.emit(())
        }
    };

    let name = props.ctx.display_name();

    html! {
        <div class = "tab active">
            <div class="tab-logo" data-tauri-drag-region = "true">
                <TabLogo menu = { *props.ctx.current_menu.borrow() } />
            </div>
            <div class="tab-text" data-tauri-drag-region = "true">{ name }</div>
            <div class="tab-close" onclick={onclose}>
                <MatIcon name="close" class="tab-close-icon" />
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
            <MatIcon name="add" />
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
            250,
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

    let maxi_icon = if *is_maximized {
        "fullscreen_exit"
    } else {
        "fullscreen"
    };

    html! {
        <div id="control-box">
            <div class="control-button close-button" onclick={onclose}>
                <MatIcon name="close" />
            </div>
            <div class="control-button maxi-button" onclick={onresize}>
                <MatIcon name={maxi_icon} />
            </div>
            <div class="control-button mini-button" onclick={onminimize}>
                <MatIcon name="remove" />
            </div>
        </div>
    }
}

#[function_component]
fn AppLogo() -> Html {
    html! {
        <div id="app-logo" data-tauri-drag-region = "true" >
            <img id="app-logo-img" src = "./public/pitou_logo.png" />
            <div id="app-logo-name">{ "pitou" }</div>
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
    let icon_name = match prop.menu {
        AppMenu::Home => "home",
        AppMenu::Explorer => "folder_open",
        AppMenu::Trash => "delete",
        AppMenu::Favorites => "star",
        AppMenu::Recents => "history",
        AppMenu::Cloud => "cloud",
        AppMenu::Settings => "settings",
        AppMenu::Locked => "lock",
        AppMenu::Search => "search",
    };
    html! { <MatIcon name={icon_name} class="tab-logo-elem" /> }
}
