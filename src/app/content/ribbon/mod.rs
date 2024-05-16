use std::rc::Rc;

use crate::app::reusables::{FindPop, ItemsSortPop, NewItemPop};
use pitou_core::{frontend::ApplicationContext, AppMenu, ItemsView, PitouFile, PitouFilePath};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_hooks::use_interval;

#[derive(Properties, PartialEq)]
pub struct RibbonProps {
    pub navigate_folder: Callback<bool>,
    pub reload: Callback<()>,
    pub quietreload: Callback<()>,
    pub onupdatedir: Callback<Option<Rc<PitouFile>>>,
}

#[function_component]
pub fn Ribbon(props: &RibbonProps) -> Html {
    html! {
        <div id="ribbon">
            <RibbonNav navigate={ props.navigate_folder.clone() }/>
            <RibbonClipboard reload={ props.reload.clone() }/>
            <RibbonCreations reload={ props.reload.clone() }/>
            <RibbonTrash reload={ props.reload.clone() }/>
            <RibbonActions onupdatedir={ props.onupdatedir.clone() }/>
            <RibbonRefresh quietreload={ props.quietreload.clone() }/>
            <RibbonProperties />
            <RibbonArrange quietreload={props.quietreload.clone()}/>
            <RibbonHighlight quietreload={props.quietreload.clone()} reload={props.reload.clone()}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct RibbonRefreshProps {
    quietreload: Callback<()>,
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
            250,
        );
    }

    let onclick = {
        let refreshing = refreshing.clone();
        let ctx = ctx.clone();
        let quietreload = props.quietreload.clone();
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
            quietreload.emit(())
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
            250,
        )
    }

    let ondelete = {
        let ctx = ctx.clone();
        let reload = props.reload.clone();
        move |_| {
            let reload = reload.clone();
            if let Some(items) = ctx.static_data.folder_entry_selections() {
                spawn_local(async move {
                    crate::app::cmds::delete(&items).await.ok();
                    reload.emit(());
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
                    <img src="./public/share.png"/>
                </div>
                <div class="ribbon-medium" title="email">
                    <img src="./public/email2.png" />
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
                    <img src="./public/cloud_download.png" />
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct RibbonArrangeProps {
    quietreload: Callback<()>,
}

#[function_component]
fn RibbonArrange(props: &RibbonArrangeProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let sorting = use_state_eq(|| None);

    let onclicksort = {
        let ctx = ctx.clone();
        let sorting = sorting.clone();
        move |_| {
            if let Some(_) = *sorting {
                sorting.set(None)
            } else {
                if ctx.current_menu() == AppMenu::Explorer {
                    sorting.set(Some(ctx.items_sort()))
                }
            }
        }
    };

    let onchangeitemsview = {
        let ctx = ctx.clone();
        let quietreload = props.quietreload.clone();
        move |e: Event| {
            let val = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
            let view = match val.parse::<u8>().unwrap() {
                0 => ItemsView::Tiles,
                1 => ItemsView::Grid,
                2 => ItemsView::Rows,
                _ => unreachable!(),
            };
            ctx.update_items_view(view);
            quietreload.emit(())
        }
    };

    let onclickchooseview = { move |e: MouseEvent| e.stop_propagation() };

    let onclickitemsview = {
        let ctx = ctx.clone();
        let quietreload = props.quietreload.clone();
        move |_| {
            match ctx.items_view() {
                ItemsView::Grid => ctx.update_items_view(ItemsView::Rows),
                ItemsView::Rows => ctx.update_items_view(ItemsView::Tiles),
                ItemsView::Tiles => ctx.update_items_view(ItemsView::Grid),
            }
            quietreload.emit(())
        }
    };

    let cnt = if let Some(selected) = *sorting {
        let onexit = {
            let sorting = sorting.clone();
            move |()| sorting.set(None)
        };

        let onfinish = {
            let ctx = ctx.clone();
            let quietreload = props.quietreload.clone();
            move |sort| {
                ctx.update_items_sort(sort);
                quietreload.emit(())
            }
        };

        html! { <ItemsSortPop {onfinish} {onexit} {selected} /> }
    } else {
        html! {}
    };

    let sort_class = format! {"ribbon-large {}", if ctx.current_menu() == AppMenu::Explorer { "active" } else { "not-active" }};
    let items_view = ctx.items_view();

    html! {
        <div id="ribbon-arrange" class="ribbon-group">
            {cnt}
            <div class={sort_class} title="sort" onclick={onclicksort}>
                <img src="./public/sort.png"/>
            </div>
            <div class="ribbon-textgroup">
                <div class="ribbon-small ribbon-mid-small" onclick={onclickitemsview}>
                    <div>{"files view"}</div>
                    <select onchange={onchangeitemsview} onclick={onclickchooseview}>
                    <option value="0" selected={items_view == ItemsView::Tiles}>{ "Tiles" }</option>
                    <option value="1" selected={items_view == ItemsView::Grid}>{ "Grid" }</option>
                    <option value="2" selected={items_view == ItemsView::Rows}>{ "List" }</option>
                    </select>
                </div>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct RibbonCreationsProps {
    reload: Callback<()>,
}

#[function_component]
fn RibbonCreations(props: &RibbonCreationsProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let can_archive = use_state_eq(|| false);
    let new_item = use_state_eq(|| None);
    let renamer = use_state(|| None);
    {
        let ctx = ctx.clone();
        let can_archive = can_archive.clone();
        use_interval(
            move || can_archive.set(ctx.static_data.has_folder_entry_selections()),
            250,
        )
    }

    let onarchive = {
        let ctx = ctx.clone();
        let reload = props.reload.clone();
        move |_| {
            if let Some(items) = ctx.static_data.folder_entry_selections() {
                if items.len() > 0 {
                    let reload = reload.clone();
                    spawn_local(async move {
                        crate::app::cmds::archive(&items).await.ok();
                        reload.emit(())
                    })
                }
            }
        }
    };

    let onclicknewfolder = {
        let new_item = new_item.clone();
        let ctx = ctx.clone();
        move |_| {
            if ctx.current_menu() == AppMenu::Explorer {
                if let None = *new_item {
                    new_item.set(Some(true))
                }
            }
        }
    };

    let onclickrename = {
        let renamer = renamer.clone();
        let ctx = ctx.clone();
        move |_| {
            if ctx.current_menu() == AppMenu::Explorer {
                if let Some(file) = ctx.static_data.openable_selection() {
                    if let None = *renamer {
                        renamer.set(Some(file));
                    }
                }
            }
        }
    };

    let onclicknewfile = {
        let new_item = new_item.clone();
        let ctx = ctx.clone();
        move |_| {
            if ctx.current_menu() == AppMenu::Explorer {
                if let None = *new_item {
                    new_item.set(Some(false))
                }
            }
        }
    };

    let onclickextract = {
        let ctx = ctx.clone();
        move |_| {
            if ctx.current_menu() == AppMenu::Explorer {
                if let Some(file) = ctx.static_data.openable_selection() {
                    spawn_local(async move {
                        crate::app::cmds::extract(file).await.ok();
                    });
                }
            }
        }
    };

    let cnt1 = if let Some(state) = *new_item {
        if let Some(dir) = ctx.active_tab.current_dir() {
            let prompt = if state {
                "Create new folder"
            } else {
                "Create new file"
            };
            let placeholder = if state {
                "Enter folder name..."
            } else {
                "Enter file name with extension..."
            };

            let value: Option<String> = None;
            let oncancel = {
                let new_item = new_item.clone();
                move |()| new_item.set(None)
            };

            let onfinish = {
                let dir = dir.clone();
                let new_item = new_item.clone();
                let reload = props.reload.clone();
                move |input| {
                    let new_path = dir.path().path.join(input);
                    let pf = Rc::new(PitouFile::without_metadata(PitouFilePath::from_pathbuf(
                        new_path,
                    )));
                    let reload = reload.clone();
                    let new_item = new_item.clone();
                    spawn_local(async move {
                        if state {
                            crate::app::cmds::create_dir(pf).await.ok();
                        } else {
                            crate::app::cmds::create_file(pf).await.ok();
                        }
                        new_item.set(None);
                        reload.emit(());
                    })
                }
            };
            html! {
                <NewItemPop {onfinish} {oncancel} {prompt} {placeholder} {value}/>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    };

    let cnt2 = if let Some(file) = (*renamer).clone() {
        let prompt = format! {"Renaming item: {}", file.name()};
        let placeholder: Option<String> = None;
        let value = file.name().to_owned();
        let oncancel = {
            let renamer = renamer.clone();
            move |()| renamer.set(None)
        };

        let onfinish = {
            let file = file.clone();
            let reload = props.reload.clone();
            let renamer = renamer.clone();
            move |name| {
                let pitou = file.clone();
                let reload = reload.clone();
                let renamer = renamer.clone();
                spawn_local(async move {
                    crate::app::cmds::rename(pitou, name).await.ok();
                    renamer.set(None);
                    reload.emit(());
                })
            }
        };

        html! { <NewItemPop {onfinish} {oncancel} {prompt} {placeholder} {value}/> }
    } else {
        html! {}
    };

    let new_folder_class =
        format! {"ribbon-large {}", if ctx.new_folder_able() { "active" } else { "not-active" }};

    let archive_class = format! {
        "ribbon-large {}",
        if *can_archive { "active" } else { "inactive" }
    };

    html! {
        <div id="ribbon-creations" class="ribbon-group">
            { cnt1 }
            { cnt2 }
            <div class={new_folder_class} title="new folder" onclick={onclicknewfolder}>
                <img src="./public/new_folder.png"/>
            </div>
            <div class={archive_class} title="archive" onclick={onarchive}>
                <img src="./public/archive.png"/>
            </div>
            <div class="ribbon-textgroup">
                <div class="ribbon-small" onclick={onclicknewfile}>
                    <img src="./public/add.png"/>
                    {"new item"}
                </div>
                <div class="ribbon-small" onclick={onclickrename}>
                    <img src="./public/rename.png"/>
                    {"rename"}
                </div>
                <div class="ribbon-small" onclick={onclickextract}>
                    <img src="./public/extract.png"/>
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
            250,
        )
    }

    let oncopy = {
        let ctx = ctx.clone();
        move |_| {
            let ctx = ctx.clone();
            spawn_local(async move {
                if let Some(items) = &ctx.static_data.folder_entry_selections() {
                    let _res = crate::app::cmds::copy(items).await.ok();
                } else if let Some(items) = &ctx.static_data.search_result_selections() {
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

    let oncopypath = {
        let ctx = ctx.clone();
        move |_| {
            if let Some(pitou) = ctx.static_data.openable_selection() {
                spawn_local(async move {
                    crate::app::cmds::copy_path(pitou).await.ok();
                })
            }
        }
    };

    let onpaste = {
        let ctx = ctx.clone();
        let reload = props.reload.clone();
        move |_| {
            if let Some(pitou) = ctx.active_tab.current_dir() {
                let reload = reload.clone();
                spawn_local(async move {
                    if let Some(id) = crate::app::cmds::paste(pitou).await.ok() {
                        crate::app::events::emit_event("pasting", &id).await;
                    }
                    reload.emit(())
                });
            }
        }
    };

    let paste_class =
        format! {"ribbon-large pasteable {}", if *can_paste { "active" } else { "not-active" }};
    html! {
        <div id="ribbon-clipboard" class="ribbon-group">
            <div class={paste_class} title="paste" onclick={onpaste}>
                <img src="./public/paste.png"/>
            </div>
            <div class="ribbon-medium-group">
                <div class="ribbon-medium" title="copy" onclick={oncopy}>
                    <img class="ribbon-clipboard-medium-ico" src="./public/copy2.png"/>
                </div>
                <div class="ribbon-medium" title="cut" onclick={oncut}>
                    <img class="ribbon-medium-ico" src="./public/cut.png"/>
                </div>
            </div>
            <div class="ribbon-textgroup clipboard">
                <div class="ribbon-small clipboard" onclick={oncopypath}>{"copy path"}</div>
                <div class="ribbon-small clipboard">{"paste shortcut"}</div>
                <div class="ribbon-small clipboard">{"clipboard"}</div>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct RibbonHighlightProps {
    quietreload: Callback<()>,
    reload: Callback<()>,
}

#[function_component]
fn RibbonHighlight(props: &RibbonHighlightProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let finding = use_state_eq(|| false);

    let ontoggleselection = {
        let ctx = ctx.clone();
        let quietreload = props.quietreload.clone();
        move |_| {
            if let Some(items) = ctx.active_tab.dir_children() {
                if ctx
                    .static_data
                    .are_all_selected_folder_entries(items.clone())
                {
                    items
                        .iter()
                        .for_each(|item| ctx.static_data.clear_dir_entry_selection(item.clone()));
                } else {
                    items
                        .iter()
                        .for_each(|item| ctx.static_data.select_folder_entry(item.clone()));
                }
                quietreload.emit(())
            }
        }
    };

    let oninvertselection = {
        let ctx = ctx.clone();
        let quietreload = props.quietreload.clone();
        move |_| {
            if let Some(items) = ctx.active_tab.dir_children() {
                items.iter().for_each(|item| {
                    if ctx.static_data.is_selected_dir_entry(item.clone()) {
                        ctx.static_data.clear_dir_entry_selection(item.clone())
                    } else {
                        ctx.static_data.select_folder_entry(item.clone())
                    }
                });
                quietreload.emit(())
            }
        }
    };

    let onclicksearch = {
        let finding = finding.clone();
        move |_| finding.set(!*finding)
    };

    let cnt = if *finding {
        let onchange = {
            let quietreload = props.quietreload.clone();
            move |input| {
                let quietreload = quietreload.clone();
                spawn_local(async move {
                    crate::app::events::emit_event("find", &input).await;
                    quietreload.emit(())
                })
            }
        };

        let onclose = {
            let finding = finding.clone();
            let quietreload = props.quietreload.clone();
            move |()| {
                finding.set(false);
                let quietreload = quietreload.clone();
                spawn_local(async move {
                    crate::app::events::emit_event("ended_find", &()).await;
                    quietreload.emit(())
                })
            }
        };

        html! {
            <FindPop {onchange} {onclose}/>
        }
    } else {
        html! {}
    };

    html! {
        <div id="ribbon-highlight" class="ribbon-group">
            {cnt}
            <div class="ribbon-large active" title="find" onclick={onclicksearch}>
                <img src="./public/search.png"/>
            </div>
            <div class="ribbon-medium-group">
                <div class="ribbon-medium" title="toggle selection" onclick={ontoggleselection}>
                    <img class="ribbon-clipboard-medium-ico" src="./public/toggle_selection.png"/>
                </div>
                <div class="ribbon-medium" title="invert selection" onclick={oninvertselection}>
                    <img class="ribbon-medium-ico" src="./public/invert_selection.png"/>
                </div>
            </div>
        </div>
    }
}
