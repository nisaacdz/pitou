use yew::prelude::*;

#[function_component]
pub fn Ribbon() -> Html {
    html! {
        <div id="ribbon">
            <RibbonNav />
            <RibbonClipboard />
            <RibbonCreations />
            <RibbonTrash />
            <RibbonActions />
            <RibbonRemote />
        </div>
    }
}

#[function_component]
pub fn RibbonTrash() -> Html {
    html! {
        <div id="ribbon-trash" class="ribbon-group">
            <div class="ribbon-large" title="delete">
                <img src="./public/delete.png"/>
            </div>
        </div>
    }
}

#[function_component]
pub fn RibbonActions() -> Html {
    html! {
        <div id="ribbon-actions" class="ribbon-group">
            <div class="ribbon-clipboard-medium-group">
                <div class="ribbon-medium" title="share">
                    <img src="./public/share.png"/>
                </div>
                <div class="ribbon-medium" title="email">
                    <img src="./public/email.png" />
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn RibbonRemote() -> Html {
    html! {
        <div id="ribbon-remote" class="ribbon-group">
        </div>
    }
}

#[function_component]
fn RibbonCreations() -> Html {
    html! {
        <div id="ribbon-creations" class="ribbon-group">
            <div class="ribbon-large" title="new folder">
                <img src="./public/new_folder.png"/>
            </div>
            <div class="ribbon-large" title="archive">
                <img src="./public/archive.png"/>
            </div>
            <div class="ribbon-textgroup">
                <div class="ribbon-small">
                    <img src="./public/add.png"/>
                    {"new item"}
                </div>
                <div class="ribbon-small">
                    <img src="./public/rename.png"/>
                    {"rename"}
                </div>
                <div class="ribbon-small">
                    <img src="./public/extract.png"/>
                    {"extract"}
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn RibbonNav() -> Html {
    html! {
        <div id="ribbon-nav" class="ribbon-group">
            <div class="ribbon-nav-item" title="backward">
                <img src="./public/arrow_left.png" class="ribbon-nav-ico" />
            </div>
            <div class="ribbon-nav-item" title="forward">
                <img src="./public/arrow_right.png" class="ribbon-nav-ico" />
            </div>
        </div>
    }
}

#[function_component]
pub fn RibbonClipboard() -> Html {
    html! {
        <div id="ribbon-clipboard" class="ribbon-group">
            <div class="ribbon-large pasteable" title="paste">
                <img src="./public/paste.png"/>
            </div>
            <div class="ribbon-clipboard-medium-group">
                <div class="ribbon-medium" title="copy">
                    <img class="ribbon-clipboard-medium-ico" src="./public/copy3.png"/>
                </div>
                <div class="ribbon-medium" title="cut">
                    <img class="ribbon-medium-ico" src="./public/cut.png"/>
                </div>
            </div>
            <div class="ribbon-textgroup clipboard">
                <div class="ribbon-small clipboard">{"copy path"}</div>
                <div class="ribbon-small clipboard">{"copy shortcut"}</div>
                <div class="ribbon-small clipboard">{"clipboard"}</div>
            </div>
        </div>
    }
}
