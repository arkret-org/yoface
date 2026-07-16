use dioxus::prelude::*;

#[css_module("/src/ui/error_banner/style.css")]
struct Styles;

/// The inline error banner shared by list / detail pages.
///
/// Decoupling note: sodmin's original implementation was coupled to
/// `crate::utils::i18n::t` (localization), `crate::utils::net::telemetry`
/// (analytics) and the `HttpError` envelope type. yoface, being a shared
/// library, does not pull in those business dependencies and is instead a pure
/// presentational component:
///   * `message` is the human-readable summary;
///   * the `error_label` / `retry_label` copy is passed in by the caller
///     (English by default), with downstream localization done at the outer
///     layer;
///   * `detail` is an optional secondary explanation (if downstream has an
///     error-code-to-copy mapping, it resolves it outside and passes the
///     result in);
///   * `errcode` / `request_id` / `retry_after_ms` are displayed directly as
///     metadata chips;
///   * side effects such as analytics are left for downstream to handle at the
///     call site.
#[component]
pub fn ErrorBanner(
    message: String,
    #[props(default = "Error".to_string())] error_label: String,
    #[props(default = "Retry".to_string())] retry_label: String,
    #[props(default)] detail: Option<String>,
    #[props(default)] errcode: Option<String>,
    #[props(default)] request_id: Option<String>,
    #[props(default)] retry_after_ms: Option<u64>,
    #[props(default)] on_retry: Option<EventHandler<MouseEvent>>,
) -> Element {
    let has_meta = errcode.is_some() || request_id.is_some() || retry_after_ms.is_some();

    rsx! {
        div { class: Styles::dx_error_banner,
            div { class: Styles::dx_error_banner_head,
                p { class: Styles::dx_error_banner_message, "{error_label}: {message}" }
                if let Some(handler) = on_retry {
                    button {
                        class: Styles::dx_error_banner_retry,
                        onclick: move |evt| handler.call(evt),
                        "{retry_label}"
                    }
                }
            }
            if let Some(copy) = detail {
                p { class: Styles::dx_error_banner_detail, "{copy}" }
            }
            if has_meta {
                div { class: Styles::dx_error_banner_meta,
                    if let Some(code) = errcode {
                        span { class: Styles::dx_error_banner_chip, "code: {code}" }
                    }
                    if let Some(rid) = request_id {
                        span { class: Styles::dx_error_banner_chip, "ref: {rid}" }
                    }
                    if let Some(retry_after_ms) = retry_after_ms {
                        span { class: Styles::dx_error_banner_chip, "retry in: {retry_after_ms / 1000}s" }
                    }
                }
            }
        }
    }
}
