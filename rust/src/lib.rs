//! # smooai-ui
//!
//! The Rust face of [`@smooai/ui`](https://github.com/SmooAI/ui) — SmooAI's
//! cross-language design system. Surfaces the shared design tokens, base CSS,
//! and the smoo monogram as `pub const &'static str` constants so the same
//! source-of-truth file is consumed identically by every SmooAI Rust desktop
//! app (smooblue, observability-studio, …).
//!
//! **No framework dependency.** This crate compiles in `no_std` and exposes
//! only constants, so adding it to your app doesn't pull in a UI-framework
//! version pin (Dioxus / egui / Tauri / Iced / …).
//!
//! ## Usage
//!
//! In a Dioxus app:
//!
//! ```ignore
//! use dioxus::prelude::*;
//!
//! fn App() -> Element {
//!     rsx! {
//!         style { "{smooai_ui::STYLES}" }
//!         // ... app body ...
//!     }
//! }
//! ```
//!
//! In an egui app (or any other framework), reference the token strings
//! directly:
//!
//! ```ignore
//! let accent = smooai_ui::tokens::SMOOAI_GREEN;
//! ```
//!
//! ## Version policy
//!
//! Token *values* may evolve at minor versions. Token *names* are stable
//! across minor versions and only change at major versions. Apps that pin
//! `smooai-ui = "0.1"` can safely take patch/minor updates without
//! re-auditing their CSS.

#![no_std]
#![doc(html_root_url = "https://docs.rs/smooai-ui/0.1.1")]
#![warn(missing_docs)]

/// Canonical brand stylesheet, sourced from
/// [`@smooai/ui` `shared/styles.css`](https://github.com/SmooAI/ui/blob/main/shared/styles.css).
/// Embed in your app's root component so every consumer sees the same tokens
/// + base component classes.
pub const STYLES: &str = include_str!("../../shared/styles.css");

/// The smoo monogram, as an SVG string with `fill="currentColor"` so the
/// surrounding CSS controls the color. Pair with the `.brand-badge` class
/// (defined in [`STYLES`]) for the gradient pill backdrop:
///
/// ```ignore
/// rsx! {
///     div {
///         class: "brand-badge",
///         style: "width:32px;height:32px;",
///         dangerous_inner_html: "{smooai_ui::MONOGRAM_SVG}",
///     }
/// }
/// ```
pub const MONOGRAM_SVG: &str = include_str!("../../shared/monogram.svg");

/// Brand + semantic token *values*, for code paths that need a colour,
/// radius, spacing step, or font stack outside of CSS (custom-painted egui
/// widgets, native menu chrome, chart libraries, etc.).
///
/// **Generated** at build time from
/// [`shared/tokens.json`](https://github.com/SmooAI/ui/blob/main/shared/tokens.json)
/// by `shared/tokens_codegen.rs` — nobody hand-writes these, so a token cannot
/// exist in the design system and be missing here. `shared/tokens_css_check.rs`
/// then asserts each one equals the resolved value of its custom property in
/// [`STYLES`], and that every colour the CSS declares has a token.
pub mod tokens {
    include!(concat!(env!("OUT_DIR"), "/tokens.rs"));

    /// The default corner radius, as used by cards and buttons. Retained as an
    /// alias of [`RADIUS_MD_PX`] so consumers pinned to the pre-codegen name
    /// keep compiling.
    pub const RADIUS_PX: u16 = RADIUS_MD_PX;
}

#[cfg(test)]
mod tests {
    // `#![no_std]` for the crate body lets consumers embed us in `no_std`
    // contexts (cdylib's, native menu bars, etc.). Tests opt back in.
    extern crate std;

    use super::*;

    #[test]
    fn styles_is_nonempty() {
        assert!(STYLES.len() > 100);
        assert!(STYLES.contains("--color-smooai-green"));
    }

    #[test]
    fn monogram_is_real_svg() {
        assert!(MONOGRAM_SVG.starts_with("<svg"));
        assert!(MONOGRAM_SVG.contains("viewBox=\"0 0 135 135\""));
        assert!(MONOGRAM_SVG.contains("fill=\"currentColor\""));
    }

    // The token <-> CSS cross-check (both directions) lives in `shared/`, so
    // SmooAI/ui and SmooAI/client-shared run the identical assertions.
    include!("../../shared/tokens_css_check.rs");

    #[test]
    fn semantic_classes_exist() {
        // Smoke check the public BEM classes consumers reach for. If anyone
        // renames `.btn--primary` they'll break consumers, so this test fails.
        for cls in [
            ".btn",
            ".btn--primary",
            ".btn--ghost",
            ".card",
            ".fab",
            ".modal__sheet",
            ".rail",
            ".rail__btn",
            ".brand-badge",
            ".input",
            ".input--lg",
            ".input-error",
            ".input-hint",
        ] {
            assert!(STYLES.contains(cls), "missing class {cls}");
        }
    }
}
