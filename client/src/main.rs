use std::{cell::Cell, collections::HashMap};

use dioxus::prelude::*;

use crate::utils::{CallbackSignal, NeverEq, use_callback_signal};

mod components;
mod constants;
mod utils;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Root {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const DEFAULT_LAYOUT_CSS: Asset = asset!("/assets/default-layout.css");
const DEFAULT_STYLE_CSS: Asset = asset!("/assets/default-style.css");
const DEFAULT_COLORS_CSS: Asset = asset!("/assets/default-colors.css");

fn main() {
    #[cfg(feature = "desktop")]
    let launch = dioxus::LaunchBuilder::desktop().with_cfg(
        dioxus::desktop::Config::default()
            .with_menu(None)
            .with_disable_context_menu(true),
    );

    #[cfg(feature = "web")]
    let launch = dioxus::LaunchBuilder::web().with_cfg(dioxus::web::Config::default());

    launch.launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap",
        }

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: DEFAULT_LAYOUT_CSS }
        document::Link { rel: "stylesheet", href: DEFAULT_STYLE_CSS }
        document::Link { rel: "stylesheet", href: DEFAULT_COLORS_CSS }
        Router::<Route> {}
    }
}

#[derive(Clone, Copy)]
struct MouseMovement(Memo<NeverEq<(f64, f64)>>);

#[derive(Clone, Copy)]
struct DndCancel(CallbackSignal);

#[derive(Default, Clone, Copy)]
struct DndConfirmData {
    confirmed: bool,
    selection: bool,
}

#[derive(Clone, Copy)]
struct DndConfirm(Signal<DndConfirmData>);

fn use_mouse_movement() -> Memo<NeverEq<(f64, f64)>> {
    use_context::<MouseMovement>().0
}

#[derive(Clone, Copy)]
struct Selection(Signal<HashMap<i64, Cell<(f64, f64)>>>);

fn use_selection() -> Signal<HashMap<i64, Cell<(f64, f64)>>> {
    use_context::<Selection>().0
}

#[component]
fn Root() -> Element {
    let mut last_mouse_coords = use_signal(|| (0.0_f64, 0.0_f64));
    let mut mouse_pos = use_signal(|| (0.0_f64, 0.0_f64));
    let mut dnd_cancel = use_callback_signal();
    use_context_provider(move || DndCancel(dnd_cancel));
    let dnd_confirm = use_signal(|| DndConfirmData {
        confirmed: false,
        selection: false,
    });
    use_context_provider(move || DndConfirm(dnd_confirm));

    let mouse_movement = use_memo(move || {
        let coords = mouse_pos();
        let mut last_coords = last_mouse_coords.write();
        let delta = (coords.0 - last_coords.0, coords.1 - last_coords.1);
        *last_coords = coords;
        NeverEq(delta)
    });
    use_context_provider(move || MouseMovement(mouse_movement));

    let mut selection = use_signal(HashMap::new);
    use_context_provider(move || Selection(selection));

    rsx! {
        div {
            id: "canvas",
            width: "100%",
            height: "100%",
            position: "absolute",

            onmousemove: move |e| {
                mouse_pos.set((e.client_coordinates().x, e.client_coordinates().y));
            },

            onmouseup: move |_| {
                dnd_cancel.trigger();
            },

            onmouseleave: move |_| {
                dnd_cancel.trigger();
            },

            onclick: move |_| {
                selection.write().clear();
            },

            crate::components::nodes::note::Note {
                node_id: 0,
                position: (200, 160),
            }

            crate::components::nodes::note::Note {
                node_id: 1,
                position: (400, 320),
            }
        }
    }
}
