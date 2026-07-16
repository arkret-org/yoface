use dioxus::prelude::*;

#[css_module("/src/ui/page_header/style.css")]
struct Styles;

/// The page title area: title + optional description + the action slot on the
/// right (`children`).
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

/// A breadcrumb item. When `href` is `None` it renders as the current page
/// (not clickable).
///
/// Decoupling note: sodmin's original implementation used the strongly typed
/// `crate::router::Route`. yoface, being a shared library, makes no assumption
/// about downstream's route type and instead uses `href: Option<String>` +
/// `<a>`. If downstream uses dioxus-router, it can wrap a layer on the outside
/// that renders `Route` into an `<a href>`.
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
