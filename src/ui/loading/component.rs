use dioxus::prelude::*;

#[css_module("/src/ui/loading/style.css")]
struct Styles;

/// The general-purpose skeleton: a title bar + three lines of text. `class` is
/// optional and passes through to the outer container.
#[component]
pub fn LoadingSkeleton(#[props(default)] class: String) -> Element {
    let block_class = if class.is_empty() {
        Styles::dx_skeleton_block.to_string()
    } else {
        format!("{} {}", Styles::dx_skeleton_block, class)
    };
    rsx! {
        div { class: block_class,
            div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_title) }
            div { class: Styles::dx_skeleton_lines,
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_full) }
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_lg) }
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_md) }
            }
        }
    }
}

/* The stat card skeleton grid. */
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

/// The full-page skeleton: a title area + the stat card grid + one content
/// card. Used as the placeholder while a page loads for the first time.
/// `card_count` controls the number of stat cards.
#[component]
pub fn PageSkeleton(#[props(default = 3)] card_count: u32) -> Element {
    rsx! {
        div { class: Styles::dx_page_skeleton,
            div { class: Styles::dx_skeleton_lines,
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_title) }
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_full) }
            }
            div { class: Styles::dx_skeleton_grid,
                for _ in 0..card_count {
                    div { class: Styles::dx_skeleton_card,
                        div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_sm) }
                        div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_stat) }
                        div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_lg) }
                    }
                }
            }
            div { class: Styles::dx_skeleton_card,
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_sm) }
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_full) }
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_full) }
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_lg) }
                div { class: format!("{} {}", Styles::dx_skeleton, Styles::dx_skeleton_line_md) }
            }
        }
    }
}

/// A single circular spinning loading indicator, colored with `currentColor`.
/// `class` is optional and passes through to the `<svg>` (for example, to
/// adjust the size/color of a spinner inlined in a button).
#[component]
pub fn Spinner(#[props(default)] class: String) -> Element {
    let svg_class = if class.is_empty() {
        Styles::dx_spinner.to_string()
    } else {
        format!("{} {}", Styles::dx_spinner, class)
    };
    rsx! {
        svg {
            class: svg_class,
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
