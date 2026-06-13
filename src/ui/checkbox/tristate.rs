//! 支持三态(checked / unchecked / indeterminate)的 yoface checkbox 变体。
//!
//! dioxus-primitives 的 `checkbox::Checkbox` 只表达 `checked` / `unchecked`
//! 两态(HTML `checked` 属性无法表达 indeterminate),故这里**自实现**一个
//! 原生 `<input type=checkbox>` 变体,不破坏既有 [`super::Checkbox`]。
//!
//! `indeterminate` 不是 HTML 属性,只能在渲染后通过 DOM property 设置——
//! 这里用 `dioxus::document::eval`(yoface/yougen 既有约定,无需引入
//! `web-sys` / `wasm-bindgen`,host 下为安全 no-op)在每次渲染后把
//! `el.indeterminate` 同步到 prop。
//!
//! 纯叶子:`onchange(bool)` 回调的值是 DOM 翻转后的**新意图**;
//! indeterminate→点击 的语义(通常是「全选当前页」)由父组件决定。

use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[css_module("/src/ui/checkbox/style.css")]
struct Styles;

static TRISTATE_SEQ: AtomicU64 = AtomicU64::new(0);

/// 三态 checkbox。`indeterminate` 为真时渲染「部分选中」横杠态(用于表头
/// 全选单元格)。`id` 留空则自动生成稳定唯一 id 供 `indeterminate` 反射。
#[component]
pub fn TristateCheckbox(
    #[props(default)] id: String,
    #[props(default)] class: String,
    #[props(default = "Select".to_string())] aria_label: String,
    #[props(default)] checked: bool,
    #[props(default)] indeterminate: bool,
    #[props(default)] disabled: bool,
    #[props(default)] onchange: EventHandler<bool>,
) -> Element {
    // 每个实例一个稳定 id(渲染间不变),用于 eval 定位元素反射 indeterminate。
    let resolved_id = use_hook(|| {
        if id.is_empty() {
            format!(
                "dx-tristate-cb-{}",
                TRISTATE_SEQ.fetch_add(1, Ordering::Relaxed)
            )
        } else {
            id.clone()
        }
    });

    // 每次渲染后把 indeterminate 反射到 DOM property(HTML 属性表达不了)。
    {
        let id_for_eval = resolved_id.clone();
        use_effect(move || {
            let script = format!(
                "var el=document.getElementById('{id}');if(el){{el.indeterminate={flag};}}",
                id = id_for_eval,
                flag = if indeterminate { "true" } else { "false" },
            );
            let _ = document::eval(&script);
        });
    }

    let cls = if class.is_empty() {
        Styles::dx_checkbox_native.to_string()
    } else {
        format!("{} {}", Styles::dx_checkbox_native, class)
    };

    rsx! {
        input {
            id: resolved_id,
            r#type: "checkbox",
            class: cls,
            "aria-label": aria_label,
            checked,
            disabled,
            onchange: move |evt| {
                // 浏览器在 fire change 前已翻转 checked;从事件读新状态而非
                // 取反过期 prop(上一帧若是 indeterminate,取反是错的)。
                let v = evt.value();
                let new_state = v == "true" || v == "on";
                onchange.call(new_state);
            },
        }
    }
}

/// 纯函数:给定当前页已选行数与本页可选总行数,推导表头「全选」单元格的
/// `(checked, indeterminate)`。空页渲染为未选 / 非中间态,避免表头「说谎」。
pub fn header_state(selected_on_page: usize, page_size: usize) -> (bool, bool) {
    if page_size == 0 || selected_on_page == 0 {
        (false, false)
    } else if selected_on_page >= page_size {
        (true, false)
    } else {
        (false, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_state_empty_page() {
        assert_eq!(header_state(0, 0), (false, false));
        assert_eq!(header_state(0, 10), (false, false));
    }

    #[test]
    fn header_state_partial() {
        assert_eq!(header_state(3, 10), (false, true));
        assert_eq!(header_state(1, 2), (false, true));
    }

    #[test]
    fn header_state_full() {
        assert_eq!(header_state(10, 10), (true, false));
        // 越界计数(防御性——不该发生,但行为稳定)
        assert_eq!(header_state(11, 10), (true, false));
    }
}
