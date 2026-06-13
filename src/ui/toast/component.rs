use dioxus::prelude::*;

#[css_module("/src/ui/toast/style.css")]
struct Styles;

#[derive(Debug, Clone, PartialEq)]
pub enum ToastVariant {
    Success,
    Error,
}

impl ToastVariant {
    fn class(&self) -> &'static str {
        match self {
            ToastVariant::Success => "success",
            ToastVariant::Error => "error",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastAction {
    pub label: String,
    pub href: String,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub id: u64,
    pub message: String,
    pub variant: ToastVariant,
    pub action: Option<ToastAction>,
}

static TOAST_COUNTER: GlobalSignal<u64> = GlobalSignal::new(|| 0);
pub static TOASTS: GlobalSignal<Vec<Toast>> = GlobalSignal::new(Vec::new);

const MAX_TOASTS: usize = 5;

/// 弹出一条 toast。到期后自动消失(成功 3s / 错误 5s;含动作时至少 5s)。
pub fn show_toast(message: &str, variant: ToastVariant) {
    show_toast_with_action(message, variant, None);
}

/// 弹出一条带可点击动作链接的 toast。
pub fn show_toast_with_action(message: &str, variant: ToastVariant, action: Option<ToastAction>) {
    let id = {
        let mut counter = TOAST_COUNTER.write();
        *counter += 1;
        *counter
    };

    let timeout_ms = match variant {
        ToastVariant::Error => 5000u32,
        _ => 3000,
    };
    let timeout_ms = if action.is_some() {
        timeout_ms.max(5000)
    } else {
        timeout_ms
    };

    {
        let mut toasts = TOASTS.write();
        toasts.push(Toast {
            id,
            message: message.to_string(),
            variant,
            action,
        });
        if toasts.len() > MAX_TOASTS {
            let drain_count = toasts.len() - MAX_TOASTS;
            toasts.drain(0..drain_count);
        }
    }

    spawn(async move {
        sleep_ms(timeout_ms).await;
        dismiss_toast(id);
    });
}

fn dismiss_toast(id: u64) {
    let mut toasts = TOASTS.write();
    toasts.retain(|t| t.id != id);
}

/// 平台无关的延时。wasm 走 `setTimeout`(gloo-timers),host 走 tokio。
async fn sleep_ms(ms: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(std::time::Duration::from_millis(ms as u64)).await;
    }
}

/// 全局 toast 容器。挂一次到应用根即可,内容由 [`show_toast`] 驱动。
#[component]
pub fn Toaster() -> Element {
    let toasts = TOASTS.read();

    if toasts.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: Styles::dx_toaster,
            for toast in toasts.iter() {
                {
                    let toast_id = toast.id;
                    let action = toast.action.clone();
                    rsx! {
                        div {
                            key: "{toast_id}",
                            class: Styles::dx_toast,
                            "data-variant": toast.variant.class(),
                            p { class: Styles::dx_toast_message, "{toast.message}" }
                            if let Some(action) = action {
                                a {
                                    href: "{action.href}",
                                    class: Styles::dx_toast_action,
                                    "{action.label}"
                                }
                            }
                            button {
                                class: Styles::dx_toast_close,
                                onclick: move |_| dismiss_toast(toast_id),
                                "\u{00D7}"
                            }
                        }
                    }
                }
            }
        }
    }
}
