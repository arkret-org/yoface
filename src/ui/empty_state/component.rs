use dioxus::prelude::*;

#[css_module("/src/ui/empty_state/style.css")]
struct Styles;

/// 空列表 / 无数据占位:可选图标 + 标题 + 描述 + 可选动作链接。
///
/// 解耦说明:sodmin 原实现用本地 `icons::Icon { name }` 字符串图标表
/// 与 router `Link`。yoface 改为:
///   * `icon` 是可选 `Element`——下游传 `yoface::dioxus_icons` 的任意图标
///     组件即可(`icon: rsx!{ Inbox { } }`),不再维护字符串图标注册表;
///   * 动作用普通 `<a href>`,不绑定具体路由类型。
#[component]
pub fn EmptyState(
    title: String,
    description: String,
    #[props(default)] icon: Option<Element>,
    #[props(default)] action_label: Option<String>,
    #[props(default)] action_href: Option<String>,
) -> Element {
    rsx! {
        div { class: Styles::dx_empty_state,
            if let Some(icon) = icon {
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
