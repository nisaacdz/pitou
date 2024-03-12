use pitou_core::{frontend::GeneralFolder, PitouDrive, PitouFilePath};

#[tauri::command]
pub fn general_folders() -> Vec<GeneralFolder> {
    pitou_core::backend::general_folders()
}

#[tauri::command]
pub fn default_folder() -> PitouFilePath {
    pitou_core::backend::default_folder()
}

#[tauri::command]
pub fn drives() -> Vec<PitouDrive> {
    PitouDrive::get_drives()
}