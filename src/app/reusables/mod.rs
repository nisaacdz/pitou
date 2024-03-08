use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ReusableItemProps {
    pub class: String,
    pub id: String,
}

#[function_component]
pub fn TrashIcon(props: &ReusableItemProps) -> Html {
    let class = props.class.clone();
    let id = props.class.clone();

    html! {
        <svg id={id} class={class} viewBox="0 0 512 512">
            <path d="M422.957,66.783H372.87V50.087C372.87,22.468,350.402,0,322.783,0H189.217c-27.619,0-50.087,22.468-50.087,50.087v16.696
            H89.044c-27.619,0-50.087,22.468-50.087,50.087v33.391c0,9.223,7.479,16.696,16.696,16.696h400.696
            c9.217,0,16.696-7.473,16.696-16.696V116.87C473.043,89.251,450.576,66.783,422.957,66.783z M339.478,66.783H172.522V50.087
            c0-9.206,7.49-16.696,16.696-16.696h133.565c9.206,0,16.696,7.49,16.696,16.696V66.783z"/>

            <path d="M72.348,200.348v261.565c0,27.619,22.468,50.087,50.087,50.087h267.13c27.619,0,50.087-22.468,50.087-50.087V200.348
            H72.348z M210.523,447.914l-17.837,1.152c-39.863,2.591-66.59-40.44-46.598-75.027l4.668-8.087
            c-6.801-0.587-12.808-5.293-14.679-12.266c-2.383-8.909,2.899-18.061,11.802-20.447l32.25-8.642
            c8.911-2.415,18.581,2.987,20.683,12.645l8.416,31.415c2.381,8.908-2.902,18.06-11.815,20.445
            c-6.812,1.843-13.786-0.853-17.791-6.378l-4.621,8.008c-6.681,11.572,2.22,25.902,15.532,25.006l17.837-1.152
            c9.087-0.462,17.141,6.375,17.74,15.587C226.697,439.376,219.73,447.315,210.523,447.914z M241.967,278.042l-7.924,16.022
            c-4.098,8.261-14.12,11.63-22.37,7.565c-8.26-4.092-11.651-14.102-7.564-22.369l7.924-16.022
            c17.694-35.775,68.31-37.436,88.272-2.837l4.68,8.104c3.923-5.589,11.025-8.458,17.961-6.598
            c8.902,2.386,14.185,11.543,11.804,20.451l-8.652,32.255c-2.351,8.747-11.608,14.55-21.119,11.621l-31.588-8.464
            c-8.902-2.386-14.185-11.538-11.804-20.445c1.846-6.879,7.733-11.547,14.419-12.24l-4.616-7.994
            C264.787,265.667,247.938,265.963,241.967,278.042z M321.608,450.783h-9.449c2.904,6.186,1.942,13.728-3.171,18.837
            c-6.521,6.521-17.087,6.521-23.609,0l-23.609-23.609c-6.445-6.446-6.825-17.552,0.693-24.302l22.915-22.921
            c6.521-6.521,17.087-6.521,23.609,0c5.044,5.04,6.083,12.462,3.324,18.603h9.295c13.392,0,21.279-14.884,13.891-25.956
            l-9.913-14.869c-5.12-7.674-3.043-18.038,4.63-23.152c7.664-5.109,18.023-3.05,23.154,4.63l9.913,14.869
            C385.454,406.145,361.568,450.783,321.608,450.783z"/>
        </svg>
    }
}

