use std::rc::Rc;

use pitou_core::{
    msg::{SearchMsg, TransferMsg, TransferSessionID},
    search::SimplifiedSearchOptions,
    *,
};

use super::{
    args::ValueArg,
    reusables::{ItemsArg, NoArg, PitouArg, RenameArg, SearchOptionsArg},
};

pub async fn open(pitou: Rc<PitouFile>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("open", &PitouArg { pitou }).await
}

pub async fn open_with(pitou: Rc<PitouFile>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("open_with", &PitouArg { pitou }).await
}

pub async fn paste(pitou: Rc<PitouFile>) -> Result<TransferSessionID, tauri_sys::Error> {
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

pub async fn rename(pitou: Rc<PitouFile>, name: String) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("rename", &RenameArg { pitou, name }).await
}

pub async fn clipboard_empty() -> Result<bool, tauri_sys::Error> {
    tauri_sys::tauri::invoke("clipboard_empty", &NoArg).await
}

pub async fn archive(items: &Vec<Rc<PitouFile>>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("archive", &ItemsArg { items }).await
}

pub async fn extract(pitou: Rc<PitouFile>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("extract", &PitouArg { pitou }).await
}

pub async fn create_dir(pitou: Rc<PitouFile>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("create_dir", &PitouArg { pitou }).await
}

pub async fn create_file(pitou: Rc<PitouFile>) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("create_file", &PitouArg { pitou }).await
}

pub async fn copy_path(_pitou: Rc<PitouFile>) -> Result<(), tauri_sys::Error> {
    Ok(())
}

pub async fn search(options: SimplifiedSearchOptions) -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("search", &SearchOptionsArg { options }).await
}

pub async fn terminate_search() -> Result<(), tauri_sys::Error> {
    tauri_sys::tauri::invoke("terminate_search", &NoArg).await
}

pub async fn search_msg() -> Result<SearchMsg, tauri_sys::Error> {
    tauri_sys::tauri::invoke("search_msg", &NoArg).await
}

pub async fn transfer_session_with_id(value: TransferSessionID) -> Option<TransferMsg> {
    tauri_sys::tauri::invoke("transfer_session_with_id", &ValueArg { value })
        .await
        .unwrap()
}
