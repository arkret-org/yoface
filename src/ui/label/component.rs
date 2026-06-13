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

/// ergonomic 版 `Label`:接受具名 `r#for`(关联控件 id)+ 可选 `class`,
/// 不依赖 primitives 的 `ReadSignal<String>` 信号 prop。便于后台调用点
/// 以 `Label { r#for: "field-id", class: "...", "文案" }` 直接调用。
///
/// 与上面信号驱动的 [`Label`] 并存:既有下游继续用 [`Label`],新调用点
/// 想要 `class` 透传与裸字符串 `for` 时用 [`LabelFor`]。
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
