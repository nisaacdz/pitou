#![allow(unused)]
use pitou_core::{
    msg::{SearchMsg, TransferMsg, TransferSessionID},
    search::SimplifiedSearchOptions,
    *,
};

#[tauri::command]
pub fn general_folders() -> Vec<GeneralFolder> {
    pitou_core::backend::general_folders()
}

#[tauri::command]
pub fn default_folder() -> PitouFile {
    pitou_core::backend::default_folder()
}

#[tauri::command]
pub fn drives() -> Vec<PitouDrive> {
    pitou_core::backend::drives()
}

#[tauri::command]
pub async fn children(
    dir: PitouFilePath,
    filter: PitouFileFilter,
    sort: Option<PitouFileSort>,
) -> Option<Vec<PitouFile>> {
    pitou_core::backend::children(dir, filter, sort).await.ok()
}

#[tauri::command]
pub fn thrash_items() -> Option<Vec<PitouTrashItem>> {
    pitou_core::backend::trash_items()
}

#[tauri::command]
pub async fn clipboard_empty() -> bool {
    pitou_core::backend::clipboard::is_empty().await
}

#[tauri::command]
pub async fn delete(items: Vec<PitouFile>) {
    pitou_core::backend::delete(items)
}

#[tauri::command]
pub async fn paste(pitou: PitouFile) -> Option<msg::TransferSessionID> {
    pitou_core::backend::transfer::paste_items(pitou.path).await
}

#[tauri::command]
pub async fn copy(items: Vec<PitouFile>) {
    pitou_core::backend::copy(items).await
}

#[tauri::command]
pub async fn cut(items: Vec<PitouFile>) {
    pitou_core::backend::cut(items).await
}

#[tauri::command]
pub async fn open(pitou: PitouFile) {
    pitou_core::backend::open(pitou.path).ok();
}

#[tauri::command]
pub async fn open_with(pitou: PitouFile) {
    pitou_core::backend::open_with(pitou.path).ok();
}

#[tauri::command]
pub async fn archive(pitou: Vec<PitouFilePath>) {
    let len = pitou.len();
}

#[tauri::command]
pub async fn create_dir(pitou: PitouFile) {
    pitou_core::backend::create_dir(pitou.path).await
}

#[tauri::command]
pub async fn create_file(pitou: PitouFile) {
    pitou_core::backend::create_file(pitou.path).await
}

#[tauri::command]
pub async fn rename(pitou: PitouFile, name: String) {
    pitou_core::backend::rename(pitou.path, name).await
}

#[tauri::command]
pub async fn extract(pitou: PitouFile) {
    let _ = pitou;
}

#[tauri::command]
pub async fn search(options: SimplifiedSearchOptions) {
    if let Some(options) = options.try_into() {
        pitou_core::backend::search::search(options).await;
    }
}

#[tauri::command]
pub async fn terminate_search() {
    pitou_core::backend::search::terminate_search().await
}

#[tauri::command]
pub async fn search_msg() -> SearchMsg {
    pitou_core::backend::search::read_stream().await
}

#[tauri::command]
pub async fn is_searching() -> bool {
    !pitou_core::backend::search::is_terminated().await
}

#[tauri::command]
pub async fn transfer_sessions() -> Vec<TransferMsg> {
    pitou_core::backend::transfer::get_all_active_sessions()
}

#[tauri::command]
pub async fn transfer_session_with_id(value: TransferSessionID) -> Option<TransferMsg> {
    pitou_core::backend::transfer::get_session_with_id(value)
}
