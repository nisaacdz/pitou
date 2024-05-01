use std::rc::Rc;

use pitou_core::{frontend::*, *};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

use crate::app::reusables::{ListFileTypeIcon, TileFileTypeIcon};

#[derive(Properties, PartialEq)]
pub struct FindPopProps {
    pub onclose: Callback<()>,
    pub onchange: Callback<String>,
}

#[function_component]
pub fn FindPop(props: &FindPopProps) -> Html {
    let input_ref = use_node_ref();
    {
        let input_ref = input_ref.clone();
        use_effect_with((), move |()| {
            let elem = input_ref.cast::<HtmlInputElement>().unwrap();
            elem.focus().ok();
        })
    }
    let onchange = {
        let onchange = props.onchange.clone();
        move |e: Event| {
            let elem = e.target_dyn_into::<HtmlInputElement>().unwrap();
            onchange.emit(elem.value())
        }
    };
    let onclose = {
        let onclose = props.onclose.clone();
        move |_| onclose.emit(())
    };

    html! {
        <div class="find-popup">
            <div class="find-popup-logo">
                <img src="./public/search.png"/>
            </div>
            <input type="text" {onchange} class="find-popup-input" ref={input_ref}/>
            <div class="find-popup-nav">
                <svg viewBox="0 0 24 24">
                    <path d="M7.33199 7.68464C6.94146 8.07517 6.3083 8.07517 5.91777 7.68464C5.52725 7.29412 5.52725 6.66095 5.91777 6.27043L10.5834 1.60483C11.3644 0.823781 12.6308 0.82378 13.4118 1.60483L18.0802 6.27327C18.4707 6.66379 18.4707 7.29696 18.0802 7.68748C17.6897 8.078 17.0565 8.078 16.666 7.68748L13 4.02145V21.9999C13 22.5522 12.5523 22.9999 12 22.9999C11.4477 22.9999 11 22.5522 11 21.9999V4.01666L7.33199 7.68464Z"/>
                </svg>
            </div>
            <div class="find-popup-nav">
                <svg viewBox="0 0 24 24">
                    <path d="M7.33199 16.3154C6.94146 15.9248 6.3083 15.9248 5.91777 16.3154C5.52725 16.7059 5.52725 17.339 5.91777 17.7296L10.5834 22.3952C11.3644 23.1762 12.6308 23.1762 13.4118 22.3952L18.0802 17.7267C18.4707 17.3362 18.4707 16.703 18.0802 16.3125C17.6897 15.922 17.0565 15.922 16.666 16.3125L13 19.9786V2.0001C13 1.44781 12.5523 1.0001 12 1.0001C11.4477 1.0001 11 1.44781 11 2.0001V19.9833L7.33199 16.3154Z"/>
                </svg>
            </div>
            <div class="find-popup-close" onclick={onclose}>
                <svg viewBox="0 0 24 24">
                    <path d="M17.71 6.71a1 1 0 0 0-1.42 0L12 10.59l-4.29-4.3a1 1 0 0 0-1.42 1.42L10.59
                    12l-4.3 4.29a1 1 0 1 0 1.42 1.42L12 13.41l4.29 4.3a1 1 0 0 0 1.42-1.42L13.41 12l4.3-4.29a1 
                    1 0 0 0 0-1.42z"/>
                </svg>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ItemsSortPopProps {
    pub selected: Option<PitouFileSort>,
    pub onfinish: Callback<Option<PitouFileSort>>,
    pub onexit: Callback<()>,
}

#[function_component]
pub fn ItemsSortPop(props: &ItemsSortPopProps) -> Html {
    let onclickfreearea = {
        let onexit = props.onexit.clone();
        move |_| onexit.emit(())
    };

    let onclicknone = {
        let onfinish = props.onfinish.clone();
        move |m: MouseEvent| {
            m.stop_propagation();
            onfinish.emit(None)
        }
    };

    let onclickcreatedinc = {
        let onfinish = props.onfinish.clone();
        move |m: MouseEvent| {
            m.stop_propagation();
            onfinish.emit(Some(PitouFileSort::DateCreated(
                PitouFileSortOrder::Increasing,
            )))
        }
    };

    let onclickcreateddec = {
        let onfinish = props.onfinish.clone();
        move |m: MouseEvent| {
            m.stop_propagation();
            onfinish.emit(Some(PitouFileSort::DateCreated(
                PitouFileSortOrder::Decreasing,
            )))
        }
    };

    let onclickmodifiedinc = {
        let onfinish = props.onfinish.clone();
        move |m: MouseEvent| {
            m.stop_propagation();
            onfinish.emit(Some(PitouFileSort::DateModified(
                PitouFileSortOrder::Increasing,
            )))
        }
    };

    let onclickmodifieddec = {
        let onfinish = props.onfinish.clone();
        move |m: MouseEvent| {
            m.stop_propagation();
            onfinish.emit(Some(PitouFileSort::DateModified(
                PitouFileSortOrder::Decreasing,
            )))
        }
    };

    let onclickaccessedinc = {
        let onfinish = props.onfinish.clone();
        move |m: MouseEvent| {
            m.stop_propagation();
            onfinish.emit(Some(PitouFileSort::DateAccessed(
                PitouFileSortOrder::Increasing,
            )))
        }
    };

    let onclickaccesseddec = {
        let onfinish = props.onfinish.clone();
        move |m: MouseEvent| {
            m.stop_propagation();
            onfinish.emit(Some(PitouFileSort::DateAccessed(
                PitouFileSortOrder::Decreasing,
            )))
        }
    };

    let onclicknameinc = {
        let onfinish = props.onfinish.clone();
        move |m: MouseEvent| {
            m.stop_propagation();
            onfinish.emit(Some(PitouFileSort::Name(PitouFileSortOrder::Increasing)))
        }
    };

    let onclicknamedec = {
        let onfinish = props.onfinish.clone();
        move |m: MouseEvent| {
            m.stop_propagation();
            onfinish.emit(Some(PitouFileSort::Name(PitouFileSortOrder::Decreasing)))
        }
    };

    html! {
        <ul class="sort-popup" onclick={onclickfreearea}>
            <li class="sort-popup-item" onclick={onclicknone}>
                <label>{ "None" }</label>
            </li>
            <li class="sort-popup-item" onclick={onclickcreateddec.clone()}>
                <label>{ "Date Created" }</label>
                <ul class="sort-popup-sub">
                    <li class="sort-popup-sub-item" onclick={onclickcreatedinc}>
                        <label>{ "Earliest First" }</label>
                    </li>
                    <li class="sort-popup-sub-item" onclick={onclickcreateddec}>
                        <label>{ "Latest First" }</label>
                    </li>
                </ul>
            </li>
            <li class="sort-popup-item" onclick={onclickmodifieddec.clone()}>
                <label>{ "Date Modified" }</label>
                <ul class="sort-popup-sub">
                    <li class="sort-popup-sub-item" onclick={onclickmodifiedinc}>
                        <label>{ "Earliest First" }</label>
                    </li>
                    <li class="sort-popup-sub-item" onclick={onclickmodifieddec}>
                        <label>{ "Latest First" }</label>
                    </li>
                </ul>
            </li>
            <li class="sort-popup-item" onclick={onclickaccesseddec.clone()}>
                <label>{ "Date Accessed" }</label>
                <ul class="sort-popup-sub">
                    <li class="sort-popup-sub-item" onclick={onclickaccessedinc}>
                        <label>{ "Earliest First" }</label>
                    </li>
                    <li class="sort-popup-sub-item" onclick={onclickaccesseddec}>
                        <label>{ "Latest First" }</label>
                    </li>
                </ul>
            </li>
            <li class="sort-popup-item" onclick={onclicknameinc.clone()}>
                <label>{ "Name" }</label>
                <ul class="sort-popup-sub">
                    <li class="sort-popup-sub-item" onclick={onclicknameinc}>
                        <label>{ "Increasing" }</label>
                    </li>
                    <li class="sort-popup-sub-item" onclick={onclicknamedec}>
                        <label>{ "Decreasing" }</label>
                    </li>
                </ul>
            </li>
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct NewItemPopProps {
    pub prompt: String,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub onfinish: Callback<String>,
    pub oncancel: Callback<()>,
}

#[function_component]
pub fn NewItemPop(props: &NewItemPopProps) -> Html {
    let entire_ref = use_node_ref();
    let input_ref = use_node_ref();

    {
        let entire_ref = entire_ref.clone();
        let input_ref = input_ref.clone();
        use_effect_with((), move |()| {
            let elem = entire_ref.cast::<HtmlElement>().unwrap();
            let input = input_ref.cast::<HtmlElement>().unwrap();
            elem.set_draggable(true);
            input.focus().ok();
        })
    }

    let onkeypress = {
        let finish = props.onfinish.clone();
        let input_ref = input_ref.clone();
        move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                if let Some(input) = input_ref.cast::<HtmlInputElement>().map(|v| v.value()) {
                    if input.len() > 0 {
                        finish.emit(input)
                    }
                }
            }
        }
    };

    let oncancel = {
        let cancel = props.oncancel.clone();
        move |_| cancel.emit(())
    };

    let oncreate = {
        let finish = props.onfinish.clone();
        let input_ref = input_ref.clone();
        move |_| {
            let input = input_ref.cast::<HtmlInputElement>().unwrap().value();
            finish.emit(input)
        }
    };

    html! {
        <div class="new-item" ref={entire_ref}>
            <label class="new-item-member prompt"> { &props.prompt } </label>
            <input placeholder={props.placeholder.clone()} value={props.value.clone()} class="new-item-member" type="text" {onkeypress} ref={input_ref} class="new-item-member"/>
            <div class="new-item-member">
                <input type="checkbox"/>
                <label>{"Override Existing"}</label>
            </div>
            <div class="new-item-member">
                <button class="new-item-member-btn" onclick={oncancel}> { "Cancel" } </button>
                <button class="new-item-member-btn" onclick={oncreate}> { "Create" } </button>
            </div>
        </div>
    }
}

#[derive(Properties)]
pub struct MainPaneProps {
    pub onopen: Callback<Rc<PitouFile>>,
    pub view: ItemsView,
    pub items: Rc<Vec<Rc<PitouFile>>>,
    pub reload: Callback<()>,
    pub quietreload: Callback<()>,
}

impl PartialEq for MainPaneProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[function_component]
pub fn MainPane(props: &MainPaneProps) -> Html {
    let items = props.items.clone();
    let onopen = props.onopen.clone();
    let reload = props.reload.clone();
    let quietreload = props.quietreload.clone();
    match props.view {
        ItemsView::Grid => html! { <GridView {items} {onopen} {reload} {quietreload}/> },
        ItemsView::Rows => html! { <ListView {items} {onopen} {reload} {quietreload}/> },
        ItemsView::Tiles => html! { <TileView {items} {onopen} {reload} {quietreload}/> },
    }
}

#[derive(Properties)]
struct ViewProps {
    onopen: Callback<Rc<PitouFile>>,
    items: Rc<Vec<Rc<PitouFile>>>,
    reload: Callback<()>,
    quietreload: Callback<()>,
}

#[derive(Properties, PartialEq)]
struct ItemProps {
    onopen: Callback<Rc<PitouFile>>,
    item: Rc<PitouFile>,
    reload: Callback<()>,
    quietreload: Callback<()>,
}

impl PartialEq for ViewProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[function_component]
fn ListView(props: &ViewProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();

    let content = props
        .items
        .iter()
        .map(|v| html! { <ListItem item = {v.clone()} onopen = {props.onopen.clone()} reload={props.reload.clone()} quietreload={props.quietreload.clone()}/> })
        .collect::<Html>();

    let ontoggleselectall = {
        let ctx = ctx.clone();
        let items = props.items.clone();
        let quietreload = props.quietreload.clone();
        move |()| {
            if ctx
                .static_data
                .are_all_selected_folder_entries(items.clone())
            {
                items
                    .iter()
                    .for_each(|item| ctx.static_data.clear_dir_entry_selection(item.clone()))
            } else {
                items
                    .iter()
                    .for_each(|item| ctx.static_data.select_folder_entry(item.clone()))
            }
            quietreload.emit(())
        }
    };

    html! {
        <>
            <ListDsc ontoggle={ontoggleselectall}/>
            <div id="pane-list-view">
                { content }
            </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct ListDscProps {
    ontoggle: Callback<()>,
}

#[function_component]
fn ListDsc(props: &ListDscProps) -> Html {
    let onchange = {
        let ontoggle = props.ontoggle.clone();
        move |_| ontoggle.emit(())
    };
    html! {
        <div id="pane-list-view-dsc">
            <div class="pane-list-view-dsc-checkbox-container">
                <input class="pane-list-view-dsc-checkbox" type="checkbox" {onchange}/>
            </div>
            <div class="pane-list-view-dsc-filetype">
                { "ico" }
            </div>
            <div class="pane-list-view-dsc-filename">
                { "filename" }
            </div>
            <div class="list-modifieddate-container">
                { "Last modified" }
            </div>
            <div class="list-accesseddate-container">
                { "Last accessed" }
            </div>
            <div class="list-createddate-container">
                { "Created On" }
            </div>
        </div>
    }
}

#[function_component]
fn ListItem(props: &ItemProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let highlighted = use_state_eq(|| ctx.static_data.is_selected_dir_entry(props.item.clone()));

    {
        let ctx = ctx.clone();
        let item = props.item.clone();
        let highlighted = highlighted.clone();
        use_effect_with(ctx.refresher_state(), move |_| {
            highlighted.set(ctx.static_data.is_selected_dir_entry(item.clone()));
        })
    }

    let onclick = {
        let highlighted = highlighted.clone();
        let item = props.item.clone();
        let ctx = ctx.clone();
        move |_| {
            if !*highlighted {
                ctx.static_data.select_folder_entry(item.clone());
                highlighted.set(true)
            } else {
                ctx.static_data.clear_dir_entry_selection(item.clone());
                highlighted.set(false)
            }
        }
    };

    let onchange = {
        let highlighted = highlighted.clone();
        let ctx = ctx.clone();
        let item = props.item.clone();
        move |e: Event| {
            e.stop_propagation();
            if !*highlighted {
                ctx.static_data.select_folder_entry(item.clone());
                highlighted.set(true)
            } else {
                ctx.static_data.clear_dir_entry_selection(item.clone());
                highlighted.set(false)
            }
        }
    };

    let ondblclick = {
        let item = props.item.clone();
        let onopen = props.onopen.clone();
        move |_| onopen.emit(item.clone())
    };

    let list_item_class = format!(
        "list-item {}",
        if *highlighted {
            "selected"
        } else {
            "not-selected"
        }
    );

    let name = if ctx.hide_system_files() {
        props.item.name()
    } else {
        props.item.name_without_extension()
    };
    let filetype = props.item.metadata.as_ref().map(|v| v.kind);
    let accessed = props
        .item
        .metadata
        .as_ref()
        .map(|v| v.accessed.datetime.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default();

    let modified = props
        .item
        .metadata
        .as_ref()
        .map(|v| v.modified.datetime.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default();

    let created = props
        .item
        .metadata
        .as_ref()
        .map(|v| v.created.datetime.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default();

    html! {
        <div class={list_item_class} {ondblclick} {onclick}>
            <div class="list-checkbox-container">
                <input class="explorer-checkbox" type="checkbox" checked={*highlighted} {onchange} />
            </div>
            <div class="list-filetypeicon-container">
                <ListFileTypeIcon {filetype}/>
            </div>
            <div class="list-filename-container">
                <div class="list-filename">{ name }</div>
            </div>
            <div class="list-modifieddate-container">
                <div>{ modified }</div>
            </div>
            <div class="list-accesseddate-container">
                <div>{ accessed }</div>
            </div>
            <div class="list-createddate-container">
                <div>{ created }</div>
            </div>
        </div>
    }
}

#[function_component]
fn TileView(props: &ViewProps) -> Html {
    let content = props
        .items
        .iter()
        .map(|v| html! { <TileItem item = {v.clone()} onopen = {props.onopen.clone()} reload={props.reload.clone()} quietreload={props.quietreload.clone()}/> })
        .collect::<Html>();

    html! {
        <div id="pane-tile-view">
            { content }
        </div>
    }
}

#[function_component]
fn TileItem(props: &ItemProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let highlighted = use_state_eq(|| ctx.static_data.is_selected_dir_entry(props.item.clone()));

    {
        let ctx = ctx.clone();
        let item = props.item.clone();
        let highlighted = highlighted.clone();
        use_effect_with(ctx.refresher_state(), move |_| {
            highlighted.set(ctx.static_data.is_selected_dir_entry(item.clone()));
        })
    }

    let ondblclick = {
        let item = props.item.clone();
        let onopen = props.onopen.clone();
        move |_| onopen.emit(item.clone())
    };

    let onclick = {
        let highlighted = highlighted.clone();
        let item = props.item.clone();
        let ctx = ctx.clone();
        move |_| {
            if !*highlighted {
                ctx.static_data.select_folder_entry(item.clone());
                highlighted.set(true)
            } else {
                ctx.static_data.clear_dir_entry_selection(item.clone());
                highlighted.set(false)
            }
        }
    };

    let tile_item_class = format!(
        "tile-item {}",
        if *highlighted {
            "selected"
        } else {
            "not-selected"
        }
    );

    let onclickcheckbox = {
        let highlighted = highlighted.clone();
        let ctx = ctx.clone();
        let item = props.item.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            if !*highlighted {
                ctx.static_data.select_folder_entry(item.clone());
                highlighted.set(true)
            } else {
                ctx.static_data.clear_dir_entry_selection(item.clone());
                highlighted.set(false)
            }
        }
    };

    let filesize = props.item.metadata.as_ref().map(|v| {
        if v.is_dir() {
            v.size.format_as_dir_entries()
        } else {
            v.size.format()
        }
    });

    let filetype = props.item.metadata.as_ref().map(|v| v.kind);

    let optional = props.item.metadata.as_ref().map(|v| {
        v.modified
            .datetime
            .format("Modified on %Y-%m-%d %H:%M")
            .to_string()
    });

    let name = if ctx.show_extensions() {
        props.item.name()
    } else {
        props.item.name_without_extension()
    };

    let description = html! {
        <div class="tile-description">
            <div class="tile-filename">
            { name }
            </div>
            <div class="tile-filesize">
                { filesize }
            </div>
            <div class="tile-optional">
                { optional }
            </div>
        </div>
    };

    html! {
        <div class={tile_item_class} {ondblclick} {onclick}>
            <div class="tile-checkbox-container">
                <input class="explorer-checkbox" type="checkbox" checked={*highlighted} onclick={onclickcheckbox} />
            </div>
            <div class="tile-filetypeicon-container">
                <TileFileTypeIcon {filetype}/>
            </div>
            {description}
        </div>
    }
}

#[function_component]
pub fn NotYetImplementedPane() -> Html {
    html! {
        <div id="unimplementedpane" class="fullpane">
            <h1>{"Not Yet Implemented"}</h1>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct GridFileTypeIconProps {
    item: Rc<PitouFile>,
}

#[function_component]
fn GridFileTypeIcon(props: &GridFileTypeIconProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let cnt = if ctx.show_thumbnails() {
        html! { <GridThumbnail item={props.item.clone()} /> }
    } else {
        if let Some(m) = props.item.metadata() {
            match m.kind() {
                PitouFileKind::Directory => {
                    if m.size.bytes == 0 {
                        html! {
                            <svg viewBox="0 0 491.52 491.52">
                                <path fill="#FFB42D" d="M445.522,88.989h-259.23c-5.832,0-11.24-3.318-14.26-8.749l-13.88-24.957
                                    c-3.021-5.432-8.427-8.749-14.259-8.749H45.998c-9.208,0-16.671,8.126-16.671,18.15v362.151c0,10.024,7.463,18.15,16.671,18.15
                                    h399.523c9.207,0,16.671-8.126,16.671-18.15V107.14C462.192,97.116,454.728,88.989,445.522,88.989z"/>
                                <path fill="#FFD264" d="M474.806,216.429H16.714c-10.557,0-17.956,8.348-16.541,18.538l27.158,195.639
                                    c1.107,7.974,9.46,14.379,18.667,14.379h399.523c9.207,0,17.56-6.405,18.667-14.379l27.158-195.639
                                    C492.761,224.777,485.362,216.429,474.806,216.429z"/>
                            </svg>
                        }
                    } else {
                        html! {
                            <svg viewBox="0 0 491.52 491.52">
                                <path fill="#FFB42D" d="M445.522,88.989h-259.23c-5.832,0-11.24-3.318-14.26-8.749l-13.88-24.957
                                    c-3.021-5.432-8.427-8.749-14.259-8.749H45.998c-9.208,0-16.671,8.126-16.671,18.15v362.151c0,10.024,7.463,18.15,16.671,18.15
                                    h399.523c9.207,0,16.671-8.126,16.671-18.15V107.14C462.192,97.116,454.728,88.989,445.522,88.989z"/>
                                <rect x="55.383" y="133.12" style="fill:#EBF0F3;" width="385.536" height="122.092"/>
                                <rect x="55.383" y="150.17" style="fill:#FFFFFF;" width="385.536" height="122.092"/>
                                <path fill="#FFD264" d="M474.806,216.429H16.714c-10.557,0-17.956,8.348-16.541,18.538l27.158,195.639
                                    c1.107,7.974,9.46,14.379,18.667,14.379h399.523c9.207,0,17.56-6.405,18.667-14.379l27.158-195.639
                                    C492.761,224.777,485.362,216.429,474.806,216.429z"/>
                            </svg>
                        }
                    }
                }
                PitouFileKind::File => html! { <img src="./public/file3.png"/> },
                PitouFileKind::Link => html! { <img src="./public/file3.png"/> },
            }
        } else {
            html! { <img src="./public/unknown_file.png"/> }
        }
    };
    html! {
        <div class="grid-filetypeicon-container">
            { cnt }
        </div>
    }
}

#[function_component]
fn GridThumbnail(_props: &GridFileTypeIconProps) -> Html {
    html! {}
}

#[function_component]
fn GridItem(props: &ItemProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let highlighted = use_state_eq(|| ctx.static_data.is_selected_dir_entry(props.item.clone()));

    {
        let ctx = ctx.clone();
        let item = props.item.clone();
        let highlighted = highlighted.clone();
        use_effect_with(ctx.refresher_state(), move |_| {
            highlighted.set(ctx.static_data.is_selected_dir_entry(item.clone()));
        })
    }

    let ondblclick = {
        let item = props.item.clone();
        let onopen = props.onopen.clone();
        move |_| onopen.emit(item.clone())
    };

    let onclick = {
        let highlighted = highlighted.clone();
        let item = props.item.clone();
        let ctx = ctx.clone();
        move |_| {
            if !*highlighted {
                ctx.static_data.select_folder_entry(item.clone());
                highlighted.set(true)
            } else {
                ctx.static_data.clear_dir_entry_selection(item.clone());
                highlighted.set(false)
            }
        }
    };

    let grid_item_class = format!(
        "grid-item {}",
        if *highlighted {
            "selected"
        } else {
            "not-selected"
        }
    );

    let onclickcheckbox = {
        let highlighted = highlighted.clone();
        let ctx = ctx.clone();
        let item = props.item.clone();
        move |e: MouseEvent| {
            e.stop_propagation();
            if !*highlighted {
                ctx.static_data.select_folder_entry(item.clone());
                highlighted.set(true)
            } else {
                ctx.static_data.clear_dir_entry_selection(item.clone());
                highlighted.set(false)
            }
        }
    };

    let name = if ctx.show_extensions() {
        props.item.name()
    } else {
        props.item.name_without_extension()
    };
    html! {
        <div class={grid_item_class} {ondblclick} {onclick}>
            <div class="grid-checkbox-container">
                <input class="grid-checkbox explorer-checkbox" type="checkbox" checked={*highlighted} onclick={onclickcheckbox} />
            </div>
            <GridFileTypeIcon item={props.item.clone()}/>
            <div class="grid-filename">{ name }</div>
        </div>
    }
}

#[function_component]
fn GridView(props: &ViewProps) -> Html {
    let content = props
        .items
        .iter()
        .map(|v| html! { <GridItem item = {v.clone()} onopen = {props.onopen.clone()} reload={props.reload.clone()} quietreload={props.quietreload.clone()}/> })
        .collect::<Html>();

    html! {
        <div id="pane-grid-view">
            { content }
        </div>
    }
}
