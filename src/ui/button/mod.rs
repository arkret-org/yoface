mod component;
pub use component::*;

/// Unhashed button styles for hosts that need a stable fallback alongside the
/// normal CSS-module asset. The component always emits [`BUTTON_CLASS`].
pub const BUTTON_CSS: &str = include_str!("style.css");
