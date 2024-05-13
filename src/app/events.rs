use serde::{de::DeserializeOwned, Serialize};
// use tokio_stream::StreamExt;

pub async fn listen_event<P: 'static + DeserializeOwned>(eventname: &str, after: impl FnOnce(P)) {
    if let Ok(event) = tauri_sys::event::once::<P>(eventname).await {
        after(event.payload)
    }
}

pub async fn listen_looping_event<P: 'static + DeserializeOwned>(
    eventname: &str,
    after: impl Fn(P),
) {
    use tokio_stream::StreamExt;

    let mut events = tauri_sys::event::listen::<P>(eventname).await.unwrap();
    while let Some(event) = events.next().await {
        after(event.payload)
    }
}

pub async fn emit_event<P: Serialize>(eventname: &str, payload: &P) {
    tauri_sys::event::emit(eventname, payload).await.unwrap();
}
