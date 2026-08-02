use dioxus::prelude::*;

pub mod note;

#[component]
fn Node(
    position: (i32, i32),
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut dnd_offset = use_context::<Signal<Option<(f64, f64)>>>();
    let mut dnd_active_is_self = use_signal(|| false);

    let position = use_signal(move || (position.0 as f64, position.1 as f64));
    let final_position = use_memo(move || {
        if let Some(offset) = dnd_offset() {
            let pos = position();
            (pos.0 + offset.0, pos.1 + offset.1)
        } else {
            position()
        }
    });

    rsx! {
        div {
            id: "node",
            class: "block",
            position: "absolute",
            left: "{final_position().0}px",
            top: "{final_position().1}px",

            onmousedown: move |_| {
                dnd_offset.set(Some((0.0, 0.0)));
                dnd_active_is_self.set(true);
            },

            ..attributes,

            {children}
        }
    }
}
