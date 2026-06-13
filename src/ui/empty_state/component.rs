use dioxus::prelude::*;
use dioxus_icons::lucide;

#[css_module("/src/ui/empty_state/style.css")]
struct Styles;

/// 把常用的 lucide kebab-case 图标名映射为对应图标 `Element`。
/// 覆盖 sodmin EmptyState 调用点用到的全部名字;未识别的名字回退到 `Inbox`。
/// 同时接受 lucide 官方新旧别名(如 `alert-triangle` / `triangle-alert`)。
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

/// 空列表 / 无数据占位:可选图标 + 标题 + 描述 + 可选动作链接。
///
/// 解耦说明:sodmin 原实现用本地 `icons::Icon { name }` 字符串图标表
/// 与 router `Link`。yoface 改为:
///   * `icon` 是可选 `Element`——下游传 `yoface::dioxus_icons` 的任意图标
///     组件即可(`icon: rsx!{ Inbox { } }`),不再维护字符串图标注册表;
///   * 动作用普通 `<a href>`,不绑定具体路由类型。
///
/// 图标二选一(便利):
///   * `icon: Option<Element>`——传 `yoface::dioxus_icons` 的任意图标组件;
///   * `icon_name: Option<String>`——传 lucide kebab-case 名字(如 `"users"`),
///     由 [`lucide_icon_by_name`] 映射到内置常用图标,便于以「字符串图标名」
///     调用的下游(如 sodmin)零改造迁移。两者都给时 `icon` 优先。
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
