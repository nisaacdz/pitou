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
    PitouDrive::get_drives()
}

#[tauri::command]
pub async fn children(
    dir: PitouFilePath,
    filter: PitouFileFilter,
    sort: Option<PitouFileSort>,
) -> Option<Vec<PitouFile>> {
    pitou_core::backend::children(dir, filter, sort).await.ok()
}
