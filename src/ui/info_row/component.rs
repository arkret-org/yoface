use dioxus::prelude::*;

#[css_module("/src/ui/info_row/style.css")]
struct Styles;

/// 详情页的「标签 / 值」行:左侧弱化标签,右侧右对齐、可换行的值。
#[component]
pub fn InfoRow(label: String, value: String) -> Element {
    rsx! {
        div { class: Styles::dx_info_row,
            span { class: Styles::dx_info_row_label, "{label}" }
            span { class: Styles::dx_info_row_value, "{value}" }
        }
    }
}
