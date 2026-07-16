//! yoface's collection of custom controls.
//!
//! Every one of them is wrapped with `#[css_module]`, and colored through the
//! design tokens in `tokens.css`. For primitives the official
//! dioxus-primitives already provides, use `yoface::dioxus_primitives`
//! directly; this module only holds the Arkret wrappers built on top of them,
//! plus the admin-facing utility controls the official crate does not have.

// --- Moved in from inkson src/ui: css_module wrappers over dioxus-primitives ---
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

// --- Collected from sodmin: admin utility controls the official primitives
//     lack (rewritten as css_module) ---
pub mod empty_state;
pub mod error_banner;
pub mod info_row;
pub mod loading;
pub mod modal;
pub mod page_header;
pub mod pagination;
pub mod table;
pub mod toast;
