use std::{cell::RefCell, rc::Rc};

use super::Ancestry;
use crate::app::reusables::ListFileTypeIcon;
use pitou_core::{frontend::*, search::SimplifiedSearchOptions, *};
use tokio_stream::StreamExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{platform::time::interval, prelude::*};
use yew_hooks::{use_effect_update, use_effect_update_with_deps, use_interval};

async fn begin_stream_search(
    options: SimplifiedSearchOptions,
    bank: Rc<RefCell<Vec<Rc<PitouFile>>>>,
    searching: Rc<RefCell<bool>>,
    update: UseForceUpdateHandle,
) {
    if let Ok(()) = crate::app::cmds::search(options).await {
        *searching.borrow_mut() = true;
        let mut interval = Box::pin(interval(std::time::Duration::from_millis(250)));
        while let Some(_) = interval.next().await {
            if let Ok(msg) = crate::app::cmds::search_msg().await {
                match msg {
                    msg::SearchMsg::Active(ll) => {
                        bank.borrow_mut().extend(ll.into_iter().map(|v| Rc::new(v)));
                        update.force_update();
                    }
                    msg::SearchMsg::Terminated(ll) => {
                        bank.borrow_mut().extend(ll.into_iter().map(|v| Rc::new(v)));
                        update.force_update();
                        *searching.borrow_mut() = false;
                        break;
                    }
                }
            } else {
                *searching.borrow_mut() = false;
                break;
            }
        }
    } else {
        web_sys::console::log_1(&serde_wasm_bindgen::to_value("couldn't begin search").unwrap());
    }
}

#[derive(PartialEq, Properties)]
pub struct SearchPaneProps {
    pub onopen: Callback<Rc<PitouFile>>,
    pub reload: Callback<()>,
    pub quietreload: Callback<()>,
}

