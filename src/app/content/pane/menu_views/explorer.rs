use yew::prelude::*;

#[function_component]
pub fn Ancestry() -> Html {
    html! {
        <div id="ancestry">
        
        </div>
    }
}

#[function_component]
pub fn ExplorerView() -> Html {
    html! {
        <>
            <Ancestry />
            <Explorer />
        </>
    }
}

#[function_component]
pub fn Explorer() -> Html {
    html! {
        <div id="pane" class="explorer-pane fullpane"></div>
    }
}