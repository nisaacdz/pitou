use std::{cell::RefCell, collections::HashSet, rc::Rc, time::Duration};

use pitou_core::{
    msg::{TransferMsg, TransferSessionID, TransferSize, TransferState},
    PitouDateTime, PitouFileSize,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component]
pub fn Status() -> Html {
    html! {
        <div id="status-bar">
            <TransfersWatcher />
        </div>
    }
}

#[function_component]
pub fn SearchWatcher() -> Html {
    let searches = use_state(|| false);
    {
        let searches = searches.clone();
        use_interval(
            move || {
                let searches = searches.clone();
                spawn_local(async move {
                    let msg = false;
                    searches.set(msg);
                })
            },
            250,
        )
    }
    html! { <div id="status-bar"></div> }
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
