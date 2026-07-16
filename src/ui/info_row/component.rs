use dioxus::prelude::*;

#[css_module("/src/ui/info_row/style.css")]
struct Styles;

/// The "label / value" row of a detail page: a muted label on the left, and a
/// right-aligned, wrappable value on the right.
#[component]
pub fn InfoRow(label: String, value: String) -> Element {
    rsx! {
        div { class: Styles::dx_info_row,
            span { class: Styles::dx_info_row_label, "{label}" }
            span { class: Styles::dx_info_row_value, "{value}" }
        }
    }
}
