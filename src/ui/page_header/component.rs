use dioxus::prelude::*;

#[css_module("/src/ui/page_header/style.css")]
struct Styles;

/// 页面标题区:标题 + 可选描述 + 右侧动作槽(`children`)。
#[component]
pub fn PageHeader(
    title: String,
    #[props(default)] description: String,
    #[props(default)] children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_page_header,
            div { class: Styles::dx_page_header_titles,
                h1 { class: Styles::dx_page_header_title, "{title}" }
                if !description.is_empty() {
                    p { class: Styles::dx_page_header_desc, "{description}" }
                }
            }
            div { class: Styles::dx_page_header_actions, {children} }
        }
    }
}

/// 面包屑条目。`href` 为 `None` 时渲染为当前页(不可点)。
///
/// 解耦说明:sodmin 原实现用 `crate::router::Route` 强类型路由。yoface
/// 作为共享库不假设下游路由类型,改用 `href: Option<String>` + `<a>`。
/// 下游若用 dioxus-router,可在外层包一层把 `Route` 渲染成 `<a href>` 即可。
#[derive(Debug, Clone, PartialEq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

#[component]
pub fn Breadcrumbs(items: Vec<BreadcrumbItem>) -> Element {
    rsx! {
        nav { class: Styles::dx_breadcrumbs,
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    span { class: Styles::dx_breadcrumb_sep, "/" }
                }
                if let Some(href) = item.href.clone() {
                    a { href: "{href}", class: Styles::dx_breadcrumb_link, "{item.label}" }
                } else {
                    span { class: Styles::dx_breadcrumb_current, "{item.label}" }
                }
            }
        }
    }
}
