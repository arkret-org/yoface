//! yoface shared pure-function utilities.
//!
//! Houses presentation-layer pure logic that was previously duplicated across
//! the frontends (yougen / sodmin), consolidated here to remove the copies.
//! Only zero-dependency, runtime-free pure functions belong here; formatting
//! coupled to chrono / a `Locale` type stays local to each app.

pub mod format;
pub mod text;
