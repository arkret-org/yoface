//! A yoface checkbox variant supporting three states (checked / unchecked /
//! indeterminate).
//!
//! dioxus-primitives' `checkbox::Checkbox` only expresses the two states
//! `checked` / `unchecked` (the HTML `checked` attribute cannot express
//! indeterminate), so this module provides its **own implementation** over a
//! native `<input type=checkbox>`, without disturbing the existing
//! [`super::Checkbox`].
//!
//! `indeterminate` is not an HTML attribute; it can only be set as a DOM
//! property after render —— so this uses `dioxus::document::eval` (the
//! established yoface/inkson convention: no need to pull in `web-sys` /
//! `wasm-bindgen`, and a safe no-op on host) to sync `el.indeterminate` to the
//! prop after every render.
//!
//! A pure leaf: the value of the `onchange(bool)` callback is the **new
//! intent** after the DOM has toggled; the semantics of clicking while
//! indeterminate (usually "select all on the current page") are decided by the
//! parent component.

use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/ui/checkbox/style.css")]
struct Styles;

static TRISTATE_SEQ: AtomicU64 = AtomicU64::new(0);

/// A tristate checkbox. When `indeterminate` is true it renders the "partially
/// selected" dash state (used by the header select-all cell). Leaving `id`
/// empty auto-generates a stable unique id for the `indeterminate` reflection.
#[component]
pub fn TristateCheckbox(
    #[props(default)] id: String,
    #[props(default)] class: String,
    #[props(default = "Select".to_string())] aria_label: String,
    #[props(default)] checked: bool,
    #[props(default)] indeterminate: bool,
    #[props(default)] disabled: bool,
    #[props(default)] onchange: EventHandler<bool>,
) -> Element {
    // One stable id per instance (unchanged across renders), used by eval to
    // locate the element and reflect indeterminate onto it.
    let resolved_id = use_hook(|| {
        if id.is_empty() {
            format!(
                "dx-tristate-cb-{}",
                TRISTATE_SEQ.fetch_add(1, Ordering::Relaxed)
            )
        } else {
            id.clone()
        }
    });

    // After each render, reflect indeterminate onto the DOM property (an HTML
    // attribute cannot express it).
    {
        let id_for_eval = resolved_id.clone();
        use_effect(move || {
            let script = format!(
                "var el=document.getElementById('{id}');if(el){{el.indeterminate={flag};}}",
                id = id_for_eval,
                flag = if indeterminate { "true" } else { "false" },
            );
            let _ = document::eval(&script);
        });
    }

    let cls = if class.is_empty() {
        Styles::dx_checkbox_native.to_string()
    } else {
        format!("{} {}", Styles::dx_checkbox_native, class)
    };

    rsx! {
        input {
            id: resolved_id,
            r#type: "checkbox",
            class: cls,
            "aria-label": aria_label,
            checked,
            disabled,
            onchange: move |evt| {
                // The browser has already toggled checked before firing change;
                // read the new state from the event rather than negating a
                // stale prop (if the previous frame was indeterminate,
                // negating is wrong).
                let v = evt.value();
                let new_state = v == "true" || v == "on";
                onchange.call(new_state);
            },
        }
    }
}

/// Pure function: given the number of selected rows on the current page and
/// the total number of selectable rows on that page, derive the
/// `(checked, indeterminate)` of the header "select all" cell. An empty page
/// renders as unselected / non-indeterminate, so the header cannot "lie".
pub fn header_state(selected_on_page: usize, page_size: usize) -> (bool, bool) {
    if page_size == 0 || selected_on_page == 0 {
        (false, false)
    } else if selected_on_page >= page_size {
        (true, false)
    } else {
        (false, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_state_empty_page() {
        assert_eq!(header_state(0, 0), (false, false));
        assert_eq!(header_state(0, 10), (false, false));
    }

    #[test]
    fn header_state_partial() {
        assert_eq!(header_state(3, 10), (false, true));
        assert_eq!(header_state(1, 2), (false, true));
    }

    #[test]
    fn header_state_full() {
        assert_eq!(header_state(10, 10), (true, false));
        // Out-of-range count (defensive —— should not happen, but the behavior
        // stays stable)
        assert_eq!(header_state(11, 10), (true, false));
    }
}
