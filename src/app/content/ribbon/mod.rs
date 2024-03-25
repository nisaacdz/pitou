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
            <RibbonRefresh />
            <RibbonProperties />
            <RibbonArrange />
        </div>
    }
}

#[function_component]
fn RibbonRefresh() -> Html {
    html! {
        <div id="ribbon-refresh" class="ribbon-group">
            <div class="ribbon-large" title="refresh">
                <img src="./public/refresh.png"/>
            </div>
        </div>
    }
}

#[function_component]
fn RibbonProperties() -> Html {
    html! {
        <div id="ribbon-properties" class="ribbon-group">
            <div class="ribbon-large" title="properties">
                <img src="./public/properties.png"/>
            </div>
        </div>
    }
}

#[function_component]
fn RibbonTrash() -> Html {
    html! {
        <div id="ribbon-trash" class="ribbon-group">
            <div class="ribbon-large" title="delete">
                <img src="./public/delete.png"/>
            </div>
        </div>
    }
}

#[function_component]
fn RibbonActions() -> Html {
    html! {
        <div id="ribbon-actions" class="ribbon-group">
            <div class="ribbon-medium-group">
                <div class="ribbon-medium" title="share">
                    <img src="./public/share2.png"/>
                </div>
                <div class="ribbon-medium" title="email">
                    <img src="./public/email.png" />
                </div>
            </div>
            <div class="ribbon-textgroup">
                <div class="ribbon-small">{"open"}</div>
                <div class="ribbon-small">{"open with"}</div>
            </div>
            <div class="ribbon-medium-group">
                <div class="ribbon-medium" title="pin">
                    <img src="./public/pin.png"/>
                </div>
                <div class="ribbon-medium" title="lock">
                    <img src="./public/locked.png" />
                </div>
            </div>
            <div class="ribbon-medium-group">
                <div class="ribbon-medium" title="upload">
                    <img src="./public/cloud_upload.png"/>
                </div>
                <div class="ribbon-medium" title="download">
                    <img src="./public/cloud_download2.png" />
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn RibbonArrange() -> Html {
    html! {
        <div id="ribbon-arrange" class="ribbon-group">
            <div class="ribbon-large" title="sort">
                <img src="./public/sort2.png"/>
            </div>
            <div class="ribbon-textgroup">
                <div class="ribbon-small">{"files view"}</div>
            </div>
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
                    <img src="./public/rename3.png"/>
                    {"rename"}
                </div>
                <div class="ribbon-small">
                    <img src="./public/extract1.png"/>
                    {"extract"}
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn RibbonNav() -> Html {
    html! {
        <div id="ribbon-nav" class="ribbon-group">
            <div class="ribbon-nav-item" title="forward">
                <img src="./public/arrow_right.png" class="ribbon-nav-ico" />
            </div>
            <div class="ribbon-nav-item" title="backward">
                <img src="./public/arrow_left.png" class="ribbon-nav-ico" />
            </div>
        </div>
    }
}

#[function_component]
fn RibbonClipboard() -> Html {
    html! {
        <div id="ribbon-clipboard" class="ribbon-group">
            <div class="ribbon-large pasteable" title="paste">
                <img src="./public/paste.png"/>
            </div>
            <div class="ribbon-medium-group">
                <div class="ribbon-medium" title="copy">
                    <img class="ribbon-clipboard-medium-ico" src="./public/copy.png"/>
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
