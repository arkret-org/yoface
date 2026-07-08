//! yoface shared pure-function utilities.
//!
//! Houses presentation-layer logic that was previously duplicated across
//! the frontends (inkson / sodmin), consolidated here to remove the copies.
//! `format` / `text` hold zero-dependency, runtime-free pure functions;
//! `dom` holds the shared `dioxus::document::eval` JS-interop helpers
//! (clipboard / new-tab). Formatting coupled to chrono / a `Locale` type
//! stays local to each app.

pub mod dom;
pub mod format;
pub mod text;
