use std::rc::Rc;

use pitou_core::{frontend::ItemsView, PitouFile, PitouFilePath};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::{reusables::ChevronRightIcon, ApplicationContext};

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
    let active_tab = gen_ctx.active_tab.clone();
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
        let onclickchevron = {
            move |e: MouseEvent| e.stop_propagation() 
        };

        let onclickancestor = {
            move |e: MouseEvent| e.stop_propagation() 
        };
        let items = Some(html! {})
            .into_iter()
            .chain(
                active_tab
                    .into_iter()
                    .map(|v| {
                        pitou_path_ancestors(&*v.current_dir).map(|v| {
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
                    })
                    .flatten(),
            )
            .collect::<Html>();
        html! {
            <div id = "ancestry-items-container">
            { items }
            </div>
        }
    } else {
        let value = gen_ctx
            .active_tab
            .as_ref()
            .map(|v| {
                v.current_dir
                    .path
                    .to_str()
                    .map(|u| u.to_owned())
                    .unwrap_or_default()
            })
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

#[function_component]
pub fn ExplorerView() -> Html {
    html! {
        <>
            <Ancestry />
            <Explorer />
        </>
    }
}

#[function_component]
pub fn Explorer() -> Html {
    html! {
        <div id="explorer-pane" class="explorer-pane">
        </div>
    }
}

#[function_component]
fn ChildrenExplorer() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let children = use_state(|| {
        ctx.active_tab
            .as_ref()
            .map(|v| v.dir_children.borrow().clone())
            .flatten()
    });

    let children_node = match ctx.gen_ctx.app_settings.items_view {
        ItemsView::Grid => html! { <GridView items = { (*children).clone() } /> },
        ItemsView::Rows => html! { <RowView items = { (*children).clone() } /> },
        ItemsView::Tiles => html! { <TileView items = { (*children).clone() } /> },
    };

    html! {
        <div>
            { children_node }
        </div>
    }
}

#[derive(Properties)]
struct ItemsViewProps {
    items: Option<Rc<Vec<Rc<PitouFile>>>>,
}

impl PartialEq for ItemsViewProps {
    fn eq(&self, other: &Self) -> bool {
        match (self.items.as_ref(), other.items.as_ref()) {
            (None, None) => true,
            (Some(vals1), Some(vals2)) => vals1.as_ptr() == vals2.as_ptr(),
            _ => false,
        }
    }
}

fn pitou_path_ancestors(pitou: &PitouFilePath) -> impl Iterator<Item = PitouFilePath> {
    let mut ll = std::collections::LinkedList::new();
    for anc in pitou.path.ancestors() {
        if anc.as_os_str().len() == 0 {
            break;
        }
        ll.push_front(PitouFilePath::from_pathbuf(std::path::PathBuf::from(anc)))
    }
    ll.into_iter()
}

#[function_component]
fn GridView(props: &ItemsViewProps) -> Html {
    html! {}
}

#[function_component]
fn TileView(props: &ItemsViewProps) -> Html {
    html! {}
}

#[function_component]
fn RowView(props: &ItemsViewProps) -> Html {
    html! {}
}
