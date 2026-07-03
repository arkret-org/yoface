//! Browser/webview DOM interop helpers shared by the frontends.
//!
//! Consolidated here (YGN-DRY-04) from five yougen copies of the clipboard
//! helper and two copies of the new-tab opener. Both are security-sensitive
//! boundaries (secure-context gating for the Clipboard API,
//! `noopener,noreferrer` on `window.open`), so a single definition keeps any
//! future hardening patch atomic. Uses `dioxus::document::eval` (the existing
//! yoface/yougen convention for JS interop) and is a no-op when serialization
//! fails.

use dioxus::document;

/// Copy `text` to the clipboard using the async Clipboard API in secure
/// contexts, with a `document.execCommand("copy")` off-screen-textarea
/// fallback elsewhere. Fire-and-forget: failures are swallowed (the caller's
/// UI feedback should not depend on the copy result being observable).
pub fn copy_text_to_clipboard(text: &str) {
    let Ok(encoded) = serde_json::to_string(text) else {
        return;
    };
    let script = format!(
        r#"(async () => {{
    const text = {encoded};
    if (navigator.clipboard && window.isSecureContext) {{
        await navigator.clipboard.writeText(text);
        return true;
    }}
    const node = document.createElement("textarea");
    node.value = text;
    node.setAttribute("readonly", "");
    node.style.position = "fixed";
    node.style.left = "-9999px";
    document.body.appendChild(node);
    node.select();
    const copied = document.execCommand("copy");
    document.body.removeChild(node);
    return copied;
}})()"#
    );
    let _ = document::eval(&script);
}

/// Open `url` in a new tab with `noopener,noreferrer` so the opened page can
/// neither script back into the opener nor learn the referrer.
pub fn open_url_in_new_tab(url: &str) {
    let Ok(encoded) = serde_json::to_string(url) else {
        return;
    };
    let script = format!("window.open({encoded}, \"_blank\", \"noopener,noreferrer\");");
    let _ = document::eval(&script);
}