#[function_component]
pub fn LockedIcon(props: &ReusableItemProps) -> Html {
    let class = props.class.clone();
    let id = props.class.clone();
    html! {
        <svg {id} {class} viewBox="0 0 484.157 484.157">
            <path d="M372.196,232.606c-10.275,0-18.629,8.354-18.629,18.629v40.362h37.258v-40.362
                    C390.825,240.96,382.471,232.606,372.196,232.606z"/>
            <path d="M453.922,62.505H283.354c-17.036,0-30.851-13.821-30.851-30.859V27.77c0-15.335-12.435-27.77-27.786-27.77H92.726
                C77.407,0,64.972,12.434,64.972,27.77v3.876c0,17.038-13.814,30.859-30.85,30.859h-3.887c-15.349,0-27.784,12.45-27.784,27.785
                v222.646c0,18.439,14.956,33.381,33.363,33.381h73.255h35.618h101.86v-10.086c0-20.6,14.026-37.969,33.024-43.106v-41.891
                c0-51.08,41.552-92.624,92.624-92.624c51.072,0,92.624,41.544,92.624,92.624v41.891c6.312,1.703,11.993,4.855,16.872,8.937V160.486
                l0.015,0.017V90.29C481.706,74.955,469.271,62.505,453.922,62.505z"/>
            <path d="M453.195,307.736h-4.514v-56.501c0-42.175-34.311-76.485-76.486-76.485c-42.175,0-76.485,34.311-76.485,76.485v56.501
                h-4.514c-15.753,0-28.51,12.75-28.51,28.495v119.415c0,15.745,12.757,28.511,28.51,28.511h161.999
                c15.753,0,28.511-12.766,28.511-28.511V336.231C481.706,320.486,468.948,307.736,453.195,307.736z M387.837,398.248v20.724
                c0,8.638-6.998,15.651-15.642,15.651c-8.644,0-15.642-7.014-15.642-15.651v-20.724c-6.825-4.887-11.3-12.83-11.3-21.859
                c0-14.878,12.055-26.952,26.95-26.952c14.878,0,26.934,12.074,26.934,26.952C399.138,385.418,394.663,393.361,387.837,398.248z
                M406.963,307.736h-69.535v-56.501c0-19.164,15.603-34.768,34.767-34.768c19.164,0,34.767,15.604,34.767,34.768V307.736z"/>
        </svg>
    }
}

#[function_component]
pub fn CloudIcon(props: &ReusableItemProps) -> Html {
    let class = props.class.clone();
    let id = props.class.clone();
    html! {
        <svg {id} {class} viewBox="0 0 22 22">
            <path d="M17 9l-.351.015c-.825-2.377-3.062-4.015-5.649-4.015-3.309 0-6 2.691-6 6l.001.126c-1.724.445-3.001 2.013-3.001 3.874 0 2.206 1.794 4 4 4h5v-4.586l-1.293 1.293c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l2.999-2.999c.093-.093.203-.166.326-.217.244-.101.52-.101.764 0 .123.051.233.124.326.217l2.999 2.999c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-1.293-1.293v4.586h4c2.757 0 5-2.243 5-5s-2.243-5-5-5z"/>
        </svg>
    }
}

#[function_component]
pub fn ExplorerIcon(props: &ReusableItemProps) -> Html {
    let class = props.class.clone();
    let id = props.class.clone();
    html! {
        <svg {id} {class} viewBox="0 0 238.687 238.687">
            <path d="M9.287,104.344h220.114c3.039,0,5.808,1.135,7.595,3.11c0.874,0.965,1.871,2.556,1.664,4.641l-10.773,108.326
            c-0.372,3.742-4.438,6.923-9.258,6.923H22.96c-4.721,0-8.78-3.103-9.243-6.766L0.044,112.143c-0.266-2.106,0.712-3.691,1.579-4.676
            C3.38,105.475,6.245,104.344,9.287,104.344z M31.01,89.344V71.198c0-5.216,4.427-9.854,9.642-9.854h157.383
            c5.216,0,8.976,4.639,8.976,9.854v18.146h20V47.605c0-4.143-2.606-7.262-6.749-7.262H132.01v-4.467
            c0-14.037-9.021-24.533-20.723-24.533H31.793c-11.701,0-20.783,10.496-20.783,24.533v53.467H31.01z"/>
        </svg>
    }
}

