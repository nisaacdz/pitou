use yew::prelude::*;
use std::{rc::Rc, cell::RefCell};
use pitou_core::frontend::*;

#[derive(PartialEq, Properties)]
pub struct BodyProps {
    pub active_tab: Rc<TabCtx>,
}

#[function_component]
pub fn Body(prop: &BodyProps) -> Html {
    html! {
        <main id = "content"></main>
    }
}