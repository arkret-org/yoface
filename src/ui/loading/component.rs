use dioxus::prelude::*;

#[css_module("/src/ui/loading/style.css")]
struct Styles;

/// 通用骨架屏:标题条 + 三行文本。
#[component]
pub fn LoadingSkeleton() -> Element {
    rsx! {
        div { class: Styles::dx_skeleton_block,
            div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_title) }
            div { class: Styles::dx_skeleton_lines,
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_full) }
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_lg) }
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_md) }
            }
        }
    }
}

/* 统计卡片骨架网格。 */
#[component]
pub fn StatsSkeleton(#[props(default = 4)] count: u32) -> Element {
    rsx! {
        div { class: Styles::dx_skeleton_grid,
            for _ in 0..count {
                div { class: Styles::dx_skeleton_card,
                    div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_sm) }
                    div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_stat) }
                }
            }
        }
    }
}

/// 单个圆形旋转加载指示器,颜色取 `currentColor`。
#[component]
pub fn Spinner() -> Element {
    rsx! {
        svg {
            class: Styles::dx_spinner,
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            circle {
                class: Styles::dx_spinner_track,
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "4",
            }
            path {
                class: Styles::dx_spinner_head,
                fill: "currentColor",
                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
            }
        }
    }
}