#[function_component]
pub fn SettingsIcon(props: &ReusableItemProps) -> Html {
    let class = props.class.clone();
    let id = props.class.clone();

    html! {
        <svg {id} {class} viewBox="0 0 24 24">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M14.2788 2.15224C13.9085 2 13.439 2 12.5 2C11.561 2 11.0915 2 10.7212 2.15224C10.2274 2.35523 9.83509 2.74458 9.63056 3.23463C9.53719 3.45834 9.50065 3.7185 9.48635 4.09799C9.46534 4.65568 9.17716 5.17189 8.69017 5.45093C8.20318 5.72996 7.60864 5.71954 7.11149 5.45876C6.77318 5.2813 6.52789 5.18262 6.28599 5.15102C5.75609 5.08178 5.22018 5.22429 4.79616 5.5472C4.47814 5.78938 4.24339 6.1929 3.7739 6.99993C3.30441 7.80697 3.06967 8.21048 3.01735 8.60491C2.94758 9.1308 3.09118 9.66266 3.41655 10.0835C3.56506 10.2756 3.77377 10.437 4.0977 10.639C4.57391 10.936 4.88032 11.4419 4.88029 12C4.88026 12.5581 4.57386 13.0639 4.0977 13.3608C3.77372 13.5629 3.56497 13.7244 3.41645 13.9165C3.09108 14.3373 2.94749 14.8691 3.01725 15.395C3.06957 15.7894 3.30432 16.193 3.7738 17C4.24329 17.807 4.47804 18.2106 4.79606 18.4527C5.22008 18.7756 5.75599 18.9181 6.28589 18.8489C6.52778 18.8173 6.77305 18.7186 7.11133 18.5412C7.60852 18.2804 8.2031 18.27 8.69012 18.549C9.17714 18.8281 9.46533 19.3443 9.48635 19.9021C9.50065 20.2815 9.53719 20.5417 9.63056 20.7654C9.83509 21.2554 10.2274 21.6448 10.7212 21.8478C11.0915 22 11.561 22 12.5 22C13.439 22 13.9085 22 14.2788 21.8478C14.7726 21.6448 15.1649 21.2554 15.3694 20.7654C15.4628 20.5417 15.4994 20.2815 15.5137 19.902C15.5347 19.3443 15.8228 18.8281 16.3098 18.549C16.7968 18.2699 17.3914 18.2804 17.8886 18.5412C18.2269 18.7186 18.4721 18.8172 18.714 18.8488C19.2439 18.9181 19.7798 18.7756 20.2038 18.4527C20.5219 18.2105 20.7566 17.807 21.2261 16.9999C21.6956 16.1929 21.9303 15.7894 21.9827 15.395C22.0524 14.8691 21.9088 14.3372 21.5835 13.9164C21.4349 13.7243 21.2262 13.5628 20.9022 13.3608C20.4261 13.0639 20.1197 12.558 20.1197 11.9999C20.1197 11.4418 20.4261 10.9361 20.9022 10.6392C21.2263 10.4371 21.435 10.2757 21.5836 10.0835C21.9089 9.66273 22.0525 9.13087 21.9828 8.60497C21.9304 8.21055 21.6957 7.80703 21.2262 7C20.7567 6.19297 20.522 5.78945 20.2039 5.54727C19.7799 5.22436 19.244 5.08185 18.7141 5.15109C18.4722 5.18269 18.2269 5.28136 17.8887 5.4588C17.3915 5.71959 16.7969 5.73002 16.3099 5.45096C15.8229 5.17191 15.5347 4.65566 15.5136 4.09794C15.4993 3.71848 15.4628 3.45833 15.3694 3.23463C15.1649 2.74458 14.7726 2.35523 14.2788 2.15224ZM12.5 15C14.1695 15 15.5228 13.6569 15.5228 12C15.5228 10.3431 14.1695 9 12.5 9C10.8305 9 9.47716 10.3431 9.47716 12C9.47716 13.6569 10.8305 15 12.5 15Z" />
        </svg>
    }
}

#[function_component]
pub fn FavoritesIcon(props: &ReusableItemProps) -> Html {
    let class = props.class.clone();
    let id = props.class.clone();
    html! {
        <svg {id} {class} viewBox="0 0 476.91 476.909">
            <path d="M62.802,142.287h121.682c4.45,0,10.95-3.805,13.32-7.999l24.816-41.066H62.802C28.12,93.222,0,121.342,0,156.026v20.779
                C13.266,156.086,36.425,142.287,62.802,142.287z"/>
            <path d="M414.105,67.708H266.913c-8.681,0-19.151,6.125-23.399,13.685l-7.147,11.828l-28.489,47.157
                c-4.246,7.558-14.719,13.684-23.393,13.684H62.802C28.12,154.062,0,182.183,0,216.865v115.794v13.737
                c0,34.685,28.12,62.805,62.802,62.805h351.303c34.685,0,62.805-28.12,62.805-62.805v-13.737V156.026v-25.515
                C476.91,95.829,448.79,67.708,414.105,67.708z M155.45,245.138h52.638l16.268-50.063c7.787-23.962,20.4-23.962,28.19,0
                l16.267,50.063h52.638c25.194,0,29.086,11.998,8.712,26.801l-42.587,30.945l16.266,50.062
                c7.783,23.956-2.428,31.378-22.806,16.574l-42.579-30.941l-42.587,30.941c-20.382,14.804-30.593,7.382-22.808-16.574
                l16.268-50.062l-42.591-30.945C126.368,257.136,130.259,245.138,155.45,245.138z"/>
        </svg>
    }
}

