use std::rc::Rc;

use pitou_core::{frontend::*, *};
use yew::prelude::*;

use crate::app::reusables::{ListFileTypeIcon, TileFileTypeIcon};

#[derive(Properties)]
pub struct PaneViewProps {
    pub onopen: Callback<Rc<PitouFile>>,
    pub view: ItemsView,
    pub items: Rc<Vec<Rc<PitouFile>>>,
}

impl PartialEq for PaneViewProps {
    fn eq(&self, other: &Self) -> bool {
        self.view == other.view && Rc::ptr_eq(&self.items, &other.items)
    }
}

#[function_component]
pub fn MainPane(props: &PaneViewProps) -> Html {
    let items = props.items.clone();
    let onopen = props.onopen.clone();
    match props.view {
        ItemsView::Grid => html! { <GridView {items} {onopen} /> },
        ItemsView::Rows => html! { <ListView {items} {onopen} /> },
        ItemsView::Tiles => html! { <TileView {items} {onopen} /> },
    }
}

#[derive(Properties)]
struct ViewProps {
    onopen: Callback<Rc<PitouFile>>,
    items: Rc<Vec<Rc<PitouFile>>>,
}

#[derive(Properties)]
struct ItemProps {
    onopen: Callback<Rc<PitouFile>>,
    item: Rc<PitouFile>,
}

impl PartialEq for ItemProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.item, &other.item)
    }
}

impl PartialEq for ViewProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.items, &other.items)
    }
}

#[function_component]
fn ListView(props: &ViewProps) -> Html {
    let free_space = html! {
        <div id="pane-list-view-bottom-space">
            <div id="pane-list-view-bottom-space-i"></div>
        </div>
    };

    let content = props
        .items
        .iter()
        .map(|v| html! { <ListItem item = {v.clone()} onopen = {props.onopen.clone()} /> })
        .chain(Some(free_space))
        .collect::<Html>();

    html! {
        <>
        <ListDsc />
            <div id="pane-list-view">
                { content }
            </div>
        </>
    }
}

#[function_component]
fn ListDsc() -> Html {
    html! {
        <div id="pane-list-view-dsc">
            <div class="pane-list-view-dsc-checkbox-container">
                <input class="pane-list-view-dsc-checkbox" type="checkbox" />
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

    let ontoggle = {
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
                <input class="explorer-checkbox" type="checkbox" checked={*highlighted} {ontoggle} />
            </div>
            <div class="list-filetypeicon-container">
                <ListFileTypeIcon {filetype}/>
            </div>
            <div class="list-filename-container">
                <div class="list-filename">{ props.item.name() }</div>
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
fn GridView(_props: &ViewProps) -> Html {
    html! {}
}

#[function_component]
fn TileView(props: &ViewProps) -> Html {
    let content = props
        .items
        .iter()
        .map(|v| html! { <TileItem item = {v.clone()} onopen = {props.onopen.clone()} /> })
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

    let ontoggle = {
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

    let filesize = props.item.metadata.as_ref().map(|v| {
        if v.is_dir() {
            v.size.format_as_dir_entries()
        } else {
            v.size.format()
        }
    });

    let filetype = props.item.metadata.as_ref().map(|v| v.kind);

    let optional = props
        .item
        .metadata
        .as_ref()
        .map(|v| v.accessed.datetime.format("%Y-%m-%d %H:%M:%S").to_string());

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
                <input class="explorer-checkbox" type="checkbox" checked={*highlighted} {ontoggle} />
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
