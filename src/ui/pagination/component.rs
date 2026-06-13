use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};

#[css_module("/src/ui/pagination/style.css")]
struct Styles;

/// 游标式 上一页 / 下一页 控件。`depth` 是调用方维护的游标栈深度
/// (1 == 第一页),回退只弹栈、不重新请求。
///
/// 文案默认英文,可经 `prev_label` / `next_label` 覆盖(便于下游本地化)。
#[component]
pub fn CursorPagination(
    /// 游标栈深度(1-indexed)。
    depth: usize,
    /// 当前页是否还有 `next_cursor`。
    has_next: bool,
    on_prev: EventHandler<()>,
    on_next: EventHandler<()>,
    #[props(default = "Previous".to_string())] prev_label: String,
    #[props(default = "Next".to_string())] next_label: String,
) -> Element {
    rsx! {
        div { class: Styles::dx_pagination,
            div { class: Styles::dx_pagination_info, {format!("Page {depth}")} }
            div { class: Styles::dx_pagination_controls,
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    disabled: depth <= 1,
                    onclick: move |_| on_prev.call(()),
                    "{prev_label}"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    disabled: !has_next,
                    onclick: move |_| on_next.call(()),
                    "{next_label}"
                }
            }
        }
    }
}

/// 经典 页码 / 总数 分页。
#[component]
pub fn Pagination(
    page: u64,
    total: u64,
    per_page: u64,
    on_page_change: EventHandler<u64>,
    #[props(default = "Previous".to_string())] prev_label: String,
    #[props(default = "Next".to_string())] next_label: String,
) -> Element {
    let total_pages = if total == 0 {
        1
    } else {
        total.div_ceil(per_page)
    };
    let from = if total == 0 {
        0
    } else {
        (page - 1) * per_page + 1
    };
    let to = std::cmp::min(page * per_page, total);

    rsx! {
        div { class: Styles::dx_pagination,
            div { class: Styles::dx_pagination_info, "Showing {from}-{to} of {total}" }
            div { class: Styles::dx_pagination_controls,
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    disabled: page <= 1,
                    onclick: move |_| on_page_change.call(page - 1),
                    "{prev_label}"
                }
                span { class: Styles::dx_pagination_info, "Page {page} of {total_pages}" }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    disabled: page >= total_pages,
                    onclick: move |_| on_page_change.call(page + 1),
                    "{next_label}"
                }
            }
        }
    }
}
