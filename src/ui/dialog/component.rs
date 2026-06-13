use dioxus::prelude::*;
use dioxus_primitives::dialog::{self, DialogDescriptionProps, DialogRootProps, DialogTitleProps};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/ui/dialog/style.css")]
struct Styles;

#[component]
pub fn Dialog(props: DialogRootProps) -> Element {
    let mut suppress_backdrop_click = use_signal(|| false);
    let base = attributes!(div {
        class: Styles::dx_dialog,
        role: "dialog",
        "aria-modal": "true",
    });
    let merged = merge_attributes(vec![base, props.attributes]);
    let open = (props.open)().unwrap_or(props.default_open);

    if !open {
        return rsx! {};
    }

    rsx! {
        div {
            class: Styles::dx_dialog_backdrop,
            id: props.id,
            "data-state": "open",
            onclick: move |_| {
                if suppress_backdrop_click() {
                    suppress_backdrop_click.set(false);
                    return;
                }
                props.on_open_change.call(false);
            },
            onmouseup: move |_| {
                // 拖拽出界后在 backdrop 上松手:清除抑制标志,使后续真正的
                // backdrop 点击能正常关闭对话框。(yougen 原实现在此处用
                // `crate::api::sleep_for` 延时 32ms 清除以躲过同帧的
                // click 事件;yoface 去除该业务耦合,改为即时清除——
                // mousedown/click 上的 stop_propagation 已足够防误关。)
                if suppress_backdrop_click() {
                    suppress_backdrop_click.set(false);
                }
            },
            div {
                onmousedown: move |event: dioxus::events::MouseEvent| {
                    suppress_backdrop_click.set(true);
                    event.stop_propagation();
                },
                onmouseup: move |_| {
                    suppress_backdrop_click.set(false);
                },
                onclick: move |event: dioxus::events::MouseEvent| {
                    suppress_backdrop_click.set(false);
                    event.stop_propagation();
                },
                ..merged,
                {props.children}
            }
        }
    }
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let base = attributes!(h2 {
        class: Styles::dx_dialog_title,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogTitle {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn DialogDescription(props: DialogDescriptionProps) -> Element {
    let base = attributes!(p {
        class: Styles::dx_dialog_description,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        dialog::DialogDescription {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}
