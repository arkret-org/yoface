use dioxus::prelude::*;

#[css_module("/src/ui/error_banner/style.css")]
struct Styles;

/// 列表 / 详情页通用的内联错误横幅。
///
/// 解耦说明:sodmin 原实现耦合 `crate::utils::i18n::t`(本地化)、
/// `crate::utils::net::telemetry`(埋点)与 `HttpError` 信封类型。yoface
/// 作为共享库不引入这些业务依赖,改为纯展示组件:
///   * `message` 为人类可读摘要;
///   * `error_label` / `retry_label` 文案由调用方传入(默认英文),
///     下游本地化在外层完成;
///   * `detail` 是可选的二级说明(下游若有错误码→文案映射,在外层解析后传入);
///   * `errcode` / `request_id` / `retry_after_ms` 直接展示为元信息 chip;
///   * 埋点等副作用留给下游在调用处自行处理。
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
