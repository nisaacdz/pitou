use std::rc::Rc;

use pitou_core::{frontend::ApplicationContext, AppMenu, PitouFile};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_interval;

#[derive(Properties, PartialEq)]
pub struct RibbonProps {
    pub navigate_folder: Callback<bool>,
    pub reload: Callback<()>,
    pub onupdatedir: Callback<Option<Rc<PitouFile>>>,
}

#[function_component]
pub fn Ribbon(props: &RibbonProps) -> Html {
    html! {
        <div id="ribbon">
            <RibbonNav navigate={ props.navigate_folder.clone() }/>
            <RibbonClipboard reload={ props.reload.clone() }/>
            <RibbonCreations />
            <RibbonTrash reload={ props.reload.clone() }/>
            <RibbonActions onupdatedir={ props.onupdatedir.clone() }/>
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
        use_interval(
            move || {
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
            },
            500,
        );
    }

    let onclick = {
        let refreshing = refreshing.clone();
        let ctx = ctx.clone();
        let reload = props.reload.clone();
        move |_| {
            ctx.static_data.clear_all_selections();
            match ctx.current_menu() {
                AppMenu::Home => {
                    ctx.static_data.reset_drives();
                    ctx.static_data.reset_gen_dirs();
                }
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

    let refresh_class = format! {"ribbon-large {}", "active"};

    let img = if *refreshing {
        html! { <img src="./public/refresh_anim.gif"/> }
    } else {
        html! { <img src="./public/refresh.png"/> }
    };

    html! {
        <div id="ribbon-refresh" class="ribbon-group">
            <div class={ refresh_class } title="refresh" {onclick}>
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

#[derive(Properties, PartialEq)]
struct RibbonTrashProps {
    reload: Callback<()>,
}

#[function_component]
fn RibbonTrash(props: &RibbonTrashProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let can_delete = use_state_eq(|| false);
    {
        let ctx = ctx.clone();
        let can_delete = can_delete.clone();
        use_interval(
            move || can_delete.set(ctx.static_data.can_attempt_delete()),
            500,
        )
    }

    let ondelete = {
        let ctx = ctx.clone();
        let reload = props.reload.clone();
        move |_| {
            if let Some(items) = ctx.static_data.folder_entry_selections() {
                let reload = reload.clone();
                spawn_local(async move {
                    crate::app::cmds::delete(&items).await.ok();
                    reload.emit(())
                })
            }
        }
    };
    let delete_class = format! {"ribbon-large {}", if *can_delete { "active" } else { "inactive" }};

    html! {
        <div id="ribbon-trash" class="ribbon-group">
            <div class={delete_class} title="delete" onclick={ondelete}>
                <img src="./public/delete.png"/>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct RibbonActionsProps {
    onupdatedir: Callback<Option<Rc<PitouFile>>>,
}

#[function_component]
fn RibbonActions(props: &RibbonActionsProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();

    let onopen = {
        let onupdatedir = props.onupdatedir.clone();
        let ctx = ctx.clone();
        move |_| {
            if let Some(pf) = ctx.static_data.openable_selection() {
                if pf.is_file() {
                    spawn_local(async move {
                        crate::app::cmds::open(pf).await.ok();
                    })
                } else if pf.is_link() {
                    //TODO
                } else {
                    ctx.active_tab.update_cur_menu(AppMenu::Explorer);
                    onupdatedir.emit(Some(pf))
                }
            }
        }
    };

    let onopenwith = {
        let ctx = ctx.clone();
        move |_| {
            if let Some(pitou) = ctx.static_data.openable_selection() {
                spawn_local(async move {
                    crate::app::cmds::open_with(pitou).await.ok();
                })
            }
        }
    };

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
                <div class="ribbon-small" onclick={ onopen }>{"open"}</div>
                <div class="ribbon-small" onclick={ onopenwith }>{"open with"}</div>
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
                navigate.emit(true);
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

    let forward_class =
        format! {"ribbon-nav-item {}", if can_nav_forward { "active" } else { "inactive" }};
    let backward_class =
        format! {"ribbon-nav-item {}", if can_nav_backward { "active" } else { "inactive" }};

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

#[derive(PartialEq, Properties)]
pub struct RibbonClipboardProps {
    reload: Callback<()>,
}

#[function_component]
fn RibbonClipboard(props: &RibbonClipboardProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let can_paste = use_state_eq(|| false);
    {
        let can_paste = can_paste.clone();
        use_interval(
            move || {
                let can_paste = can_paste.clone();
                spawn_local(async move {
                    let res = crate::app::cmds::clipboard_empty().await.ok();
                    can_paste.set(!res.unwrap_or(true));
                })
            },
            500,
        )
    }

    let oncopy = {
        let ctx = ctx.clone();
        move |_| {
            let ctx = ctx.clone();
            spawn_local(async move {
                if let Some(items) = &ctx.static_data.folder_entry_selections() {
                    let _res = crate::app::cmds::copy(items).await.ok();
                }
            });
        }
    };

    let oncut = {
        let ctx = ctx.clone();
        move |_| {
            let ctx = ctx.clone();
            spawn_local(async move {
                if let Some(items) = &ctx.static_data.folder_entry_selections() {
                    let _res = crate::app::cmds::cut(items).await.ok();
                }
            });
        }
    };

    let onpaste = {
        let ctx = ctx.clone();
        let reload = props.reload.clone();
        move |_| {
            if let Some(pitou) = ctx.active_tab.current_dir() {
                let reload = reload.clone();
                spawn_local(async move {
                    let _res = crate::app::cmds::paste(pitou).await.ok();
                    reload.emit(())
                });
            }
        }
    };

    let paste_class = format! {"ribbon-large pasteable{}", if *can_paste { " active" } else { "" }};
    html! {
        <div id="ribbon-clipboard" class="ribbon-group">
            <div class={paste_class} title="paste" onclick={onpaste}>
                <img src="./public/paste.png"/>
            </div>
            <div class="ribbon-medium-group">
                <div class="ribbon-medium" title="copy" onclick={oncopy}>
                    <img class="ribbon-clipboard-medium-ico" src="./public/copy.png"/>
                </div>
                <div class="ribbon-medium" title="cut" onclick={oncut}>
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
