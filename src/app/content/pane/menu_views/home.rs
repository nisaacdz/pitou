use std::{path::PathBuf, rc::Rc};

use pitou_core::{frontend::*, *};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::app::reusables::{DiskIcon, DriveItems, GenFolderIco, GeneralFolderElems, NoArg};

#[derive(PartialEq, Properties)]
pub struct HomeViewProps {
    pub onopen: Callback<Rc<PitouFile>>,
}

#[function_component]
pub fn HomeView(props: &HomeViewProps) -> Html {
    let onopen = {
        let onopen = props.onopen.clone();
        Callback::from(move |pb: PathBuf| {
            let pf = PitouFile {
                path: pb.into(),
                metadata: None,
            };
            onopen.emit(Rc::new(pf))
        })
    };

    html! {
        <div id="home-pane" class="fullpane">
            <DrivesSection onopen={onopen.clone()}/>
            <FoldersSection onopen={onopen.clone()}/>
            <FavoritesSection onopen={onopen.clone()}/>
            <RecentsSection {onopen}/>
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

#[derive(PartialEq, Properties)]
struct DrivesSectionProps {
    onopen: Callback<PathBuf>,
}

#[function_component]
fn DrivesSection(props: &DrivesSectionProps) -> Html {
    let ctx: ApplicationContext = use_context::<ApplicationContext>().unwrap();
    let refresher = use_force_update();
    {
        let data = ctx.static_data.clone();
        let refresher = refresher.clone();
        use_effect_with(ctx.refresher_state(), move |_| {
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
            15000,
        );
    }

    let drives = ctx.static_data.drives.borrow().clone();

    let content = if let Some(drives) = drives {
        drives
            .iter()
            .map(|drive| {
                html! {
                    <DrivesSectionItem {drive} onopen={props.onopen.clone()}/>
                }
            })
            .collect::<Html>()
    } else {
        html! {}
    };
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
    onopen: Callback<PathBuf>,
}

#[function_component]
fn DrivesSectionItem(props: &DrivesSectionItemProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let force_update = use_force_update();

    let highlighted = ctx.static_data.is_selected_drive(props.drive.clone());

    let class = format!(
        "drives-section-elem {}",
        if highlighted {
            "selected"
        } else {
            "not-selected"
        }
    );

    let onclick = {
        let ctx = ctx.clone();
        let drive = props.drive.clone();
        let force_update = force_update.clone();
        move |_| {
            if ctx.static_data.is_selected_drive(drive.clone()) {
                ctx.static_data.clear_drive_selection(drive.clone());
            } else {
                ctx.static_data.select_drive(drive.clone());
            }
            force_update.force_update();
        }
    };

    let ondblclick = {
        let onopen = props.onopen.clone();
        let drive = props.drive.clone();
        move |_| onopen.emit(drive.mount_point.path.clone())
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
        <div {class} {onclick} {ondblclick}>
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

#[derive(PartialEq, Properties)]
struct FoldersSectionProps {
    onopen: Callback<PathBuf>,
}

#[function_component]
fn FoldersSection(props: &FoldersSectionProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let force_update = use_force_update();
    {
        let ctx = ctx.clone();
        let force_update = force_update.clone();
        use_effect_with(ctx.refresher_state(), move |_| {
            let ctx = ctx.clone();
            let force_update = force_update.clone();
            spawn_local(async move {
                let val = tauri_sys::tauri::invoke::<NoArg, GeneralFolderElems>(
                    "general_folders",
                    &NoArg,
                )
                .await
                .ok();
                ctx.static_data
                    .update_gen_dirs(val.map(|v| Rc::new(v.items)));
                force_update.force_update();
            })
        });
    }

    let elems = ctx.static_data.gen_dirs()
        .iter()
        .map(|v| v.iter())
        .flatten()
        .map(|v| html! { <FoldersSectionItem folder = { v.clone() } onopen={props.onopen.clone()}/> })
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
    onopen: Callback<PathBuf>,
    folder: Rc<GeneralFolder>,
}

#[function_component]
fn FoldersSectionItem(props: &FoldersSectionItemProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let force_update = use_force_update();

    let highlighted = ctx.static_data.is_selected_gen_folder(props.folder.clone());

    let onclick = {
        let ctx = ctx.clone();
        let folder = props.folder.clone();
        let force_update = force_update.clone();
        move |_| {
            if ctx.static_data.is_selected_gen_folder(folder.clone()) {
                ctx.static_data.clear_gen_folder_selection(folder.clone());
            } else {
                ctx.static_data.select_gen_folder(folder.clone());
            }
            force_update.force_update();
        }
    };

    let ondblclick = {
        let onopen = props.onopen.clone();
        let folder = props.folder.clone();
        move |_| onopen.emit(folder.path().path.clone())
    };

    let class = format!(
        "folders-section-elem {}",
        if highlighted {
            "selected"
        } else {
            "not-selected"
        }
    );

    html! {
        <div {class} {onclick} {ondblclick}>
            <GenFolderIco folder={props.folder.clone()}/>
            <div class="folders-section-elem-name"> { props.folder.name() } </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct FavoritesSectionProps {
    onopen: Callback<PathBuf>,
}

#[function_component]
fn FavoritesSection(_props: &FavoritesSectionProps) -> Html {
    html! {
        <div id="favorites-section" class="home-section">
            <div class="home-section-dsc-text">{ "Favorites" }</div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct RecentsSectionProps {
    onopen: Callback<PathBuf>,
}

#[function_component]
fn RecentsSection(_props: &RecentsSectionProps) -> Html {
    html! {
        <div id="recents-section" class="home-section">
            <div class="home-section-dsc-text">{ "Recents" }</div>
        </div>
    }
}
