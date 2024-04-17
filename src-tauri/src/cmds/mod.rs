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
