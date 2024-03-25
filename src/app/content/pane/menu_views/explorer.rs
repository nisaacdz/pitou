use std::rc::Rc;

use pitou_core::{frontend::*, *};
use tauri_sys::tauri::invoke;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::app::{
    reusables::{ChevronRightIcon, DirChildren, DirChildrenArgs, MainPane},
    ApplicationContext,
};

#[derive(PartialEq, Properties)]
struct AncestryProps {
    onopen: Callback<Rc<PitouFile>>,
}

#[function_component]
fn Ancestry(props: &AncestryProps) -> Html {
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

    let onopen = {
        let show_ancestry = show_ancestry.clone();
        let onopen = props.onopen.clone();
        move |path_str: String| {
            let file = PitouFile {
                path: PitouFilePath::from_pathbuf(std::path::PathBuf::from(path_str)),
                metadata: None,
            };
            show_ancestry.set(true);
            onopen.emit(Rc::new(file))
        }
    };

    let onkeypress = {
        let onopen = onopen.clone();
        move |e: KeyboardEvent| {
            if e.char_code() == 13 {
                let val = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
                onopen(val)
            }
        }
    };

    let onblur = {
        move |e: FocusEvent| {
            let e = e.target_dyn_into::<HtmlInputElement>().unwrap();
            let val = e.value();
            onopen(val)
        }
    };

    let onclick = {
        let show_ancestry = show_ancestry.clone();
        move |_| show_ancestry.set(false)
    };

    let content = if *show_ancestry {
        let onclickchevron = { move |e: MouseEvent| e.stop_propagation() };

        let onclickancestor = { move |e: MouseEvent| e.stop_propagation() };
        let db = gen_ctx.active_tab.current_dir();
        let items = db
            .as_ref()
            .into_iter()
            .map(|v| v.path.ancestors())
            .flatten()
            .map(|path| {
                Rc::new(PitouFile {
                    path,
                    metadata: None,
                })
            })
            .map(|v| {
                let onclick = {
                    let onopen = props.onopen.clone();
                    let v = v.clone();
                    move |_| onopen.emit(v.clone())
                };
                html! {
                    <>
                        <div class="ancestry-ancestor" onclick={onclickancestor.clone()} {onclick}>
                        { v.name() }
                        </div>
                        <div class="ancestry-chevron-container" onclick={onclickchevron}>
                            <ChevronRightIcon id="" class="ancestry-chevron"/>
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
            .current_dir()
            .as_ref()
            .map(|v| v.path.path.display().to_string())
            .unwrap_or_default();
        html! {
            <input ref={input_elem_ref} id="ancestry-path" type="text" {onblur} {value} {onkeypress}/>
        }
    };

    html! {
        <div id="ancestry" {class} {onclick}>
        { content }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ExplorerViewProps {
    pub onopen: Callback<Rc<PitouFile>>,
}

#[function_component]
pub fn ExplorerView(props: &ExplorerViewProps) -> Html {
    html! {
        <div id="explorer-view-pane" class="fullpane">
            <Ancestry onopen={props.onopen.clone()}/>
            <Explorer onopen={props.onopen.clone()}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ExplorerProps {
    onopen: Callback<Rc<PitouFile>>,
}

pub async fn update_children(tab: Rc<TabCtx>, after: impl Fn()) {
    let file = tab.current_dir();
    if let Some(file) = file {
        let new_children = invoke::<DirChildrenArgs, DirChildren>(
            "children",
            &DirChildrenArgs::new_default(&file.path),
        )
        .await
        .ok()
        .map(|v| Rc::new(v.children));

        let current_dir = tab.current_dir();
        if matches!(current_dir, Some(v) if v.path == file.path) {
            {
                let to_print = new_children.as_ref().map(|v| v.len());
                web_sys::console::log_1(&serde_wasm_bindgen::to_value(&format!("obtained {:?} files", to_print)).unwrap());
            }
            *tab.dir_children.borrow_mut() = new_children;
            after()
        } else {
            web_sys::console::log_1(&serde_wasm_bindgen::to_value("mishit").unwrap())
        }
    }
}

#[function_component]
fn Explorer(props: &ExplorerProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let refresher = use_force_update();

    {
        let tab = ctx.active_tab.clone();
        let refresher = refresher.clone();
        let current_dir = tab.current_dir();
        use_effect_with(current_dir, move |_| {
            spawn_local(async move {
                update_children(tab, move || refresher.force_update()).await;
            })
        })
    }

    {
        let ctx = ctx.clone();
        let refresher = refresher.clone();
        use_interval(
            move || {
                let ctx = ctx.clone();
                let refresher = refresher.clone();
                let tab = ctx.active_tab.clone();
                spawn_local(async move {
                    update_children(tab, move || refresher.force_update()).await;
                })
            },
            5000,
        )
    }

    let onopen = props.onopen.clone();

    let content = if let Some(items) = &*ctx.active_tab.dir_children.borrow() {
        let items = items.clone();
        let view = ctx.gen_ctx.app_settings.items_view;
        html! { <MainPane {view} {items} {onopen} />}
    } else {
        html! {}
    };

    html! {
        <div id="explorer-main-section">
        { content }
        </div>
    }
}
