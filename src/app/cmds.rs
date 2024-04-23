use std::rc::Rc;

use pitou_core::*;

use super::reusables::{ItemsArg, NoArg, PitouArg};

pub async fn open(pitou: Rc<PitouFile>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("open", &PitouArg { pitou }).await
}

pub async fn open_with(pitou: Rc<PitouFile>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("open_with", &PitouArg { pitou }).await
}

pub async fn paste(pitou: Rc<PitouFile>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("paste", &PitouArg { pitou }).await
}

pub async fn copy(items: &Vec<Rc<PitouFile>>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("copy", &ItemsArg { items }).await
}

pub async fn cut(items: &Vec<Rc<PitouFile>>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("cut", &ItemsArg { items }).await
}

pub async fn delete(items: &Vec<Rc<PitouFile>>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("delete", &ItemsArg { items }).await
}

pub async fn clipboard_empty() -> Result<bool, tauri_sys::Error> {
    tauri_sys::tauri::invoke("clipboard_empty", &NoArg).await
}
