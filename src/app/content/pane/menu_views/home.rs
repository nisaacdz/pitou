use std::rc::Rc;

use pitou_core::{frontend::*, *};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::app::{
    reusables::{DiskIcon, DriveItems, GeneralFolderElems, NoArg},
    ApplicationContext,
};
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

pub async fn obtain_drives(data: Rc<StaticData>, after: impl Fn()) {
    let drives = tauri_sys::tauri::invoke::<NoArg, DriveItems>("drives", &NoArg)
        .await
        .unwrap_or_default();
    data.update_drives(drives.items);
    after()
}

#[function_component]
pub fn DrivesSection() -> Html {
    let ctx: ApplicationContext = use_context::<ApplicationContext>().unwrap();
    let refresher = use_force_update();
    {
        let data = ctx.static_data.clone();
        let refresher = refresher.clone();
        use_effect_with((), move |()| {
            let data = data.clone();
            spawn_local(async move {
                obtain_drives(data, move || refresher.force_update()).await;
            })
        })
    }
    {
        let data = ctx.static_data.clone();
        let refresher = refresher.clone();
        use_interval(
            move || {
                let data = data.clone();
                let refresher = refresher.clone();
                spawn_local(async move {
                    obtain_drives(data, move || refresher.force_update()).await;
                })
            },
            5000,
        );
    }

    let drives = ctx.static_data.drives.borrow().clone();

    let content = drives
        .iter()
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
    let ctx = use_context::<ApplicationContext>().unwrap();
    let highlighted = use_state_eq(|| {
        ctx.static_data
            .is_selected(VWrapper::Drive(props.drive.clone()))
    });
    let class = format!(
        "drives-section-elem{}",
        if *highlighted { " selected" } else { "" }
    );

    let onclick = {
        let ctx = ctx.clone();
        let drive = props.drive.clone();
        let highlighted = highlighted.clone();
        move |_| {
            if ctx.static_data.is_selected(VWrapper::Drive(drive.clone())) {
                ctx.static_data
                    .clear_selection(VWrapper::Drive(drive.clone()));
                highlighted.set(false);
            } else {
                ctx.static_data
                    .add_selection(VWrapper::Drive(drive.clone()));
                highlighted.set(true);
            }
        }
    };

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
        <div {class} {onclick}>
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
