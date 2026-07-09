//! # yoface — Arkret 共享前端组件库
//!
//! 所有 Arkret 的 Dioxus 前端统一依赖本 crate。它做三件事:
//!
//! 1. **透传官方原语**:`pub use dioxus_primitives;` / `pub use dioxus_icons;`
//!    —— 下游通过 `yoface::dioxus_primitives::...` 拿到与 yoface 同源(同一
//!    components fork rev)的原语,杜绝 `GenerationalBox` 跨源 mismatch。
//! 2. **自定义控件**:`pub mod ui` 收纳一组 `#[css_module]` 封装控件
//!    (按钮 / 卡片 / 表格 / toast / 分页 …),全部以设计令牌着色。
//! 3. **设计令牌**:`tokens.css`(见 [`TOKENS_CSS`])集中所有颜色 / 半径 /
//!    阴影变量,组件样式只引用变量、不写死颜色,主题留给下游覆盖。
//!
//! ## 样式注入
//!
//! `#[css_module]` 的样式由 manganis 在编译期收集,随组件首次渲染注入。
//! `tokens.css` 不走 css_module(它定义的是全局 `:root` 变量),下游需要
//! 自行把 [`TOKENS_CSS`] 注入文档(`document::Style { {yoface::TOKENS_CSS} }`
//! 或 `include_str!` 内联),或提供等价的同名令牌覆盖。

pub use dioxus_icons;
pub use dioxus_primitives;

pub mod ui;
pub mod utils;

/// yoface 默认设计令牌(shadcn 命名)。下游可直接注入,或用同名令牌覆盖。
///
/// ```ignore
/// use dioxus::prelude::*;
/// rsx! { document::Style { {yoface::TOKENS_CSS} } }
/// ```
pub const TOKENS_CSS: &str = include_str!("tokens.css");
