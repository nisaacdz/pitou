use yew::prelude::*;
use std::{rc::Rc, cell::RefCell};
use pitou_core::frontend::*;

use super::AllTabsCtx;

#[derive(PartialEq, Properties)]
pub struct TitleBarProps {
    pub tabs_ctx: AllTabsCtx,
}

#[function_component]
pub fn TitleBar(props: &TitleBarProps) -> Html {
    html! {
        <div id="title-bar">
            <AppLogo />
            <TabbedInterface tabs_ctx = { props.tabs_ctx.clone() }/>
            <ControlBox />
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct TabbedInterfaceProps {
    tabs_ctx: AllTabsCtx,
}

#[function_component]
fn TabbedInterface(_props: &TabbedInterfaceProps) -> Html {
    html! {
        <div id="tabs-container">
            <div id="all-tabs"></div>
        </div>
    }
}

#[function_component]
fn ControlBox() -> Html {
    html! {
        <div id="control-box">
            <div class="control-button">{"C"}</div>
            <div class="control-button">{"M"}</div>
            <div class="control-button">{"M"}</div>
        </div>
    }
}

#[function_component]
fn AppLogo() -> Html {
    html! {
        <div id="app-logo"> {"App Logo"} </div>
    }
}