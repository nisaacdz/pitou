use pitou_core::{frontend::*, *};
use serde_wasm_bindgen::to_value;
use web_sys::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SettingsViewProps {
    pub onupdatetheme: Callback<ColorTheme>,
}

#[function_component]
pub fn SettingsView(props: &SettingsViewProps) -> Html {
    html! {
        <div id="settings-pane" class="fullpane">
            <Themes onupdatetheme={props.onupdatetheme.clone()}/>
            <Extensions />
            <SystemFiles />
            <RefreshRate />
            <Siblings />
            <FilesView />
            <Thumbnails />
            <Zoom />
            <LoadSettings />
            <ResetSettings />
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ThemesProps {
    onupdatetheme: Callback<ColorTheme>,
}

#[function_component]
pub fn Themes(props: &ThemesProps) -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let cur_theme = ctx.color_theme();
    let onchange = {
        let onupdatetheme = props.onupdatetheme.clone();
        move |e: Event| {
            let val = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
            let theme = match val.parse::<u8>().unwrap() {
                0 => ColorTheme::DEFAULT_DARK,
                1 => ColorTheme::DEFAULT_LIGHT,
                2 => ColorTheme::GEM_DARK,
                3 => ColorTheme::POLISH_DARK,
                _ => unimplemented!(),
            };
            onupdatetheme.emit(theme)
        }
    };

    html! {
        <div class="selectable">
            <label class="label">{ "Themes" }</label>
            <select class="selector" {onchange} value="1">
                <option value="0" selected={cur_theme == ColorTheme::DEFAULT_DARK}>{ "Default Dark" }</option>
                <option value="1" selected={cur_theme == ColorTheme::DEFAULT_LIGHT}>{ "Default Light" }</option>
                <option value="2" selected={cur_theme == ColorTheme::GEM_DARK}>{ "Gem Dark" }</option>
                <option value="3" selected={cur_theme == ColorTheme::POLISH_DARK}>{ "Polish Dark" }</option>
            </select>
        </div>
    }
}

#[function_component]
pub fn Extensions() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();

    let onchange = {
        let ctx = ctx.clone();
        move |e: Event| {
            let state = e.target_dyn_into::<HtmlInputElement>().unwrap().checked();
            ctx.update_show_extensions(state);
        }
    };

    let checked = ctx.show_extensions();

    html! {
        <div class="selectable">
            <label class="label">{ "Show extensions" }</label>
            <input class="selector" type="checkbox" {onchange} {checked}/>
        </div>
    }
}

#[function_component]
pub fn SystemFiles() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let onchange = {
        let ctx = ctx.clone();
        move |e: Event| {
            let state = e.target_dyn_into::<HtmlInputElement>().unwrap().checked();
            ctx.update_hide_system_files(state)
        }
    };

    let checked = ctx.hide_system_files();

    html! {
        <div class="selectable">
            <label class="label">{ "Hide System Files" }</label>
            <input class="selector" type="checkbox" {checked} {onchange}/>
        </div>
    }
}

#[function_component]
pub fn RefreshRate() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let onchange = {
        let ctx = ctx.clone();
        move |e: Event| {
            let val = e
                .target_dyn_into::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse()
                .unwrap_or(AppSettings::default_refresh_rate());
            ctx.update_refresh_rate(val)
        }
    };
    let value = ctx.refresh_rate().to_string();

    html! {
        <div class="selectable">
            <label class="label">{ "Automatic refresh rate" }</label>
            <input class="selector" type="number" {value} {onchange} min="1" max="60"/>
        </div>
    }
}

#[function_component]
pub fn Siblings() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let onchange = {
        move |_| {
            let val = ctx.show_parents();
            ctx.toggle_show_parents(!val);
        }
    };
    html! {
        <div class="selectable">
            <label class="label">{ "Show siblings panel" }</label>
            <input class="selector" type="checkbox" checked={false} {onchange} />
        </div>
    }
}

#[function_component]
pub fn Thumbnails() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let onchange = {
        let ctx = ctx.clone();
        move |_| {
            let val = ctx.show_thumbnails();
            ctx.toggle_show_thumbnails(!val);
        }
    };

    let checked = ctx.show_thumbnails();

    html! {
        <div class="selectable">
            <label class="label">{ "Show thumbnails" }</label>
            <input class="selector" type="checkbox" {checked} {onchange} />
        </div>
    }
}

#[function_component]
pub fn Zoom() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();
    let onchange = {
        move |e: Event| {
            let val = e
                .target_dyn_into::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse()
                .unwrap();
            ctx.update_zoom_value(val);
        }
    };
    html! {
        <div class="selectable">
            <label class="label">{ "Zoom" }</label>
            <input class="selector" type="number" value="0" {onchange} min="-10" max="10"/>
        </div>
    }
}

#[function_component]
pub fn FilesView() -> Html {
    let ctx = use_context::<ApplicationContext>().unwrap();

    let onchange = {
        let ctx = ctx.clone();
        move |e: Event| {
            let val = e.target_dyn_into::<HtmlSelectElement>().unwrap().value();
            let view = match val.parse::<u8>().unwrap() {
                0 => ItemsView::Tiles,
                1 => ItemsView::Grid,
                2 => ItemsView::Rows,
                _ => unreachable!(),
            };
            ctx.update_items_view(view);
        }
    };

    let items_view = ctx.items_view();

    html! {
        <div class="selectable">
            <label class="label">{ "Files view" }</label>
            <select class="selector" {onchange}>
                <option value="0" selected={items_view == ItemsView::Tiles}>{ "Tiles" }</option>
                <option value="1" selected={items_view == ItemsView::Grid}>{ "Grid" }</option>
                <option value="2" selected={items_view == ItemsView::Rows}>{ "List" }</option>
            </select>
        </div>
    }
}

#[function_component]
pub fn LoadSettings() -> Html {
    let oninput = { move |_| () };

    html! {
        <div class="selectable">
            <label class="label">{ "Load a saved settings file" }</label>
            <input class="selector" type="file" accept=".json"{oninput}/>
        </div>
    }
}

#[function_component]
pub fn ResetSettings() -> Html {
    let onclick = { move |_| () };

    html! {
        <div class="selectable">
            <label class="label">{ "Revert to default settings" }</label>
            <button class="selector" {onclick}/>
        </div>
    }
}
