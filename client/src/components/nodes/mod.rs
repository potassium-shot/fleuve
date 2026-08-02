use dioxus::prelude::*;

use crate::DndContext;

pub mod note;

#[component]
fn Node(
    position: (i32, i32),
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut dnd_context = use_context::<DndContext>();

    let position = use_signal(move || (position.0 as f64, position.1 as f64));
    let final_position = use_memo(move || {
        if (dnd_context.initial_offset)().is_some() {
            let offset = (dnd_context.offset)();
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

            onmousedown: move |e| {
                dnd_context.initial_offset.set(Some((e.client_coordinates().x, e.client_coordinates().y)));
                dnd_context.offset.set((0.0, 0.0));
            },

            ..attributes,

            {children}
        }
    }
}
