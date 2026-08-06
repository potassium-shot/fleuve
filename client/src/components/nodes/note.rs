use dioxus::prelude::*;

use crate::components::nodes::Node;

#[component]
pub fn Note(node_id: i64, position: (i32, i32)) -> Element {
    rsx! {
        Node {
            id: "note",
            node_id: node_id,
            position: position,

            p {
                "Label text!"
            }
        }
    }
}
