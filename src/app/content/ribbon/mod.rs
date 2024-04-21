use pitou_core::{frontend::ApplicationContext, AppMenu};
use yew::prelude::*;
use yew_hooks::use_interval;

#[derive(Properties, PartialEq)]
pub struct RibbonProps {
    pub navigate_folder: Callback<bool>,
    pub reload: Callback<()>,
}

#[function_component]
pub fn Ribbon(props: &RibbonProps) -> Html {
    html! {
        <div id="ribbon">
            <RibbonNav navigate={ props.navigate_folder.clone() }/>
            <RibbonClipboard />
            <RibbonCreations />
            <RibbonTrash />
            <RibbonActions />
            <RibbonRefresh reload={ props.reload.clone() }/>
            <RibbonProperties />
            <RibbonArrange />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct RibbonRefreshProps {
    reload: Callback<()>,
}


#[function_component]
fn RibbonRefresh(props: &RibbonRefreshProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let refreshing = use_state_eq(|| false);
    {
        let refreshing = refreshing.clone();
        let ctx = ctx.clone();
        use_interval(move || {
            let still_refreshing = match ctx.current_menu() {
                AppMenu::Home => ctx.static_data.drives.borrow().is_none(),
                AppMenu::Explorer => ctx.active_tab.dir_children.borrow().is_none(),
                AppMenu::Trash => ctx.static_data.no_trash_items(),
                AppMenu::Favorites => false,
                AppMenu::Search => false,
                AppMenu::Locked => false,
                AppMenu::Recents => false,
                AppMenu::Cloud => false,
                AppMenu::Settings => false,
            };
            refreshing.set(still_refreshing);
        }, 500);
    }

    let onclick = {
        let refreshing = refreshing.clone();
        let ctx = ctx.clone();
        let reload = props.reload.clone();
        move |_| {
            ctx.static_data.clear_all_selections();
            match ctx.current_menu() {
                AppMenu::Home => ctx.static_data.reset_drives(),
                AppMenu::Explorer => ctx.active_tab.reset_current_files(),
                AppMenu::Trash => ctx.static_data.reset_trash_items(),
                AppMenu::Favorites => (),
                AppMenu::Search => (),
                AppMenu::Locked => (),
                AppMenu::Recents => (),
                AppMenu::Cloud => (),
                AppMenu::Settings => (),
            }
            refreshing.set(true);
            reload.emit(())
        }
    };

    let img = if *refreshing {
        html! { <img src="./public/refresh_anim.gif"/> }
    } else {
        html! { <img src="./public/refresh.png"/> }
    };

    html! {
        <div id="ribbon-refresh" class="ribbon-group">
            <div class="ribbon-large" title="refresh" {onclick}>
                { img }
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

#[derive(Properties, PartialEq)]
struct RibbonNavProps {
    navigate: Callback<bool>,
}

#[function_component]
fn RibbonNav(props: &RibbonNavProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    
    let onclick_forward = {
        let navigate = props.navigate.clone();
        let ctx = ctx.clone();
        move |_| {
            if ctx.current_menu() == AppMenu::Explorer {
                navigate.emit(true)
            }
        }
    };

    let onclick_backward = {
        let navigate = props.navigate.clone();
        let ctx = ctx.clone();
        move |_| {
            if ctx.current_menu() == AppMenu::Explorer {
                navigate.emit(false)
            }
        }
    };

    let cur_theme = ctx.current_menu();
    let can_nav_forward = cur_theme == AppMenu::Explorer && ctx.active_tab.can_navigate_forward();
    let can_nav_backward = cur_theme == AppMenu::Explorer && ctx.active_tab.can_navigate_backward(); 

    let forward_class = format!{"ribbon-nav-item {}", if can_nav_forward { "active" } else { "inactive" }};
    let backward_class = format!{"ribbon-nav-item {}", if can_nav_backward { "active" } else { "inactive" }};

    html! {
        <div id="ribbon-nav" class="ribbon-group">
            <div class={forward_class} title="forward" onclick={onclick_forward} >
                <img src="./public/arrow_right.png" class="ribbon-nav-ico" />
            </div>
            <div class={backward_class} title="backward" onclick={onclick_backward}>
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
