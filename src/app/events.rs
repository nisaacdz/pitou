use serde::{de::DeserializeOwned, Serialize};
use tokio_stream::StreamExt;

pub async fn listen_event<P: 'static + DeserializeOwned>(eventname: &str, after: impl FnOnce(P)) {
    let mut events = tauri_sys::event::listen::<P>(eventname).await.unwrap();
    web_sys::console::log_1(&serde_wasm_bindgen::to_value(&format!("beginning wait for : {}", eventname)).unwrap());
    if let Some(event) = events.next().await {
        web_sys::console::log_1(&serde_wasm_bindgen::to_value(&format!("found some: {}", eventname)).unwrap());
        after(event.payload)
    } else {
        web_sys::console::log_1(&serde_wasm_bindgen::to_value(&format!("found none: {}", eventname)).unwrap());
    }
}

pub async fn emit_event<P: Serialize>(eventname: &str, payload: &P) {
    tauri_sys::event::emit(eventname, payload).await.unwrap();
    web_sys::console::log_1(&serde_wasm_bindgen::to_value(&format!("emited event: {}", eventname)).unwrap());
}