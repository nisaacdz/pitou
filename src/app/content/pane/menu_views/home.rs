use std::rc::Rc;

use pitou_core::{frontend::GeneralFolder, PitouDrive};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::app::reusables::{DiskIcon, DriveItems, GeneralFolderElems, NoArg};
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
        use_interval(
            move || {
                let drives = drives.clone();
                spawn_local(async move {
                    let new_drives =
                        tauri_sys::tauri::invoke::<NoArg, DriveItems>("drives", &NoArg)
                            .await
                            .ok();
                    drives.set(new_drives.map(|d| d.items))
                })
            },
            5000,
        );
    }

    let content = drives
        .as_ref()
        .map(|v| v.iter())
        .into_iter()
        .flatten()
        .map(|drive| {
            html! {
                <DrivesSectionItem {drive}/>
            }
        })
        .collect::<Html>();
    html! {
        <div id="drives-section" class="home-section">
            <div class="home-section-dsc-text">{ "Drives & Devices" }</div>
            { content }
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct DrivesSectionItemProps {
    drive: Rc<PitouDrive>,
}

#[function_component]
fn DrivesSectionItem(props: &DrivesSectionItemProps) -> Html {
    let kind = props.drive.kind;
    let port = props.drive.mount_point.name();
    let name = {
        let val = if props.drive.name.is_empty() {
            if props.drive.is_removable {
                "External Disk"
            } else {
                "Local Disk"
            }
        } else {
            &props.drive.name
        };

        format!("{val} ({port})")
    };
    let val = 100f64 * (props.drive.total_space - props.drive.free_space) as f64
        / props.drive.total_space as f64;
    let guage_inner_style = format!("width: {:.0}%;", val);
    let dsc = {
        let free = props.drive.free_space as f64 / f64::powi(1024f64, 3);
        let total = props.drive.total_space as f64 / f64::powi(1024f64, 3);
        format! {"{:.0} GB free of {:.0} GB", free, total}
    };
    html! {
        <div class="drives-section-elem">
            <div class="drives-section-elem-icon">
                <DiskIcon {kind} />
            </div>
            <div class="drives-section-elem-name">{ name }</div>
            <div class="drives-section-elem-guage">
                <div class="drives-section-elem-guage-outer">
                    <div class="drives-section-elem-guage-inner" style={guage_inner_style}></div>
                </div>
            </div>
            <div class="drives-section-elem-desc">{ dsc }</div>
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
