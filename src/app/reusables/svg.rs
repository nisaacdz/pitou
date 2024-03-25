use pitou_core::GeneralFolder;
use yew::prelude::*;
use std::rc::Rc;


#[derive(Properties, PartialEq)]
pub struct GenFolderProps {
    pub folder: Rc<GeneralFolder>
}

#[function_component]
pub fn GenFolderIco(props: &GenFolderProps) -> Html {
    html! {
        <div class="folders-section-elem-img">
            <GenFolderIcoOuter />
            <GenFolderIcoInner folder={props.folder.clone()}/>
        </div>
    }
}

#[function_component]
pub fn GenFolderIcoOuter() -> Html {
    html! {
        <svg class="folders-section-elem-img-outer" viewBox="0 0 22 22">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M8.39445 0C8.7288 0 9.041 0.1671 9.2265 0.4453L10.6328 2.5547C10.8182 2.8329 11.1305 3 11.4648 3H20C21.1046 3 22 3.89543 22 5V18C22 19.1046 21.1046 20 20 20H2C0.89543 20 0 19.1046 0 18L0 2C0 0.89543 0.89543 0 2 0L8.39445 0z"/>
        </svg>
    }
}


#[function_component]
fn GenFolderIcoInner(props: &GenFolderProps) -> Html {
    match &*props.folder {
        GeneralFolder::DocumentsFolder(_) => html! { <DocumentsFolder /> },
        GeneralFolder::AudiosFolder(_) => html! { <AudiosFolder /> },
        GeneralFolder::PicturesFolder(_) => html! { <PicturesFolder /> },
        GeneralFolder::VideosFolder(_) => html! { <VideosFolder /> },
        GeneralFolder::DesktopFolder(_) => html! { <DesktopFolder /> },
        GeneralFolder::DownloadsFolder(_) => html! { <DownloadsFolder /> },
    }
}

#[function_component]
pub fn DocumentsFolder() -> Html {
    html! {
        <svg class="folders-section-elem-img-inner" viewBox="0 0 342 342">
            <rect x="45.885" y="7.5" class="fill-secondary-spare" width="173.23" height="250"/>
            <polygon class="fill-secondary-spare" points="219.115,47.5 219.115,257.5 85.885,257.5 85.885,297.5 259.115,297.5 259.115,47.5 	"/>
            <path class="fill-primary-spare" d="M259.115,40h-32.5V7.5c0-4.142-3.357-7.5-7.5-7.5H45.885c-4.143,0-7.5,3.358-7.5,7.5v250
                c0,4.142,3.357,7.5,7.5,7.5h32.5v32.5c0,4.142,3.357,7.5,7.5,7.5h173.23c4.143,0,7.5-3.358,7.5-7.5v-250
                C266.615,43.358,263.258,40,259.115,40z M53.385,15h158.23c0,8.349,0,226.321,0,235c-5.558,0-147.952,0-158.23,0
                C53.385,250,53.385,15,53.385,15z M251.615,290H93.385v-25h125.73c4.143,0,7.5-3.358,7.5-7.5V55h25V290z"/>
            <path class="fill-primary-spare" d="M92.465,78.713h80.07c4.143,0,7.5-3.358,7.5-7.5s-3.357-7.5-7.5-7.5h-80.07
                c-4.142,0-7.5,3.358-7.5,7.5C84.965,75.355,88.322,78.713,92.465,78.713z"/>
            <path class="fill-primary-spare" d="M92.465,122.211h80.07c4.143,0,7.5-3.358,7.5-7.5s-3.357-7.5-7.5-7.5h-80.07
                c-4.142,0-7.5,3.358-7.5,7.5S88.322,122.211,92.465,122.211z"/>
            <path class="fill-primary-spare" d="M92.465,165.709h80.07c4.143,0,7.5-3.358,7.5-7.5s-3.357-7.5-7.5-7.5h-80.07
                c-4.142,0-7.5,3.358-7.5,7.5S88.322,165.709,92.465,165.709z"/>
        </svg>
    }
}

#[function_component]
pub fn AudiosFolder() -> Html {
    html! {
        <svg class="folders-section-elem-img-inner" viewBox="2 0 24 24">
            <path class="fill-secondary-spare stroke-primary-spare" d="M10,18a3.28,3.28,0,0,1-3.5,3A3.28,3.28,0,0,1,3,18a3.28,3.28,0,0,1,3.5-3A3.28,3.28,0,0,1,10,18Zm7.5-5A3.28,3.28,0,0,0,14,16a3.28,3.28,0,0,0,3.5,3A3.28,3.28,0,0,0,21,16,3.28,3.28,0,0,0,17.5,13Z" stroke-width="2">
            </path>
            <polyline class="stroke-primary-spare fill-none" points="10 18 10 5 21 3 21 16" stroke-linecap="round" stroke-linejoin="round" stroke-width="2">
            </polyline>
            <path class="fill-secondary-spare stroke-primary-spare" d="M21,7,10,9M6.5,15A3.28,3.28,0,0,0,3,18a3.28,3.28,0,0,0,3.5,3A3.28,3.28,0,0,0,10,18,3.28,3.28,0,0,0,6.5,15Zm11-2A3.28,3.28,0,0,0,14,16a3.28,3.28,0,0,0,3.5,3A3.28,3.28,0,0,0,21,16,3.28,3.28,0,0,0,17.5,13Z" stroke-linecap="round" stroke-linejoin="round" stroke-width="2">
            </path>
        </svg>
    }
}

