use std::{cell::RefCell, collections::HashSet, rc::Rc};

use pitou_core::{frontend::ApplicationContext, PitouFileKind, PitouTrashItem};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::app::reusables::{ListFileTypeIcon, NoArg, PitouTrashItemsVec, TrashItem};

async fn retrieve_trash_items() -> Option<Rc<Vec<Rc<PitouTrashItem>>>> {
    tauri_sys::tauri::invoke("thrash_items", &NoArg)
        .await
        .map(|v: PitouTrashItemsVec| v.items)
        .ok()
}

#[function_component]
pub fn TrashView() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let force_update = use_force_update();
    let selections = use_mut_ref(|| HashSet::new());
    {
        let ctx = ctx.clone();
        let force_update = force_update.clone();
        use_effect_with(ctx.refresher_state(), move |_| {
            if ctx.static_data.no_trash_items() {
                let ctx = ctx.clone();
                let force_update = force_update.clone();
                spawn_local(async move {
                    let trash_items = retrieve_trash_items().await;
                    ctx.static_data.update_trash_items(trash_items);
                    force_update.force_update();
                })
            }
        });
    }
    {
        let ctx = ctx.clone();
        let force_update = force_update.clone();
        use_interval(
            move || {
                let ctx = ctx.clone();
                let force_update = force_update.clone();
                spawn_local(async move {
                    let trash_items = retrieve_trash_items().await;
                    ctx.static_data.update_trash_items(trash_items);
                    force_update.force_update()
                })
            },
            10000,
        );
    }

    let content = if let Some(val) = ctx.static_data.trash_items() {
        html! { <TrashListView items= { val.clone() } {selections}/> }
    } else {
        html! {}
    };

    html! {
        <div id="trash-pane" class="fullpane">
            { content }
        </div>
    }
}

#[derive(Properties)]
struct TrashPaneProps {
    items: Rc<Vec<Rc<PitouTrashItem>>>,
    selections: Rc<RefCell<HashSet<TrashItem>>>,
}

impl PartialEq for TrashPaneProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.items, &other.items)
    }
}

#[function_component]
fn TrashListView(props: &TrashPaneProps) -> Html {
    let free_space = html! {
        <div id="pane-list-view-bottom-space">
            <div id="pane-list-view-bottom-space-i"></div>
        </div>
    };

    let content = props
        .items
        .iter()
        .map(|v| html! { <TrashListItem item = {v.clone()} selections = {props.selections.clone()} /> })
        .chain(Some(free_space))
        .collect::<Html>();

    html! {
        <>
            <TrashListDsc />
            <div id="pane-list-view">
                { content }
            </div>
        </>
    }
}

#[function_component]
fn TrashListDsc() -> Html {
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
                { "Deleted on" }
            </div>
        </div>
    }
}

#[derive(Properties)]
struct TrashItemProps {
    item: Rc<PitouTrashItem>,
    selections: Rc<RefCell<HashSet<TrashItem>>>,
}

impl PartialEq for TrashItemProps {
    fn eq(&self, other: &Self) -> bool {
        &self.item.original_path == &other.item.original_path
    }
}

#[function_component]
fn TrashListItem(props: &TrashItemProps) -> Html {
    let selected = use_state_eq(|| {
        props
            .selections
            .borrow()
            .contains(&TrashItem::new(props.item.clone()))
    });

    let toggle = {
        let item = TrashItem::new(props.item.clone());
        let selections = props.selections.clone();
        let selected = selected.clone();
        move || {
            let mut selections = selections.borrow_mut();
            if selections.contains(&item) {
                selections.remove(&item);
                selected.set(false);
            } else {
                selections.insert(item.clone());
                selected.set(true);
            }
            std::mem::drop(selections);
        }
    };

    let onclick = {
        let toggle = toggle.clone();
        move |_| toggle()
    };

    let onchange = { move |_| toggle() };

    let ondblclick = {
        let _item = props.item.clone();
        move |_| ()
    };

    let list_item_class = format!(
        "list-item {}",
        if *selected {
            "selected"
        } else {
            "not-selected"
        }
    );

    let filetype = if props.item.is_dir() {
        PitouFileKind::Directory
    } else {
        PitouFileKind::File
    };

    let deleted = props
        .item
        .metadata()
        .deleted
        .datetime
        .format("%Y-%m-%d %H:%M")
        .to_string();
    let name = props.item.name().to_owned();

    html! {
        <div class={list_item_class} {ondblclick} {onclick}>
            <div class="list-checkbox-container">
                <input class="explorer-checkbox" type="checkbox" checked={*selected} {onchange} />
            </div>
            <div class="list-filetypeicon-container">
                <ListFileTypeIcon {filetype}/>
            </div>
            <div class="list-filename-container">
                <div class="list-filename">{ name }</div>
            </div>
            <div class="list-modifieddate-container">
                <div>{ deleted }</div>
            </div>
        </div>
    }
}
