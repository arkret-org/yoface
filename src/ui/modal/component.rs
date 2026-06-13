use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};

#[css_module("/src/ui/modal/style.css")]
struct Styles;

/// 居中的全屏遮罩 + 内容容器,是后台各列表/详情页大量手写
/// `fixed inset-0 z-50 …` 弹窗的 ergonomic 封装。
///
/// `open` 为 `false` 时渲染空。点击遮罩触发 `on_close`;`title` 非空时渲染
/// 标准 `h2` 头部,表单字段与 [`DialogActions`] 行作为 `children` 传入。
///
/// 与 yoface 既有 [`crate::ui::dialog::Dialog`](信号驱动的 dioxus-primitives
/// 封装)互补:`Modal` 走 `open: bool` + 回调的简洁 API,内部不接 primitives
/// 的 `Signal<Option<bool>>`,适合「布尔状态 + 关闭回调」的后台调用点。
#[component]
pub fn Modal(
    open: bool,
    #[props(default)] title: String,
    /// 透传到内容容器的额外 class(例如自定义最大宽度)。
    #[props(default)]
    class: String,
    on_close: EventHandler<()>,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    let panel_class = if class.is_empty() {
        Styles::dx_modal_panel.to_string()
    } else {
        format!("{} {}", Styles::dx_modal_panel, class)
    };

    rsx! {
        ModalOverlay { on_close,
            div { class: panel_class,
                if !title.is_empty() {
                    h2 { class: Styles::dx_modal_title, "{title}" }
                }
                {children}
            }
        }
    }
}

/// 裸的居中遮罩(背景 + flex 容器),不含标准弹窗面板。
/// 当某页需要自定义面板外壳但仍想复用共享的遮罩 + 点击关闭行为时使用。
///
/// 点击遮罩背景触发 `on_close`;内容区域阻止冒泡,避免点击内部误关。
#[component]
pub fn ModalOverlay(on_close: EventHandler<()>, children: Element) -> Element {
    rsx! {
        div { class: Styles::dx_modal_overlay,
            div {
                class: Styles::dx_modal_backdrop,
                onclick: move |_| on_close.call(()),
            }
            {children}
        }
    }
}

/// 弹窗尾部的确认/取消按钮行(右对齐),复用 yoface [`Button`]。
///
/// `confirm_loading` 在 mutation 进行中禁用确认按钮;`variant` 决定确认按钮
/// 样式(默认 [`ButtonVariant::Primary`],销毁操作传 [`ButtonVariant::Destructive`])。
#[component]
pub fn DialogActions(
    #[props(default = "Confirm".to_string())] confirm_label: String,
    #[props(default = "Cancel".to_string())] cancel_label: String,
    #[props(default)] variant: ButtonVariant,
    #[props(default = false)] confirm_loading: bool,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: Styles::dx_dialog_actions,
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| on_cancel.call(()),
                "{cancel_label}"
            }
            Button {
                variant,
                disabled: confirm_loading,
                onclick: move |_| on_confirm.call(()),
                "{confirm_label}"
            }
        }
    }
}

/// 确认对话框:标题 + 可选说明 + 确认/取消按钮行。`open: bool` + 回调驱动。
///
/// `variant` 决定确认按钮样式(默认 [`ButtonVariant::Primary`];销毁操作传
/// [`ButtonVariant::Destructive`])。点击遮罩等同于取消(触发 `on_cancel`)。
#[component]
pub fn ConfirmDialog(
    open: bool,
    title: String,
    #[props(default)] message: String,
    #[props(default = "Confirm".to_string())] confirm_label: String,
    #[props(default = "Cancel".to_string())] cancel_label: String,
    #[props(default)] variant: ButtonVariant,
    #[props(default = false)] confirm_loading: bool,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    if !open {
        return rsx! {};
    }

    rsx! {
        ModalOverlay { on_close: move |_| on_cancel.call(()),
            div { class: format!("{} {}", Styles::dx_modal_panel, Styles::dx_confirm_panel),
                div { class: Styles::dx_confirm_head,
                    h2 { class: Styles::dx_modal_title, "{title}" }
                    if !message.is_empty() {
                        p { class: Styles::dx_confirm_message, "{message}" }
                    }
                }
                DialogActions {
                    confirm_label,
                    cancel_label,
                    variant,
                    confirm_loading,
                    on_confirm: move |_| on_confirm.call(()),
                    on_cancel: move |_| on_cancel.call(()),
                }
            }
        }
    }
}
