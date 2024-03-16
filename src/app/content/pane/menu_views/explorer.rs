use std::rc::Rc;

use tauri_sys::tauri::invoke;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::app::{
    reusables::{ChevronRightIcon, DirChildren, DirChildrenArgs, MainPane},
    ApplicationContext,
};

#[function_component]
pub fn Ancestry() -> Html {
    let gen_ctx = use_context::<ApplicationContext>().unwrap();
    let show_ancestry = use_state_eq(|| true);
    let input_elem_ref = use_node_ref();
    {
        let input_elem_ref = input_elem_ref.clone();
        use_effect(move || {
            if let Some(elem) = input_elem_ref.cast::<HtmlInputElement>() {
                elem.focus().unwrap()
            }
        });
    }
    let class = if *show_ancestry {
        "show-ancestry"
    } else {
        "show-absolute-path"
    };

    let onblur = {
        let show_ancestry = show_ancestry.clone();
        move |_| show_ancestry.set(true)
    };

    let onclick = {
        let show_ancestry = show_ancestry.clone();
        move |_| show_ancestry.set(false)
    };

    let content = if *show_ancestry {
        let onclickchevron = { move |e: MouseEvent| e.stop_propagation() };

        let onclickancestor = { move |e: MouseEvent| e.stop_propagation() };
        let db = gen_ctx.active_tab.current_dir.borrow();
        let items = db
            .as_ref()
            .into_iter()
            .map(|v| v.path.ancestors())
            .flatten()
            .map(|v| {
                html! {
                    <>
                        <div class="ancestry-chevron-container" onclick={onclickchevron}>
                            <ChevronRightIcon id="" class="ancestry-chevron"/>
                        </div>
                        <div class="ancestry-ancestor" onclick={onclickancestor.clone()}>
                        { v.name() }
                        </div>
                    </>
                }
            })
            .collect::<Html>();
        html! {
            <div id = "ancestry-items-container">
            { items }
            </div>
        }
    } else {
        let value = gen_ctx
            .active_tab
            .current_dir
            .borrow()
            .as_ref()
            .map(|v| v.path.path.display().to_string())
            .unwrap_or_default();
        html! {
            <input ref={input_elem_ref} id="ancestry-path" type="text" {onblur} {value}/>
        }
    };

    html! {
        <div id="ancestry" {class} {onclick}>
        { content }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ExplorerViewProps {}

#[function_component]
pub fn ExplorerView(props: &ExplorerViewProps) -> Html {
    html! {
        <>
            <Ancestry />
            <Explorer />
        </>
    }
}

#[function_component]
pub fn Explorer() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let children = use_state(|| ctx.active_tab.dir_children.borrow().clone());

    {
        let ctx = ctx.clone();
        let children = children.clone();
        use_interval(
            move || {
                let ctx = ctx.clone();
                let children = children.clone();
                spawn_local(async move {
                    if let Some(file) = &*ctx.active_tab.current_dir.borrow() {
                        let new_children = invoke::<DirChildrenArgs, DirChildren>(
                            "children",
                            &DirChildrenArgs::new_default(&file.path),
                        )
                        .await
                        .ok()
                        .map(|v| Rc::new(v.children));

                        *ctx.active_tab.dir_children.borrow_mut() = new_children;
                        let items = ctx.active_tab.dir_children.borrow().clone();
                        children.set(items)
                    }
                })
            },
            250,
        )
    }

    let onopen = {
        let _ctx = ctx.clone();
        move |_v| {
            // TODO
        }
    };

    let content = if let Some(items) = &*children {
        let items = items.clone();
        let view = ctx.gen_ctx.app_settings.items_view;
        html! { <MainPane {view} {items} {onopen} />}
    } else {
        html! {}
    };

    html! {
        <div id="explorer-pane" class="explorer-pane">
        { content }
        </div>
    }
}
