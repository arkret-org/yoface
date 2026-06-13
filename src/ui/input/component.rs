use dioxus::prelude::*;
use dioxus_icons::lucide::Search;
#[css_module("/src/ui/input/style.css")]
struct Styles;

#[component]
pub fn Input(
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    oninvalid: Option<EventHandler<FormEvent>>,
    onselect: Option<EventHandler<SelectionEvent>>,
    onselectionchange: Option<EventHandler<SelectionEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onfocusin: Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    onwheel: Option<EventHandler<WheelEvent>>,
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    oncompositionend: Option<EventHandler<CompositionEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        input {
            class: Styles::dx_input,
            oninput: move |e| _ = oninput.map(|callback| callback(e)),
            onchange: move |e| _ = onchange.map(|callback| callback(e)),
            oninvalid: move |e| _ = oninvalid.map(|callback| callback(e)),
            onselect: move |e| _ = onselect.map(|callback| callback(e)),
            onselectionchange: move |e| _ = onselectionchange.map(|callback| callback(e)),
            onfocus: move |e| _ = onfocus.map(|callback| callback(e)),
            onblur: move |e| _ = onblur.map(|callback| callback(e)),
            onfocusin: move |e| _ = onfocusin.map(|callback| callback(e)),
            onfocusout: move |e| _ = onfocusout.map(|callback| callback(e)),
            onkeydown: move |e| _ = onkeydown.map(|callback| callback(e)),
            onkeypress: move |e| _ = onkeypress.map(|callback| callback(e)),
            onkeyup: move |e| _ = onkeyup.map(|callback| callback(e)),
            onwheel: move |e| _ = onwheel.map(|callback| callback(e)),
            oncompositionstart: move |e| _ = oncompositionstart.map(|callback| callback(e)),
            oncompositionupdate: move |e| _ = oncompositionupdate.map(|callback| callback(e)),
            oncompositionend: move |e| _ = oncompositionend.map(|callback| callback(e)),
            oncopy: move |e| _ = oncopy.map(|callback| callback(e)),
            oncut: move |e| _ = oncut.map(|callback| callback(e)),
            onpaste: move |e| _ = onpaste.map(|callback| callback(e)),
            ..attributes,
            {children}
        }
    }
}

/// 带搜索图标的搜索输入框。图标用 `dioxus_icons::lucide::Search`,定位在
/// 输入框左侧,文本左内边距留出图标空间。`value` 受控,`oninput` 上抛
/// 输入事件。`placeholder` / `aria_label` 可选。
#[component]
pub fn SearchInput(
    #[props(default)] placeholder: String,
    #[props(default = "Search".to_string())] aria_label: String,
    #[props(default)] value: String,
    #[props(default)] class: String,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let label = if aria_label.is_empty() {
        "Search".to_string()
    } else {
        aria_label
    };
    let ph = if placeholder.is_empty() {
        "Search...".to_string()
    } else {
        placeholder
    };
    let wrapper_class = if class.is_empty() {
        Styles::dx_search.to_string()
    } else {
        format!("{} {}", Styles::dx_search, class)
    };
    rsx! {
        div { class: wrapper_class,
            span { class: Styles::dx_search_icon,
                Search { size: "1rem" }
            }
            input {
                r#type: "search",
                class: format!("{} {}", Styles::dx_input, Styles::dx_search_input),
                placeholder: ph,
                value,
                "aria-label": label,
                oninput: move |evt| oninput.call(evt),
            }
        }
    }
}
