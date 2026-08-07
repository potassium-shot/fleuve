use dioxus::prelude::*;

use crate::{
    DndCancel, DndConfirm, DndConfirmData, constants::DRAG_DEADZONE, use_mouse_movement,
    use_selection,
};

pub mod note;

#[component]
fn Node(
    node_id: i64,
    position: (i32, i32),
    #[props(extends = GlobalAttributes)]
    #[props(extends = div)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let movement = use_mouse_movement();
    let dnd_cancel = use_context::<DndCancel>().0;
    let mut selection = use_selection();

    let mut position = use_signal(move || (position.0 as f64, position.1 as f64));
    let mut moved_position = use_signal(&*position);
    let mut dragging = use_signal(|| false);
    let mut dnd_confirm = use_context::<DndConfirm>().0;

    let mut mouse_down_pos = use_signal(|| (0.0, 0.0));

    use_memo(move || {
        dnd_cancel.get();
        dragging.set(false);
        dnd_confirm.set(DndConfirmData::default());
    });

    let final_position = use_memo(move || {
        let movement = movement();

        let mut write_moved_position = moved_position.write();
        let mut write_position = position.write();

        if dragging() {
            write_moved_position.0 += movement.0;
            write_moved_position.1 += movement.1;
        } else {
            *write_moved_position = *write_position;
        }

        let is_in_selection = selection.read().contains(&node_id);

        let confirm = dnd_confirm();

        if confirm.confirmed {
            if !dragging() && is_in_selection && confirm.selection {
                write_moved_position.0 += movement.0;
                write_moved_position.1 += movement.1;
            }

            *write_position = *write_moved_position;
        } else if (write_moved_position.0 - write_position.0).abs() > DRAG_DEADZONE
            || (write_moved_position.1 - write_position.1).abs() > DRAG_DEADZONE
        {
            let mut confirm_w = dnd_confirm.write();
            confirm_w.confirmed = true;
            confirm_w.selection = is_in_selection;
        }

        *write_position
    });

    rsx! {
        div {
            id: "node",
            class: "block",
            position: "absolute",
            left: "{final_position().0}px",
            top: "{final_position().1}px",
            "css-selected": selection.read().contains(&node_id),

            onmousedown: move |e| {
                dragging.set(true);
                mouse_down_pos.set((e.client_coordinates().x, e.client_coordinates().y));
            },

            onmouseup: move |e| {
                let down_pos = mouse_down_pos();

                if (e.client_coordinates().x - down_pos.0).abs() <= DRAG_DEADZONE && (e.client_coordinates().y - down_pos.1).abs() <= DRAG_DEADZONE {
                    if !e.modifiers().contains(Modifiers::CONTROL) {
                        selection.write().clear();
                    }

                    selection.write().insert(node_id);
                }
            },

            onclick: |e| {
                e.stop_propagation();
            },

            ..attributes,

            div { id: "marquee" }

            {children}
        }
    }
}
