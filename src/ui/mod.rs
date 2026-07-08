//! yoface 自定义控件集合。
//!
//! 全部以 `#[css_module]` 封装,颜色走 `tokens.css` 的设计令牌。
//! 官方 dioxus-primitives 已提供的原语请直接用 `yoface::dioxus_primitives`,
//! 这里只放在其之上的 Cokret 封装,以及官方没有的后台实用控件。

// --- 自 inkson src/ui 迁入:dioxus-primitives 之上的 css_module 封装 ---
pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod dialog;
pub mod input;
pub mod label;
pub mod select;
pub mod separator;
pub mod slider;
pub mod switch;
pub mod tabs;
pub mod textarea;

// --- 自 sodmin 收纳:官方原语没有的后台实用控件(已改写为 css_module) ---
pub mod empty_state;
pub mod error_banner;
pub mod info_row;
pub mod loading;
pub mod modal;
pub mod page_header;
pub mod pagination;
pub mod table;
pub mod toast;
