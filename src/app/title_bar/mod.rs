use pitou_core::frontend::*;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

use super::AllTabsCtx;

#[derive(PartialEq, Properties)]
pub struct TitleBarProps {
    pub tabs_ctx: AllTabsCtx,
    pub onclose: Callback<()>,
    pub onmaximize: Callback<()>,
    pub onrestore: Callback<()>,
    pub onminimize: Callback<()>,
}

#[function_component]
pub fn TitleBar(props: &TitleBarProps) -> Html {
    let onclose = props.onclose.clone();
    let onrestore = props.onrestore.clone();
    let onmaximize = props.onmaximize.clone();
    let onminimize = props.onminimize.clone();
    html! {
        <div id="title-bar">
            <AppLogo />
            <TabbedInterface tabs_ctx = { props.tabs_ctx.clone() }/>
            <ControlBox {onclose} {onminimize} {onmaximize} {onrestore}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct TabbedInterfaceProps {
    tabs_ctx: AllTabsCtx,
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
            if idx == active_tab {
                html! { <ActiveTab {idx} {ctx} /> }
            } else {
                html! { <InactiveTab {idx} {ctx} /> }
            }
        })
        .chain(Some(html! { <AddTab /> }))
        .collect::<Html>();
    html! {
        <div id="tabs-container">
            <div id="all-tabs">
                {tabs_disp}
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct TabProps {
    idx: usize,
    ctx: Rc<TabCtx>,
}

#[function_component]
fn InactiveTab(prop: &TabProps) -> Html {
    html! {
        <div class = "tab inactive"> { "nisaacdz" } </div>
    }
}

#[function_component]
fn ActiveTab(prop: &TabProps) -> Html {
    html! {
        <div class = "tab active"> { "nisaacdz" } </div>
    }
}

#[function_component]
fn AddTab() -> Html {
    html! {
        <div class = "tab add-tab"> 
            <svg id = "add-tab-plus" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 23 23" width="24" height="24">
                <path d="M19 11h-6V5a1 1 0 0 0-2 0v6H5a1 1 0 0 0 0 2h6v6a1 1 0 0 0 2 0v-6h6a1 1 0 0 0 0-2z"/>
            </svg>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ControlBoxProps {
    onclose: Callback<()>,
    onmaximize: Callback<()>,
    onrestore: Callback<()>,
    onminimize: Callback<()>,
}

#[function_component]
fn ControlBox(props: &ControlBoxProps) -> Html {
    let onclose = {
        let onclose = props.onclose.clone();
        move |_| onclose.emit(())
    };

    let onresize = {
        let onrestore = props.onrestore.clone();
        move |_| onrestore.emit(())
    };

    let onminimize = {
        let onminimize = props.onminimize.clone();
        move |_| onminimize.emit(())
    };

    let maxi_or_restore = if true {
        html! {
            <svg class="elem" width="24" height="24" viewBox="-3 -4 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path class="maxi-button-line" d="M8 2H4C2.89543 2 2 2.89543 2 4V8" stroke-width="2" stroke-linecap="round"/>
                <path class="maxi-button-line" d="M22 8L22 4C22 2.89543 21.1046 2 20 2H16" stroke-width="2" stroke-linecap="round"/>
                <path class="maxi-button-line" d="M16 22L20 22C21.1046 22 22 21.1046 22 20L22 16" stroke-width="2" stroke-linecap="round"/>
                <path class="maxi-button-line" d="M8 22L4 22C2.89543 22 2 21.1046 2 20V16" stroke-width="2" stroke-linecap="round"/>
            </svg>
        }
    } else {
        html! {
            <svg class="elem" width="24" height="24" viewBox="-4 -5 30 30" version="1.1" xmlns="http://www.w3.org/2000/svg">
                <g stroke="none" stroke-width="1" fill="none" fill-rule="evenodd" stroke-linecap="round" stroke-linejoin="round">
                    <g transform="translate(-969.000000, -748.000000)" id="Group" stroke="#000000" stroke-width="2">
                        <g transform="translate(967.000000, 746.000000)" id="Shape">
                            <path class="maxi-button-line" d="M3,15 L9,15 L9,21 M15,21 L15,15 L21,15 M21,9 L15,9 L15,3 M9,3 L9,9 L3,9"></path>
                        </g>
                    </g>
                </g>
            </svg>
        }
    };

    html! {
        <div id="control-box">
            <div class="control-button close-button" onclick={onclose}>
                <svg class="elem" id="close-button-cross" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
                    <path d="M17.71 6.71a1 1 0 0 0-1.42 0L12 10.59l-4.29-4.3a1 1 0 0 0-1.42 1.42L10.59 12l-4.3 4.29a1 1 0 1 0 1.42 1.42L12 13.41l4.29 4.3a1 1 0 0 0 1.42-1.42L13.41 12l4.3-4.29a1 1 0 0 0 0-1.42z"/>
                </svg>
            </div>
            <div class="control-button maxi-button" onclick={onresize}>
                { maxi_or_restore }
            </div>
            <div class="control-button mini-button" onclick={onminimize}>
                <svg class="elem" id="mini-button-dash" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
                    <rect x="5" y="12" width="14" height="2"/>
                </svg>
            </div>
        </div>
    }
}

#[function_component]
fn AppLogo() -> Html {
    html! {
        <div id="app-logo"> {"AppLogo"} </div>
    }
}
