use dioxus::prelude::*;
use dioxus_icons::lucide;

#[css_module("/src/ui/empty_state/style.css")]
struct Styles;

/// Maps the common lucide kebab-case icon names to the corresponding icon
/// `Element`. Covers every name used by sodmin's EmptyState call sites;
/// unrecognized names fall back to `Inbox`. Also accepts both lucide's old and
/// new official aliases (e.g. `alert-triangle` / `triangle-alert`).
fn lucide_icon_by_name(name: &str) -> Element {
    let sz = "2rem";
    match name {
        "bot" => rsx! { lucide::Bot { size: sz } },
        "search" => rsx! { lucide::Search { size: sz } },
        "alert-triangle" | "triangle-alert" => rsx! { lucide::TriangleAlert { size: sz } },
        "git-branch" => rsx! { lucide::GitBranch { size: sz } },
        "flag" => rsx! { lucide::Flag { size: sz } },
        "globe" => rsx! { lucide::Globe { size: sz } },
        "shield" => rsx! { lucide::Shield { size: sz } },
        "message-square" => rsx! { lucide::MessageSquare { size: sz } },
        "smartphone" => rsx! { lucide::Smartphone { size: sz } },
        "plug" => rsx! { lucide::Plug { size: sz } },
        "key" => rsx! { lucide::KeyRound { size: sz } },
        "users" => rsx! { lucide::Users { size: sz } },
        _ => rsx! { lucide::Inbox { size: sz } },
    }
}

/// Empty-list / no-data placeholder: optional icon + title + description +
/// optional action link.
///
/// Decoupling note: sodmin's original implementation used a local
/// `icons::Icon { name }` string icon table and the router `Link`. yoface
/// changes this to:
///   * `icon` is an optional `Element` —— downstream just passes any icon
///     component from `yoface::dioxus_icons` (`icon: rsx!{ Inbox { } }`); no
///     string icon registry is maintained anymore;
///   * the action uses a plain `<a href>`, not bound to any concrete route
///     type.
///
/// Two ways to give the icon (for convenience):
///   * `icon: Option<Element>` —— pass any icon component from
///     `yoface::dioxus_icons`;
///   * `icon_name: Option<String>` —— pass a lucide kebab-case name (e.g.
///     `"users"`), which [`lucide_icon_by_name`] maps to a built-in common
///     icon, letting downstream that calls with a "string icon name" (such as
///     sodmin) migrate with zero changes. When both are given, `icon` wins.
#[component]
pub fn EmptyState(
    title: String,
    description: String,
    #[props(default)] icon: Option<Element>,
    #[props(default)] icon_name: Option<String>,
    #[props(default)] action_label: Option<String>,
    #[props(default)] action_href: Option<String>,
) -> Element {
    let resolved_icon = icon.or_else(|| icon_name.as_deref().map(lucide_icon_by_name));
    rsx! {
        div { class: Styles::dx_empty_state,
            if let Some(icon) = resolved_icon {
                div { class: Styles::dx_empty_state_icon, {icon} }
            }
            h3 { class: Styles::dx_empty_state_title, "{title}" }
            p { class: Styles::dx_empty_state_desc, "{description}" }
            if let (Some(label), Some(href)) = (action_label, action_href) {
                a { href: "{href}", class: Styles::dx_empty_state_action, "{label}" }
            }
        }
    }
}