#[function_component]
pub fn SearchPane(props: &SearchPaneProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let update = use_force_update();
    let searching = use_mut_ref(|| false);

    let onsearch = {
        let ctx = ctx.clone();
        let update = update.clone();
        let searching = searching.clone();
        move |options: FrontendSearchOptions| {
            if !*searching.borrow() {
                let ctx = ctx.clone();
                ctx.static_data.clear_all_selections();
                ctx.active_tab.reset_search_results();
                let bank = ctx.active_tab.get_or_init_search_results();
                let update = update.clone();
                let searching = searching.clone();
                if let Some(search_dir) = ctx.active_tab.current_dir() {
                    spawn_local(async move {
                        let options = SimplifiedSearchOptions::build_from(options, search_dir);
                        begin_stream_search(options, bank, searching, update).await;
                    })
                }
            }
        }
    };

    let oncancel = {
        move |()| {
            spawn_local(async move {
                crate::app::cmds::terminate_search().await.unwrap();
            })
        }
    };

    let results = ctx.active_tab.search_results();

    html! {
        <div id="search-pane" class="fullpane">
            <Ancestry onopen={props.onopen.clone()}/>
            <SearchOptionsPane {onsearch} {oncancel} {searching} />
            <SearchResultsPane {results} onopen={props.onopen.clone()} reload={props.reload.clone()} quietreload={props.quietreload.clone()}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct SearchOptionsPaneProps {
    onsearch: Callback<FrontendSearchOptions>,
    oncancel: Callback<()>,
    searching: Rc<RefCell<bool>>,
}

#[function_component]
fn SearchOptionsPane(props: &SearchOptionsPaneProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let force_update = use_force_update();
    let input_ref = use_node_ref();
    let searching = use_state_eq(|| *props.searching.borrow());

    {
        let input_ref = input_ref.clone();
        use_effect_with((), move |()| {
            input_ref.cast::<HtmlInputElement>().unwrap().focus().ok();
        })
    }

    {
        let prop_searching = props.searching.clone();
        let searching = searching.clone();
        use_interval(move || searching.set(*prop_searching.borrow()), 250)
    }

    let finish = {
        let onsearch = props.onsearch.clone();
        let ctx = ctx.clone();
        move || {
            if ctx.active_tab.search_options.borrow().input.len() > 0 {
                let options = ctx.active_tab.search_options();
                onsearch.emit(options)
            }
        }
    };

    let onclicksearch = {
        let onfinish = finish.clone();
        move |_| onfinish()
    };

    let onclickcancel = {
        let oncancel = props.oncancel.clone();
        move |_| oncancel.emit(())
    };

    let onchangesearchtype = {
        let ctx = ctx.clone();
        let force_update = force_update.clone();
        move |e: Event| {
            let idx = e
                .target_dyn_into::<HtmlSelectElement>()
                .unwrap()
                .selected_index();
            if idx == 0 || idx == 1 {
                ctx.active_tab
                    .update_search_options(|so| so.search_kind = idx as u8)
            }
            force_update.force_update();
        }
    };

    let onclickfilterfile = {
        let ctx = ctx.clone();
        move |m: Event| {
            ctx.active_tab.update_search_options(|so| {
                so.filter.files = m.target_dyn_into::<HtmlInputElement>().unwrap().checked();
            });
        }
    };

    let onclickfilterfolder = {
        let ctx = ctx.clone();
        move |m: Event| {
            ctx.active_tab.update_search_options(|so| {
                so.filter.dirs = m.target_dyn_into::<HtmlInputElement>().unwrap().checked();
            });
        }
    };

    let onclickfilterlink = {
        let ctx = ctx.clone();
        move |m: Event| {
            ctx.active_tab.update_search_options(|so| {
                so.filter.links = m.target_dyn_into::<HtmlInputElement>().unwrap().checked();
            });
        }
    };

    let onclickfiltersys = {
        let ctx = ctx.clone();
        move |m: Event| {
            ctx.active_tab.update_search_options(|so| {
                so.filter.sys_items = m.target_dyn_into::<HtmlInputElement>().unwrap().checked();
            });
        }
    };

    let onkeypress = {
        let finish = finish.clone();
        move |k: KeyboardEvent| {
            if k.key_code() == 13 {
                finish()
            }
        }
    };

    let oninput = {
        let ctx = ctx.clone();
        move |e: InputEvent| {
            let value = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
            ctx.active_tab.update_search_options(|so| so.input = value);
        }
    };

    let onchangemaxfinds = {
        let ctx = ctx.clone();
        move |e: Event| {
            let value = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
            if let Ok(val) = value.parse() {
                ctx.active_tab
                    .update_search_options(|so| so.max_finds = val);
            }
        }
    };

    let onchangedepth = {
        let ctx = ctx.clone();
        move |e: Event| {
            let value = e.target_dyn_into::<HtmlInputElement>().unwrap().value();
            if let Ok(val) = value.parse() {
                ctx.active_tab.update_search_options(|so| so.depth = val);
            }
        }
    };

    let onclickcasesensitive = {
        let ctx = ctx.clone();
        move |e: Event| {
            let cs = e.target_dyn_into::<HtmlInputElement>().unwrap().checked();
            ctx.active_tab
                .update_search_options(|so| so.case_sensitive = cs);
        }
    };

    let placeholder = format! {"Enter search key"};

    let so = ctx.active_tab.search_options.borrow();
    let current_depth = so.depth.to_string();
    let max_finds = so.max_finds.to_string();
    let search_kind = so.search_kind;
    let files_filtered = so.filter.files;
    let dirs_filtered = so.filter.dirs;
    let links_filtered = so.filter.links;
    let sys_filtered = so.filter.sys_items;
    let value = so.input.clone();
    std::mem::drop(so);

    let search_sub_kind = {
        let onchange = {
            let ctx = ctx.clone();
            move |e: Event| {
                let idx = e
                    .target_dyn_into::<HtmlSelectElement>()
                    .unwrap()
                    .selected_index();
                if idx >= 0 {
                    ctx.active_tab
                        .update_search_options(|so| so.search_kind = (1 + idx) as u8);
                }
            }
        };

        match search_kind {
            0 => html! {},
            v => {
                html! {
                    <div class="search-type">
                        <label>
                            {"Portion to match: "}
                            <select class="select-box" {onchange}>
                                <option selected={v==1}>{"StartsWith"}</option>
                                <option selected={v==2}>{"EndsWith"}</option>
                                <option selected={v==3}>{"Contains"}</option>
                            </select>
                        </label>
                    </div>
                }
            }
        }
    };

    let search_or_cancel_btn = {
        if *searching {
            html! {
                <button onclick={onclickcancel} id="search-options-cancel-btn">{"Cancel"}</button>
            }
        } else {
            html! {
                <button onclick={onclicksearch} id="search-options-search-btn">{"Search"}</button>
            }
        }
    };

    let autocomplete = "off";

    html! {
        <div id="search-options-pane" class="side-pane">
            <input id="search-options-input" type="text" {onkeypress} {placeholder} {oninput} ref={input_ref} {value} {autocomplete}/>
            <span class="title">{"Search Options"}</span>
            <label>
                {"Type:"}
                <select class="select-box" onchange={onchangesearchtype}>
                    <option selected={search_kind==0}>{"Regex"}</option>
                    <option selected={search_kind>0}>{"Standard"}</option>
                </select>
            </label>
            {search_sub_kind}
            <label>
                {"Depth:"}
                <input type="number" min={1} max={32} value={current_depth} onchange={onchangedepth}/>
            </label>
            <label>
                {"Case Sensitive:"}
                <input type="checkbox" ontoggle={onclickcasesensitive}/>
            </label>
            <label>
                {"Files"}
                <input type="checkbox" ontoggle={onclickfilterfile} checked={files_filtered}/>
            </label>
            <label>
                {"Folders"}
                <input type="checkbox" ontoggle={onclickfilterfolder} checked={dirs_filtered}/>
            </label>
            <label>
                {"Symlinks"}
                <input type="checkbox" ontoggle={onclickfilterlink} checked={links_filtered}/>
            </label>
            <label>
                {"System Files"}
                <input type="checkbox" ontoggle={onclickfiltersys} checked={sys_filtered}/>
            </label>
            <label>
                {"Max Finds:"}
                <input type="number" min={1} max={5000} value={max_finds} onchange={onchangemaxfinds}/>
            </label>
            { search_or_cancel_btn }
        </div>
    }
}

#[derive(Properties)]
struct SearchResultsPaneProps {
    results: Option<Rc<RefCell<Vec<Rc<PitouFile>>>>>,
    onopen: Callback<Rc<PitouFile>>,
    quietreload: Callback<()>,
    reload: Callback<()>,
}

impl PartialEq for SearchResultsPaneProps {
    fn eq(&self, other: &Self) -> bool {
        match (&self.results, &other.results) {
            (None, None) => true,
            _ => false,
        }
    }
}

#[function_component]
fn SearchResultsPane(props: &SearchResultsPaneProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let ontoggleselectall = {
        let ctx = ctx.clone();
        let items = props.results.clone();
        let quietreload = props.quietreload.clone();
        move |()| {
            if let Some(items) = items.clone() {
                if ctx
                    .static_data
                    .are_all_selected_search_results(items.clone())
                {
                    items
                        .borrow()
                        .iter()
                        .for_each(|item| ctx.static_data.clear_dir_entry_selection(item.clone()))
                } else {
                    items
                        .borrow()
                        .iter()
                        .for_each(|item| ctx.static_data.select_folder_entry(item.clone()))
                }
                quietreload.emit(())
            }
        }
    };

    let contents = match &props.results {
        Some(items) => items
            .borrow()
            .iter()
            .map(|item| {
                let item = item.clone();
                let onopen = props.onopen.clone();
                let quietreload = { move |()| () };
                let reload = { move |_| () };
                html! { <ListItem {item} {onopen} {reload} {quietreload}/>}
            })
            .collect::<Html>(),
        None => html! {},
    };

    html! {
        <>
            <div id="search-results-pane">
                <ListDsc ontoggle={ontoggleselectall}/>
                <div id="pane-list-view">
                    { contents }
                </div>
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

#[derive(Properties, PartialEq)]
struct ListItemProps {
    onopen: Callback<Rc<PitouFile>>,
    item: Rc<PitouFile>,
    reload: Callback<()>,
    quietreload: Callback<()>,
}

#[function_component]
fn ListItem(props: &ListItemProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let highlighted = use_state_eq(|| {
        ctx.static_data
            .is_selected_search_result(props.item.clone())
    });

    {
        let ctx = ctx.clone();
        let item = props.item.clone();
        let highlighted = highlighted.clone();
        use_effect_with(ctx.refresher_state(), move |_| {
            highlighted.set(ctx.static_data.is_selected_search_result(item.clone()));
        })
    }

    let toggle_selection = {
        let item = props.item.clone();
        let highlighted = highlighted.clone();
        let ctx = ctx.clone();
        move || {
            if !*highlighted {
                ctx.static_data.select_search_result(item.clone());
                highlighted.set(true)
            } else {
                ctx.static_data.clear_search_result(item.clone());
                highlighted.set(false)
            }
        }
    };

    let onclick = {
        let toggle_selection = toggle_selection.clone();
        move |_| toggle_selection()
    };

    let onchange = {
        let toggle_selection = toggle_selection.clone();
        move |e: Event| {
            e.stop_propagation();
            toggle_selection();
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

    let name = props.item.full_path_str();
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
                <div class="list-filename search-filename">{ name }</div>
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

#[derive(Properties, PartialEq)]
struct SearchResultProps {
    item: Rc<PitouFile>,
    onopen: Callback<()>,
}
