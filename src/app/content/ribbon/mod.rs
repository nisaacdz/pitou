use yew::prelude::*;

#[function_component]
pub fn Ribbon() -> Html {
    html! {
        <div id="ribbon">
            <RibbonNav />
            <RibbonClipboard />
        </div>
    }
}

#[function_component]
pub fn RibbonNav() -> Html {
    html! {
        <div id="ribbon-nav" class="ribbon-group">
            <div class="ribbon-nav-item">
                <img src="./public/arrow_left.png" class="ribbon-nav-ico"/>
            </div>
            <div class="ribbon-nav-item">
                <img src="./public/arrow_right.png" class="ribbon-nav-ico"/>
            </div>
        </div>
    }
}

#[function_component]
pub fn RibbonClipboard() -> Html {
    html! {
        <div id="ribbon-clipboard" class="ribbon-group">
            <div class="ribbon-clipboard-large pasteable" title="paste">
                <img class="ribbon-clipboard-large-ico" src="./public/paste.png"/>
            </div>
            <div class="ribbon-clipboard-copy-cut">
                <div class="ribbon-clipboard-medium" title="copy">
                    <img class="ribbon-clipboard-medium-ico" src="./public/copy3.png"/>
                </div>
                <div class="ribbon-clipboard-medium" title="cut">
                    <img class="ribbon-clipboard-medium-ico" src="./public/cut.png"/>
                </div>
            </div>
            <div class="ribbon-clipboard-textgroup">
                <div class="ribbon-clipboard-small">{"copy path"}</div>
                <div class="ribbon-clipboard-small">{"copy shortcut"}</div>
                <div class="ribbon-clipboard-small">{"clipboard"}</div>
            </div>
        </div>
    }
}
