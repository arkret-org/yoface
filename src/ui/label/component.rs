use dioxus::prelude::*;
use dioxus_primitives::label::{self, LabelProps};
#[css_module("/src/ui/label/style.css")]
struct Styles;

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label::Label {
            class: Styles::dx_label,
            html_for: props.html_for,
            attributes: props.attributes,
            {props.children}
        }
    }
}

/// The ergonomic flavor of `Label`: takes a named `r#for` (the id of the
/// associated control) + an optional `class`, without depending on the
/// primitives' `ReadSignal<String>` signal prop. This lets admin call sites
/// call it directly as `Label { r#for: "field-id", class: "...", "copy" }`.
///
/// Coexists with the signal-driven [`Label`] above: existing downstream keeps
/// using [`Label`], while new call sites that want `class` pass-through and a
/// bare-string `for` use [`LabelFor`].
#[component]
pub fn LabelFor(
    #[props(default)] r#for: String,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let cls = if class.is_empty() {
        Styles::dx_label.to_string()
    } else {
        format!("{} {}", Styles::dx_label, class)
    };
    rsx! {
        label { r#for, class: cls, {children} }
    }
}
