use std::{cell::RefCell, collections::HashSet, rc::Rc, time::Duration};

use pitou_core::{
    frontend::ApplicationContext, msg::{TransferMsg, TransferSessionID, TransferSize, TransferState}, PitouDateTime, PitouFileSize
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component]
pub fn Status() -> Html {
    html! {
        <div id="status-bar">
            <SearchWatcher />
            <TransfersWatcher />
            <SelectionsWatcher />
        </div>
    }
}

#[function_component]
pub fn SelectionsWatcher() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let number = use_state_eq(|| 0);
    {
        let number = number.clone();
        use_interval(move || {
            // Calculate selections size based on the selection type
            // Using available public methods from pitou_core
            let new_number = if ctx.static_data.has_folder_entry_selections() {
                ctx.static_data.folder_entry_selections().map(|v| v.len()).unwrap_or(0)
            } else if ctx.static_data.search_result_selections().is_some() {
                ctx.static_data.search_result_selections().map(|v| v.len()).unwrap_or(0)
            } else {
                // For other selection types, check if any selection exists
                0
            };
            number.set(new_number);
        }, 250)
    }
    if *number == { 0 } { return html! {} }
    let prompt = format!{"{} selection{}", *number, if *number == 1 { "" } else { "s" }};
    html! {
        <div id="file-selections-watcher">
            <input type="checkbox" checked={true}/>
            <span>{ prompt }</span>
        </div>
    }
}

#[function_component]
pub fn SearchWatcher() -> Html {
    let searching = use_state_eq(|| false);
    {
        let searching = searching.clone();
        use_interval(
            move || {
                let searching = searching.clone();
                spawn_local(async move {
                    let msg = crate::app::cmds::is_searching().await;
                    searching.set(msg);
                })
            },
            500,
        )
    }
    if !*searching {
        return html! {};
    }
    let content = html! {
        <img src="./public/search_anim.gif"/>
    };
    html! {
        <div id="file-search-watcher">
            { content }
        </div>
    }
}

#[function_component]
pub fn TransfersWatcher() -> Html {
    let sessions = use_state(|| Rc::new(RefCell::new(HashSet::new())));

    {
        let sessions = sessions.clone();
        use_effect_with((), move |()| {
            let sessions = sessions.clone();

            spawn_local(async move {
                crate::app::events::listen_looping_event("pasting", |v: TransferSessionID| {
                    let new_sessions = (*sessions).clone();
                    new_sessions.borrow_mut().insert(v);
                    sessions.set(new_sessions);
                })
                .await;
            })
        });
    }

    if sessions.borrow().is_empty() {
        return html! {};
    }

    let onfinish = {
        let sessions = sessions.clone();
        move |ns: TransferMsg| {
            let new_sessions = (*sessions).clone();
            new_sessions.borrow_mut().remove(&ns.id());
            sessions.set(new_sessions);
        }
    };

    let onsessiondead = {
        let sessions = sessions.clone();
        move |id| {
            let new_sessions = (*sessions).clone();
            new_sessions.borrow_mut().remove(&id);
            sessions.set(new_sessions);
        }
    };

    let content = sessions
        .borrow()
        .iter()
        .map(|&sessionid| {
            let onfinish = onfinish.clone();
            let onsessiondead = onsessiondead.clone();
            html! {
                <TransferSession {sessionid} {onfinish} {onsessiondead} />
            }
        })
        .collect::<Html>();

    html! {
        <div id="file-transfer-watcher">
            {content}
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct TransferSessionProps {
    sessionid: TransferSessionID,
    onfinish: Callback<TransferMsg>,
    onsessiondead: Callback<TransferSessionID>,
}

#[function_component]
fn TransferSession(props: &TransferSessionProps) -> Html {
    let msg: UseStateHandle<Option<TransferMsg>> = use_state(|| None);

    {
        let onfinish = props.onfinish.clone();
        let msg = msg.clone();
        use_effect(move || {
            if let Some(msg) = &*msg {
                if msg.is_terminated() {
                    onfinish.emit(*msg)
                }
            }
        });
    }

    {
        let msg = msg.clone();
        let sessionid = props.sessionid;
        let onsessiondead = props.onsessiondead.clone();
        use_interval(
            move || {
                let msg = msg.clone();
                let onsessiondead = onsessiondead.clone();
                spawn_local(async move {
                    let res = crate::app::cmds::transfer_session_with_id(sessionid).await;
                    if let Some(res) = res {
                        msg.set(Some(res))
                    } else {
                        onsessiondead.emit(sessionid)
                    }
                })
            },
            250,
        )
    }

    let content = match *msg {
        None => html! {},
        Some(msg) => match msg {
            TransferMsg::Copy {
                id: _,
                state,
                time_elapsed,
            } => {
                let prompt1 = "pasting a copy";
                format_session_state(state, time_elapsed, prompt1)
            }
            TransferMsg::Move {
                id: _,
                state,
                time_elapsed,
            } => {
                let prompt1 = "pasting a cut";
                format_session_state(state, time_elapsed, prompt1)
            }
        },
    };

    html! {
        <div class="file-transfer-session">
            { content }
        </div>
    }
}

fn format_session_state(state: TransferState, time_elapsed: Duration, prompt1: &str) -> Html {
    match state {
        pitou_core::msg::TransferState::Initializing(_) => {
            let value = "0";
            html! {
                <div>
                    <span> { prompt1 } </span>
                    <progress {value} max="100"></progress>
                </div>
            }
        }
        pitou_core::msg::TransferState::Active(TransferSize { current, total }) => {
            let c_level = format! {"{} of {}", PitouFileSize::new(current).format(), PitouFileSize::new(total).format()};
            let t_passed = format! {"{} elapsed", PitouDateTime::format_duration(time_elapsed) };
            let ratio = current as f64 / total as f64;
            let elapsed = time_elapsed.as_secs() as f64;
            let rem_time = if ratio == 0.0 {
                "__".into()
            } else {
                let cmp = ((elapsed / ratio) - elapsed).max(0.0) as u64;
                PitouDateTime::format_duration(Duration::from_secs(cmp))
            };
            let t_remaining = format! {"{} remaining", rem_time};
            let value = format!("{}", (ratio * 100 as f64).ceil());
            html! {
                <>
                    <div class="progress-box">
                        <div>
                            <progress {value} max="100"></progress>
                        </div>
                        <span> { c_level } </span>
                    </div>
                    <div class="info-box">
                        <div> { t_passed } </div>
                        <div> { t_remaining } </div>
                    </div>
                </>
            }
        }
        pitou_core::msg::TransferState::Terminated(TransferSize { current, total: _ }) => {
            let prompt2 = format! {"{} of {}", PitouFileSize::new(current).format(), PitouFileSize::new(current).format()};
            let value = "100";
            html! {
                <div>
                    <span> { prompt1 } </span>
                    <span> { prompt2 } </span>
                    <progress {value} max="100"></progress>
                </div>
            }
        }
    }
}