#[function_component]
pub fn VideosFolder() -> Html {
    html! {
        <svg class="folders-section-elem-img-inner fill-secondary-spare" viewBox="0 0 24 24">
            <path class="stroke-primary-spare" d="M8.50989 2.00001H15.49C15.7225 1.99995 15.9007 1.99991 16.0565 2.01515C17.1643 2.12352 18.0711 2.78958 18.4556 3.68678H5.54428C5.92879 2.78958 6.83555 2.12352 7.94337 2.01515C8.09917 1.99991 8.27741 1.99995 8.50989 2.00001Z"/>
            <path class="stroke-primary-spare" d="M6.31052 4.72312C4.91989 4.72312 3.77963 5.56287 3.3991 6.67691C3.39117 6.70013 3.38356 6.72348 3.37629 6.74693C3.77444 6.62636 4.18881 6.54759 4.60827 6.49382C5.68865 6.35531 7.05399 6.35538 8.64002 6.35547H15.5321C17.1181 6.35538 18.4835 6.35531 19.5639 6.49382C19.9833 6.54759 20.3977 6.62636 20.7958 6.74693C20.7886 6.72348 20.781 6.70013 20.773 6.67691C20.3925 5.56287 19.2522 4.72312 17.8616 4.72312H6.31052Z"/>
            <path class="stroke-primary-spare" fill-rule="evenodd" clip-rule="evenodd" d="M15.3276 7.54204H8.67239C5.29758 7.54204 3.61017 7.54204 2.66232 8.52887C1.71447 9.5157 1.93748 11.0403 2.38351 14.0896L2.80648 16.9811C3.15626 19.3724 3.33115 20.568 4.22834 21.284C5.12553 22 6.4488 22 9.09534 22H14.9046C17.5512 22 18.8745 22 19.7717 21.284C20.6689 20.568 20.8437 19.3724 21.1935 16.9811L21.6165 14.0896C22.0625 11.0404 22.2855 9.51569 21.3377 8.52887C20.3898 7.54204 18.7024 7.54204 15.3276 7.54204ZM14.5812 15.7942C15.1396 15.4481 15.1396 14.5519 14.5812 14.2058L11.2096 12.1156C10.6669 11.7792 10 12.2171 10 12.9099V17.0901C10 17.7829 10.6669 18.2208 11.2096 17.8844L14.5812 15.7942Z"/>
        </svg>
    }
}

#[function_component]
pub fn DesktopFolder() -> Html {
    html! {
        <svg class="folders-section-elem-img-inner fill-secondary-spare" viewBox="0 0 24 24">
            <path class="stroke-primary-spare" d="M3 7C3 5.11438 3 4.17157 3.58579 3.58579C4.17157 3 5.11438 3 7 3H12H17C18.8856 3 19.8284 3 20.4142 3.58579C21 4.17157 21 5.11438 21 7V10V13C21 14.8856 21 15.8284 20.4142 16.4142C19.8284 17 18.8856 17 17 17H12H7C5.11438 17 4.17157 17 3.58579 16.4142C3 15.8284 3 14.8856 3 13V10V7Z" />
            <path class="stroke-primary-spare" d="M3 7C3 5.11438 3 4.17157 3.58579 3.58579C4.17157 3 5.11438 3 7 3H12H17C18.8856 3 19.8284 3 20.4142 3.58579C21 4.17157 21 5.11438 21 7V10V13C21 14.8856 21 15.8284 20.4142 16.4142C19.8284 17 18.8856 17 17 17H12H7C5.11438 17 4.17157 17 3.58579 16.4142C3 15.8284 3 14.8856 3 13V10V7Z" stroke-width="2" stroke-linejoin="round"/>
            <path class="stroke-primary-spare" d="M7 21H17" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path class="stroke-primary-spare" d="M12 17V21" stroke-width="2" stroke-linecap="round"/>
        </svg>
    }
}

#[function_component]
pub fn DownloadsFolder() -> Html {
    html! {
        <svg class="folders-section-elem-img-inner fill-secondary-spare" viewBox="0 0 48 48">
            <path class="stroke-primary-spare" fill-rule="evenodd" clip-rule="evenodd" d="M23.9999 29L12 17L19.9999 17L19.9999 5.99999L27.9999 5.99999L27.9999 17L35.9999 17L23.9999 29Z" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/> 
            <path class="stroke-primary-spare" d="M42 37L6 37" stroke-width="4" stroke-linecap="round"/> 
            <path class="stroke-primary-spare" d="M34 44H14" stroke-width="4" stroke-linecap="round"/>
        </svg>
    }
}

#[function_component]
pub fn PicturesFolder() -> Html {
    html! {
        <svg class="folders-section-elem-img-inner" viewBox="0 0 512 512">
            <rect class="stroke-primary-spare" height="259" stroke="black" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="35" width="394" x="59" y="126.5"/> 
            <polyline points=" 59,385.5 230,269.5 294,385.5 " class="stroke-primary-spare" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="20"/> 
            <polyline points=" 262,327.5 345,269.5 453,385.5 " class="stroke-primary-spare" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="20"/> 
            <circle cx="245.5" cy="197.166" fill="none" r="25.5" class="stroke-primary-spare" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="20"/> 
            <rect class="fill-secondary-spare stroke-primary-spare" height="259" class="stroke-primary-spare" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="20" width="394" x="59" y="126.5"/> 
            <polyline class="fill-primary-spare" points=" 294,385.5 230,269.5 59,385.5 " class="stroke-primary-spare" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="20"/> 
            <polygon class="fill-primary-spare" points=" 262,327.5 345,269.5 453,385.5 294,385.5 " class="stroke-primary-spare" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="20"/> 
            <circle cx="245.5" cy="197.166" class="fill-primary-spare stroke-primary-spare" r="25.5" class="stroke-primary-spare" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" stroke-width="20"/>
        </svg>
    }
}