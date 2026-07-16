//! # yoface — Arkret shared frontend component library
//!
//! Every Arkret Dioxus frontend depends on this crate. It does three things:
//!
//! 1. **Re-export the official primitives**: `pub use dioxus_primitives;` /
//!    `pub use dioxus_icons;` —— downstream gets, via
//!    `yoface::dioxus_primitives::...`, primitives from the same source as
//!    yoface (the same components fork rev), ruling out cross-source
//!    `GenerationalBox` mismatches.
//! 2. **Custom controls**: `pub mod ui` collects a set of `#[css_module]`
//!    wrapper controls (button / card / table / toast / pagination …), all
//!    colored through design tokens.
//! 3. **Design tokens**: `tokens.css` (see [`TOKENS_CSS`]) centralizes every
//!    color / radius / shadow variable; component styles only reference the
//!    variables and never hardcode colors, leaving theming to downstream
//!    overrides.
//!
//! ## Style injection
//!
//! `#[css_module]` styles are collected by manganis at compile time and
//! injected when the component first renders. `tokens.css` does not go through
//! css_module (it defines global `:root` variables), so downstream must inject
//! [`TOKENS_CSS`] into the document itself
//! (`document::Style { {yoface::TOKENS_CSS} }`, or inline it via
//! `include_str!`), or provide an equivalent set of same-named token
//! overrides.

pub use dioxus_icons;
pub use dioxus_primitives;

pub mod ui;
pub mod utils;

/// yoface's default design tokens (shadcn naming). Downstream can inject them
/// as-is, or override them with same-named tokens.
///
/// ```ignore
/// use dioxus::prelude::*;
/// rsx! { document::Style { {yoface::TOKENS_CSS} } }
/// ```
pub const TOKENS_CSS: &str = include_str!("tokens.css");
