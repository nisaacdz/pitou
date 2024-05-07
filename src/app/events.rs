use serde::{de::DeserializeOwned, Serialize};
use tokio_stream::StreamExt;

pub async fn listen_event<P: 'static + DeserializeOwned>(eventname: &str, after: impl FnOnce(P)) {
    let mut events = tauri_sys::event::listen::<P>(eventname).await.unwrap();
    if let Some(event) = events.next().await {
        after(event.payload)
    }
}

pub async fn emit_event<P: Serialize>(eventname: &str, payload: &P) {
    tauri_sys::event::emit(eventname, payload).await.unwrap();
}