#[function_component]
pub fn RecentsIcon(props: &ReusableItemProps) -> Html {
    let class = props.class.clone();
    let id = props.class.clone();

    html! {
        <svg {id} {class} viewBox="0 0 24 24">
            <path d="M18,19.75 C18,20.940864 17.0748384,21.9156449 15.9040488,21.9948092 L15.75,22 L8.25,22 C7.05913601,22 6.08435508,21.0748384 6.00519081,19.9040488 L6,19.75 L6,4.25 C6,3.05913601 6.92516159,2.08435508 8.09595119,2.00519081 L8.25,2 L15.75,2 C16.940864,2 17.9156449,2.92516159 17.9948092,4.09595119 L18,4.25 L18,19.75 Z M19,5 L19.75,5 C20.940864,5 21.9156449,5.92516159 21.9948092,7.09595119 L22,7.25 L22,16.75 C22,17.940864 21.0748384,18.9156449 19.9040488,18.9948092 L19.75,19 L19,19 L19,5 Z M5,19 L4.25,19 C3.05913601,19 2.08435508,18.0748384 2.00519081,16.9040488 L2,16.75 L2,7.25 C2,6.05913601 2.92516159,5.08435508 4.09595119,5.00519081 L4.25,5 L5,5 L5,19 Z">
            </path>
        </svg>
    }
}

#[function_component]
pub fn SearchIcon(props: &ReusableItemProps) -> Html {
    let class = props.class.clone();
    let id = props.class.clone();
    html! {
        <svg {id} {class} viewBox="0 0 512 512">
            <path d="M495.272,423.558c0,0-68.542-59.952-84.937-76.328c-24.063-23.938-33.69-35.466-25.195-54.931
                c37.155-75.78,24.303-169.854-38.72-232.858c-79.235-79.254-207.739-79.254-286.984,0c-79.245,79.264-79.245,207.729,0,287.003
                c62.985,62.985,157.088,75.837,232.839,38.691c19.466-8.485,31.022,1.142,54.951,25.215c16.384,16.385,76.308,84.937,76.308,84.937
                c31.089,31.071,55.009,11.95,69.368-2.39C507.232,478.547,526.362,454.638,495.272,423.558z M286.017,286.012
                c-45.9,45.871-120.288,45.871-166.169,0c-45.88-45.871-45.88-120.278,0-166.149c45.881-45.871,120.269-45.871,166.169,0
                C331.898,165.734,331.898,240.141,286.017,286.012z"/>
        </svg>
    }
}

#[function_component]
pub fn HomeIcon(props: &ReusableItemProps) -> Html {
    let class = props.class.clone();
    let id = props.class.clone();

    html! {
        <svg {id} {class} viewBox="0 0 500 500">
            <path d="M487.083,225.514l-75.08-75.08V63.704c0-15.682-12.708-28.391-28.413-28.391c-15.669,0-28.377,12.709-28.377,28.391
                        v29.941L299.31,37.74c-27.639-27.624-75.694-27.575-103.27,0.05L8.312,225.514c-11.082,11.104-11.082,29.071,0,40.158
                        c11.087,11.101,29.089,11.101,40.172,0l187.71-187.729c6.115-6.083,16.893-6.083,22.976-0.018l187.742,187.747
                        c5.567,5.551,12.825,8.312,20.081,8.312c7.271,0,14.541-2.764,20.091-8.312C498.17,254.586,498.17,236.619,487.083,225.514z"/>
            <path d="M257.561,131.836c-5.454-5.451-14.285-5.451-19.723,0L72.712,296.913c-2.607,2.606-4.085,6.164-4.085,9.877v120.401
                c0,28.253,22.908,51.16,51.16,51.16h81.754v-126.61h92.299v126.61h81.755c28.251,0,51.159-22.907,51.159-51.159V306.79
                c0-3.713-1.465-7.271-4.085-9.877L257.561,131.836z"/>
        </svg>
    }
}