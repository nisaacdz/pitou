use pitou_core::*;

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
pub async fn paste(pitou: PitouFile) {
    pitou_core::backend::paste(pitou).await
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
