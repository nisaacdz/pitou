use std::rc::Rc;

use pitou_core::{
    frontend::{extra::PitouFileWrapper, *},
    PitouFile,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::{reusables::FileTypeIcon, ApplicationContext};

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
    let content = props
        .items
        .iter()
        .map(|v| html! { <ListItem item = {v.clone()} onopen = {props.onopen.clone()} /> })
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
            <div class="list-checkbox-container">
                <input class="list-checkbox" type="checkbox" />
            </div>
            <div class="list-filetypeicon-container">
                { "ico" }
            </div>
            <div class="list-filename-container">
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
    let highlighted = use_state_eq(|| {
        ctx.active_tab
            .selected_files
            .borrow()
            .contains(&PitouFileWrapper {
                file: props.item.clone(),
            })
    });

    let ontoggle = {
        let highlighted = highlighted.clone();
        let ctx = ctx.clone();
        let item = props.item.clone();
        move |e: Event| {
            e.stop_propagation();
            let elem = e.target_dyn_into::<HtmlInputElement>().unwrap();
            if elem.checked() {
                ctx.active_tab.append_selected(item.clone());
                highlighted.set(true)
            } else {
                ctx.active_tab.remove_selected(item.clone());
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

    let checkbox_class = format!(
        "list-checkbox {}",
        if *highlighted {
            "checked"
        } else {
            "not-checked"
        }
    );

    let filetype = props.item.metadata.as_ref().map(|v| v.kind);

    html! {
        <div class={list_item_class} {ondblclick}>
            <div class="list-checkbox-container">
                <input class={checkbox_class} type="checkbox" checked={*highlighted} {ontoggle} />
            </div>
            <div class="list-filetypeicon-container">
                <FileTypeIcon {filetype}/>
            </div>
            <div class="list-filename-container">
                <div class="list-filename">{ props.item.name() }</div>
            </div>
            <div class="list-modifieddate-container">
                <div>{ "Today" }</div>
            </div>
            <div class="list-accesseddate-container">
                <div>{ "Yesterday" }</div>
            </div>
            <div class="list-createddate-container">
                <div>{ "Last week" }</div>
            </div>
        </div>
    }
}

#[function_component]
fn GridView(_props: &ViewProps) -> Html {
    html! {}
}

#[function_component]
fn TileView(_props: &ViewProps) -> Html {
    todo!()
}
