use std::rc::Rc;

use pitou_core::{frontend::GeneralFolder, PitouDrive};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::app::reusables::{GeneralFolderElems, NoArg};
/*
place for drives,
place for connected devices
place for default folders
place for favorites
place for recents
*/
#[function_component]
pub fn HomeView() -> Html {
    html! {
        <div id="home-pane" class="fullpane">
            <DrivesSection />
            <FoldersSection />
            <FavoritesSection />
            <RecentsSection />
        </div>
    }
}

#[function_component]
pub fn DrivesSection() -> Html {
    let drives = use_state(|| None);
    {
        let drives = drives.clone();
        use_effect_with((), move |()| {
            spawn_local(async move {
                let res: Vec<PitouDrive> = tauri_sys::tauri::invoke("drives", &NoArg).await.unwrap();
                drives.set(Some(res))
            })
        })
    }
    html! {
        <div id="drives-section" class="home-section">
            <div class="home-section-dsc-text">{ "Drives & Devices" }</div>
        </div>
    }
}

#[function_component]
pub fn FoldersSection() -> Html {
    let folders = use_state(|| None);
    {
        let folders = folders.clone();
        use_effect_with((), move |()| {
            spawn_local(async move {
                let val: GeneralFolderElems = tauri_sys::tauri::invoke("general_folders", &NoArg)
                    .await
                    .unwrap();
                folders.set(Some(val.items))
            })
        });
    }

    let elems = folders
        .iter()
        .flatten()
        .map(|v| html! { <FoldersSectionItem folder = { v.clone() }/> })
        .collect::<Html>();
    html! {
        <div id="folders-section" class="home-section">
            <div class="home-section-dsc-text">{ "Important Folders" }</div>
            { elems }
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct FoldersSectionItemProps {
    folder: Rc<GeneralFolder>,
}

#[function_component]
fn FoldersSectionItem(prop: &FoldersSectionItemProps) -> Html {
    let src = match prop.folder.as_ref() {
        GeneralFolder::DocumentsFolder(_) => "./public/documents_folder.png",
        GeneralFolder::AudiosFolder(_) => "./public/audios_folder.png",
        GeneralFolder::PicturesFolder(_) => "./public/pictures_folder.png",
        GeneralFolder::VideosFolder(_) => "./public/videos_folder.png",
        GeneralFolder::DesktopFolder(_) => "./public/desktop_folder.png",
        GeneralFolder::DownloadsFolder(_) => "./public/downloads_folder.png",
    };

    html! {
        <div class="folders-section-elem">
            <img class="folders-section-elem-img" {src}/>
            <div class="folders-section-elem-name"> { prop.folder.name() } </div>
        </div>
    }
}

#[function_component]
pub fn FavoritesSection() -> Html {
    html! {
        <div id="favorites-section" class="home-section">
            <div class="home-section-dsc-text">{ "Favorites" }</div>
        </div>
    }
}

#[function_component]
pub fn RecentsSection() -> Html {
    html! {
        <div id="recents-section" class="home-section">
            <div class="home-section-dsc-text">{ "Recents" }</div>
        </div>
    }
}
