use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};

#[css_module("/src/ui/modal/style.css")]
struct Styles;

/// A centered full-screen overlay + content container; the ergonomic wrapper
/// for the many hand-written `fixed inset-0 z-50 …` dialogs across the admin's
/// list/detail pages.
///
/// Renders nothing when `open` is `false`. Clicking the overlay fires
/// `on_close`; when `title` is non-empty it renders the standard `h2` header,
/// with form fields and the [`DialogActions`] row passed in as `children`.
///
/// Complements yoface's existing [`crate::ui::dialog::Dialog`] (the
/// signal-driven dioxus-primitives wrapper): `Modal` takes the concise
/// `open: bool` + callback API and does not touch the primitives'
/// `Signal<Option<bool>>` internally, which suits admin call sites built on
/// "a boolean state + a close callback".
#[component]
pub fn Modal(
    open: bool,
    #[props(default)] title: String,
    /// Extra class passed through to the content container (for example, a
    /// custom max width).
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

/// The bare centered overlay (backdrop + flex container), without the standard
/// dialog panel. Use it when a page needs a custom panel shell but still wants
/// to reuse the shared overlay + click-to-close behavior.
///
/// Clicking the overlay backdrop fires `on_close`; the content area stops
/// propagation so that clicking inside does not close it by accident.
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

/// The confirm/cancel button row at the foot of a dialog (right-aligned),
/// reusing yoface's [`Button`].
///
/// `confirm_loading` disables the confirm button while a mutation is in
/// flight; `variant` decides the confirm button's style (defaults to
/// [`ButtonVariant::Primary`]; pass [`ButtonVariant::Destructive`] for
/// destructive operations).
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

/// The confirmation dialog: title + optional description + confirm/cancel
/// button row. Driven by `open: bool` + callbacks.
///
/// `variant` decides the confirm button's style (defaults to
/// [`ButtonVariant::Primary`]; pass [`ButtonVariant::Destructive`] for
/// destructive operations). Clicking the overlay is equivalent to cancelling
/// (it fires `on_cancel`).
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
