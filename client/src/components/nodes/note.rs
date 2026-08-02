use dioxus::prelude::*;

use crate::components::nodes::Node;

#[component]
pub fn Note(position: (i32, i32)) -> Element {
    rsx! {
        Node {
            id: "note",
            position: position,

            p {
                "Label text!"
            }
        }
    }
}
