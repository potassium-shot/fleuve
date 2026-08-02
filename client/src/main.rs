use dioxus::prelude::*;

mod components;

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

#[component]
fn Root() -> Element {
    let mut dnd_offset: Signal<Option<(f64, f64)>> = use_signal(|| None);
    use_context_provider(move || dnd_offset);

    rsx! {
        div {
            id: "canvas",
            width: "100%",
            height: "100%",
            position: "relative",

            crate::components::nodes::note::Note {
                position: (200, 160),
            }
        }

        div {
            id: "dnd_capture",
            position: "absolute",
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            z_index: 999,
            background_color: "red",
            display: if dnd_offset() { "block" } else { "none" },

            onmouseup: move |_| dnd_offset.set(None),
            onmousemove: move |e| {
                if let Some(offset) = dnd_offset.write().as_mut() {
                    offset.x += e.client_coordinates().x;
                    offset.y += e.client_coordinates().y;
                }
            }
        }
    }
}
