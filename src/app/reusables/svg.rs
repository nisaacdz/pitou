use pitou_core::GeneralFolder;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GenFolderProps {
    pub folder: Rc<GeneralFolder>,
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

pub fn video_file() -> Html {
    html! {
        <svg viewBox="0 0 24 24">
            <path d="M19.71,6.29l-4-4A1,1,0,0,0,15,2H6A2,2,0,0,0,4,4V20a2,2,0,0,0,2,2H18a2,2,0,0,0,2-2V7A1,1,0,0,0,19.71,6.29Z" class="fill-secondary-spare"/>
            <path d="M11,14.5a1,1,0,0,1-.45-.11A1,1,0,0,1,10,13.5v-3a1,1,0,0,1,1.6-.8l2,1.5a1,1,0,0,1,0,1.6l-2,1.5A1,1,0,0,1,11,14.5Z" class="fill-primary-spare"/>
        </svg>
    }
}

pub fn audio_file() -> Html {
    html! {
        <svg viewBox="0 0 36 36">
            <path class="fill-primary-spare" d="M18 0C9.716 0 3 6.716 3 15v9h3v-9C6 8 11.269 2.812 18 2.812C24.73 2.812 30 8 30 15v10l3-1v-9c0-8.284-6.716-15-15-15z" />
            <path class="fill-secondary-foreground" d="M6 27a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2v-9a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v9zm30 0a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-9a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v9z" />
            <path class="fill-secondary-spare" d="M19.182 10.016l-6.364 1.313c-.45.093-.818.544-.818 1.004v16.185a6.218 6.218 0 0 0-2.087-.36c-2.785 0-5.042 1.755-5.042 3.922c0 2.165 2.258 3.827 5.042 3.827C12.649 35.905 14.922 34 15 32V16.39l4.204-.872c.449-.093.796-.545.796-1.004v-3.832c0-.458-.368-.759-.818-.666zm8 3.151l-4.297.865c-.45.093-.885.544-.885 1.003V26.44c0-.152-.878-.24-1.4-.24c-2.024 0-3.633 1.276-3.633 2.852c0 1.574 1.658 2.851 3.683 2.851s3.677-1.277 3.677-2.851l-.014-11.286l2.869-.598c.45-.093.818-.544.818-1.003v-2.33c0-.459-.368-.76-.818-.668z" />
        </svg>
    }
}

pub fn spreadsheet_file() -> Html {
    html! {
        <img src="./public/spreadsheet_file.svg"/>
    }
}

pub fn doc_file() -> Html {
    html! {
        <img src="./public/doc_file.svg"/>
    }
}

pub fn pdf_file() -> Html {
    html! {
        <svg viewBox="0 0 32 32">
            <path d="M2 12.1333C2 8.58633 2 6.81283 2.69029 5.45806C3.29749 4.26637 4.26637 3.29749 5.45806 2.69029C6.81283 2 8.58633 2 12.1333 2H19.8667C23.4137 2 25.1872 2 26.5419 2.69029C27.7336 3.29749 28.7025 4.26637 29.3097 5.45806C30 6.81283 30 8.58633 30 12.1333V19.8667C30 23.4137 30 25.1872 29.3097 26.5419C28.7025 27.7336 27.7336 28.7025 26.5419 29.3097C25.1872 30 23.4137 30 19.8667 30H12.1333C8.58633 30 6.81283 30 5.45806 29.3097C4.26637 28.7025 3.29749 27.7336 2.69029 26.5419C2 25.1872 2 23.4137 2 19.8667V12.1333Z" fill="#B30B00"/>
            <path d="M24.0401 17.8976C22.7327 16.464 19.1701 17.0912 18.3094 17.1808C17.0891 15.9264 16.2284 14.504 15.8798 13.9664C16.3156 12.6224 16.6642 11.1104 16.6642 9.6768C16.6642 8.3328 16.1413 7 14.7576 7C14.2347 7 13.7989 7.2688 13.5374 7.7168C12.9273 8.792 13.1887 10.9312 14.1475 13.16C13.6245 14.7728 12.753 17.1808 11.7179 19.0512C10.3234 19.5888 7.28369 21.0112 7.02221 22.624C6.93505 23.072 7.10937 23.6096 7.45801 23.8784C7.80665 24.2368 8.24244 24.3264 8.67824 24.3264C10.4977 24.3264 12.328 21.7392 13.6354 19.4096C14.6814 19.0512 16.3265 18.5136 17.9825 18.2448C19.8891 20.0368 21.6323 20.2944 22.5039 20.2944C23.7242 20.2944 24.16 19.7568 24.3234 19.3088C24.5522 18.8832 24.3887 18.256 24.0401 17.8976ZM22.8199 18.7936C22.7327 19.152 22.2969 19.5104 21.5125 19.3312C20.5537 19.0624 19.693 18.6144 18.9958 17.9872C19.6059 17.8976 21.0767 17.7184 22.1226 17.8976C22.4712 17.9872 22.907 18.256 22.8199 18.7936ZM14.3872 8.0752C14.4744 7.896 14.6487 7.8064 14.823 7.8064C15.2588 7.8064 15.3459 8.344 15.3459 8.792C15.2588 9.8672 15.0845 11.0208 14.7358 12.0064C14.0386 10.0464 14.1257 8.6128 14.3872 8.0752ZM14.3 18.1664C14.7358 17.36 15.2588 15.848 15.4331 15.3104C15.8689 16.1168 16.6533 17.0128 17.002 17.4496C17.0891 17.3712 15.5203 17.7184 14.3 18.1664ZM11.3475 20.2272C10.1382 22.1872 9.00509 23.4416 8.30781 23.4416C8.22065 23.4416 8.04634 23.4416 7.95918 23.352C7.87202 23.1728 7.78486 22.9936 7.87202 22.8144C7.95918 22.0976 9.35373 21.112 11.3475 20.2272Z" fill="white"/>
        </svg>
    }
}

pub fn app_file() -> Html {
    html! {
        <svg viewBox="0 0 24 24" >
            <g transform="translate(0 -1028.4)">
                <path d="m5 1030.4c-1.1046 0-2 0.9-2 2v8 4 6c0 1.1 0.8954 2 2 2h14c1.105 0 2-0.9 2-2v-6-4-4l-6-6h-10z" fill="#95a5a6"/>
                <path d="m5 1029.4c-1.1046 0-2 0.9-2 2v8 4 6c0 1.1 0.8954 2 2 2h14c1.105 0 2-0.9 2-2v-6-4-4l-6-6h-10z" fill="#bdc3c7"/>
                <path d="m13.41 1042.8c0.391-0.4 0.586-0.9 0.586-1.4 0-0.6-0.195-1.1-0.586-1.5-0.39-0.3-0.862-0.5-1.414-0.5s-1.023 0.2-1.414 0.5c-0.391 0.4-0.5859 0.9-0.5859 1.5 0 0.5 0.1949 1 0.5859 1.4s0.862 0.6 1.414 0.6 1.024-0.2 1.414-0.6m4.586-2.3v1.7c0 0.1-0.021 0.2-0.062 0.2-0.042 0.1-0.094 0.1-0.157 0.1l-1.445 0.2c-0.099 0.3-0.2 0.6-0.305 0.8 0.183 0.2 0.461 0.6 0.836 1 0.052 0.1 0.078 0.2 0.078 0.2 0 0.1-0.023 0.2-0.07 0.2-0.141 0.2-0.398 0.5-0.773 0.9-0.375 0.3-0.62 0.5-0.735 0.5-0.062 0-0.13 0-0.203-0.1l-1.078-0.8c-0.229 0.1-0.466 0.2-0.711 0.3-0.083 0.7-0.159 1.2-0.226 1.4-0.037 0.2-0.131 0.3-0.282 0.3h-1.734c-0.073 0-0.138-0.1-0.195-0.1-0.052 0-0.081-0.1-0.086-0.2l-0.219-1.4c-0.255-0.1-0.49-0.2-0.7032-0.3l-1.1016 0.8c-0.0521 0.1-0.1172 0.1-0.1953 0.1-0.0729 0-0.138 0-0.1953-0.1-0.6563-0.6-1.0859-1-1.2891-1.3-0.0364 0-0.0547-0.1-0.0547-0.2 0 0 0.0209-0.1 0.0625-0.2 0.0782-0.1 0.211-0.2 0.3985-0.5 0.1875-0.2 0.3281-0.4 0.4219-0.5-0.1407-0.3-0.2474-0.5-0.3204-0.8l-1.4296-0.2c-0.0678 0-0.1224 0-0.1641-0.1s-0.0625-0.1-0.0625-0.2v-1.7c0-0.1 0.0208-0.1 0.0625-0.2s0.0911-0.1 0.1484-0.1l1.4532-0.2c0.0729-0.3 0.1744-0.5 0.3046-0.7-0.2083-0.3-0.4869-0.7-0.8359-1.1-0.0521-0.1-0.0781-0.1-0.0781-0.2s0.0234-0.1 0.0703-0.2c0.1354-0.2 0.3906-0.5 0.7656-0.8 0.3802-0.4 0.6276-0.6 0.7422-0.6 0.0677 0 0.1354 0 0.2031 0.1l1.0782 0.8c0.2288-0.1 0.4658-0.2 0.7108-0.3 0.083-0.7 0.159-1.2 0.227-1.4 0.036-0.2 0.13-0.2 0.281-0.2h1.734 0.188c0.057 0.1 0.088 0.1 0.094 0.2l0.218 1.4c0.255 0.1 0.49 0.2 0.703 0.3l1.11-0.8c0.047-0.1 0.109-0.1 0.187-0.1 0.068 0 0.133 0 0.196 0.1 0.671 0.6 1.101 1.1 1.289 1.3 0.036 0.1 0.054 0.1 0.054 0.2s-0.02 0.1-0.062 0.2c-0.078 0.1-0.211 0.3-0.399 0.5-0.187 0.2-0.328 0.4-0.421 0.5 0.135 0.3 0.242 0.6 0.32 0.8l1.43 0.2c0.067 0 0.122 0.1 0.164 0.1 0.041 0.1 0.062 0.1 0.062 0.2" fill="#7f8c8d"/>
                <path d="m21 1035.4-6-6v4c0 1.1 0.895 2 2 2h4z" fill="#95a5a6"/>
            </g>
        </svg>
    }
}

pub fn txt_file() -> Html {
    html! {
        <img src="./public/text_file2.png"/>
    }
}

pub fn code_file() -> Html {
    html! {
        <svg viewBox="0 0 24 24" >
            <path d="M2 12C2 7.28595 2 4.92893 3.46447 3.46447C4.92893 2 7.28595 2 12 2C16.714 2 19.0711 2 20.5355 3.46447C22 4.92893 22 7.28595 22 12C22 16.714 22 19.0711 20.5355 20.5355C19.0711 22 16.714 22 12 22C7.28595 22 4.92893 22 3.46447 20.5355C2 19.0711 2 16.714 2 12Z" fill="#00B7EF"/>
            <path d="M13.4881 6.44591C13.8882 6.55311 14.1256 6.96437 14.0184 7.36447L11.4302 17.0237C11.323 17.4238 10.9117 17.6613 10.5116 17.5541C10.1115 17.4468 9.8741 17.0356 9.98131 16.6355L12.5695 6.97624C12.6767 6.57614 13.088 6.3387 13.4881 6.44591Z" fill="#1C274C"/>
            <path d="M14.9697 8.46967C15.2626 8.17678 15.7374 8.17678 16.0303 8.46967L16.2387 8.67801C16.874 9.3133 17.4038 9.84308 17.7678 10.3202C18.1521 10.8238 18.4216 11.3559 18.4216 12C18.4216 12.6441 18.1521 13.1762 17.7678 13.6798C17.4038 14.1569 16.874 14.6867 16.2387 15.322L16.0303 15.5303C15.7374 15.8232 15.2626 15.8232 14.9697 15.5303C14.6768 15.2374 14.6768 14.7626 14.9697 14.4697L15.1412 14.2981C15.8229 13.6164 16.2797 13.1574 16.5753 12.7699C16.8577 12.3998 16.9216 12.1843 16.9216 12C16.9216 11.8157 16.8577 11.6002 16.5753 11.2301C16.2797 10.8426 15.8229 10.3836 15.1412 9.70191L14.9697 9.53033C14.6768 9.23744 14.6768 8.76257 14.9697 8.46967Z" fill="#1C274C"/>
            <path d="M7.96986 8.46967C8.26275 8.17678 8.73762 8.17678 9.03052 8.46967C9.32341 8.76257 9.32341 9.23744 9.03052 9.53033L8.85894 9.70191C8.17729 10.3836 7.72052 10.8426 7.42488 11.2301C7.14245 11.6002 7.07861 11.8157 7.07861 12C7.07861 12.1843 7.14245 12.3998 7.42488 12.7699C7.72052 13.1574 8.17729 13.6164 8.85894 14.2981L9.03052 14.4697C9.32341 14.7626 9.32341 15.2374 9.03052 15.5303C8.73762 15.8232 8.26275 15.8232 7.96986 15.5303L7.76151 15.322C7.12617 14.6867 6.59638 14.1569 6.23235 13.6798C5.84811 13.1762 5.57861 12.6441 5.57861 12C5.57861 11.3559 5.84811 10.8238 6.23235 10.3202C6.59638 9.84308 7.12617 9.31331 7.76151 8.67801L7.96986 8.46967Z" fill="#1C274C"/>
        </svg>
    }
}

pub fn pic_file() -> Html {
    html! {
        <svg viewBox="0 0 1024 1024" >
            <path d="M853.333333 874.666667H170.666667c-46.933333 0-85.333333-38.4-85.333334-85.333334V234.666667c0-46.933333 38.4-85.333333 85.333334-85.333334h682.666666c46.933333 0 85.333333 38.4 85.333334 85.333334v554.666666c0 46.933333-38.4 85.333333-85.333334 85.333334z" fill="#F57C00"/>
            <path d="M746.666667 341.333333m-64 0a64 64 0 1 0 128 0 64 64 0 1 0-128 0Z" fill="#FFF9C4"/>
            <path d="M426.666667 341.333333L192 682.666667h469.333333z" fill="#942A09"/>
            <path d="M661.333333 469.333333l-170.666666 213.333334h341.333333z" fill="#BF360C"/>
        </svg>
    }
}

pub fn archive_file() -> Html {
    html! {
        <img src="./public/archive_file.png"/>
    }
}

pub fn presentation_file() -> Html {
    html! {
        <img src="./public/ppt_file.svg" />
    }
}

pub fn other_file() -> Html {
    html! {
        <img src="./public/file.png"/>
    }
}

pub fn match_extension(pitou: &Rc<pitou_core::PitouFile>) -> Html {
    match pitou.extension().to_ascii_lowercase().as_str() {
        "mp3" => audio_file(),
        "mp4" => video_file(),
        "mkv" => video_file(),
        "doc" => doc_file(),
        "docx" => doc_file(),
        "pdf" => pdf_file(),
        "txt" => txt_file(),
        "exe" => app_file(),
        "msi" => app_file(),
        "png" => pic_file(),
        "jpg" => pic_file(),
        "html" => code_file(),
        "css" => code_file(),
        "py" => code_file(),
        "cpp" => code_file(),
        "java" => code_file(),
        "rs" => code_file(),
        "zip" => archive_file(),
        "tar" => archive_file(),
        "7z" => archive_file(),
        "xls" => spreadsheet_file(),
        "xlsx" => spreadsheet_file(),
        "ppt" => presentation_file(),
        "pptx" => presentation_file(),
        _ => other_file(),
    }
}
