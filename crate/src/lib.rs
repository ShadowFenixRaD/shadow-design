//! # Shadow Design
//!
//! A modular, high-performance UI component library for **Dioxus 0.7** applications (Web, Desktop, and Mobile).
//! Built with Mantine-inspired themes, tactile craft surfaces, dark aesthetics, and zero external runtime dependencies.
//!
//! ## 🎨 Zero-Tooling Style Integration (`STYLES`)
//!
//! Shadow Design uses a **100% Pure SCSS** architecture bundled directly inside the crate.
//! You do **not** need Node.js, npm, Tailwind CLI, or external bundlers/watchers in your application.
//!
//! The compiled stylesheet is exposed as a Dioxus [`Asset`] via [`STYLES`]. Dioxus's native asset pipeline
//! automatically processes and bundles the SCSS stylesheet at compile time.
//!
//! ### Quick Setup
//!
//! In your root application component, link [`STYLES`] directly using `document::Link`:
//!
//! ```rust,no_run
//! use dioxus::prelude::*;
//! use shadow_design::prelude::*;
//!
//! fn main() {
//!     dioxus::launch(App);
//! }
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! {
//!         // 1. Link the master stylesheet directly from the crate
//!         document::Link { rel: "stylesheet", href: STYLES }
//!
//!         // 2. Wrap with NotificationProvider if using toast notifications
//!         NotificationProvider {
//!             MainView {}
//!         }
//!     }
//! }
//!
//! #[component]
//! fn MainView() -> Element {
//!     rsx! {
//!         Card {
//!             title: "Welcome to Shadow Design".to_string(),
//!             shadow: CardShadow::Md,
//!             p { "High-performance Dioxus UI components with pure SCSS styling." }
//!             Button {
//!                 variant: ButtonVariant::Primary,
//!                 onclick: move |_| println!("Clicked!"),
//!                 "Get Started"
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Alternatively, you can alias or re-export the stylesheet constant:
//!
//! ```rust,no_run
//! use dioxus::prelude::*;
//!
//! pub const STYLES: Asset = shadow_design::STYLES;
//! ```
//!
//! ### What [`STYLES`] Includes
//!
//! - **CSS Preflight Reset**: Normalizes browser default styles (`button`, `input`, `select`, etc.) to eliminate native browser artifacts.
//! - **Elevation Layers & Z-Index Stack**: Mathematically sound z-index scale (`_zindex.scss`) preventing dropdown, modal, drawer, or tooltip clipping.
//! - **Built-in Theme Presets**: Design tokens for 5 core presets:
//!   - `Ultraviolet Shadow` (Default): Deep black (`#000000`) with purple accents and specular highlights.
//!   - `Obsidian Titanium`: Strict monochrome silver and titanium finish.
//!   - `Crimson Phoenix`: Deep black with vibrant phoenix crimson glow.
//!   - `Cyber Cyan`: High-contrast electric cyan aesthetic.
//!   - `Emerald Matrix`: Crisp emerald green telemetry style.
//! - **Micro-Interactions**: Tactile button press feedback, smooth hover states, and ambient glow matrix.
//! - **Keyframe Animations**: Smooth slide-in toast notifications, dialog zoom scaling, accordion unrolls, and indeterminate loaders.
//!
//! ## 🧩 Feature Flags
//!
//! Shadow Design is built with a zero-bloat architecture. Each component is isolated behind its own Cargo feature:
//!
//! | Feature Flag | Module | Description |
//! | :--- | :--- | :--- |
//! | `all` | *All modules* | Enables the complete suite of components and theming |
//! | `button` | [`button`] | Buttons with BEM variants, sizes, loading spinners, and tactile active states |
//! | `card` | [`card`] | Container cards with elevated, interactive, and metric variants |
//! | `input` | [`input`] | Reactive text inputs with clickable icons, clear buttons, and signal binding |
<<<<<<< HEAD
//! | `select` | [`select`] | Advanced dropdown Select, MultiSelect with tags, and Combobox autocomplete |
=======
//! | `textarea` | [`textarea`] | Multiline text areas with dynamic auto-resize, character count, and clear button |
>>>>>>> origin/component/textarea
//! | `checkbox` | [`checkbox`] | Tactile checkboxes with indeterminate and error states |
//! | `badge` | [`badge`] | Status labels, pill badges, and pulsating live dots |
//! | `navbar` | [`navbar`] | Responsive navigation bars with brand and action slots |
//! | `modal` | [`modal`] | Overlay dialog modals with animated backdrop and signal control |
//! | `alert` | [`alert`] | Callout banners with semantic variants and dismiss handlers |
//! | `theme` | [`theme`] | Mantine-style reactive theme engine (`ThemeProvider`, `use_theme`) |
//! | `icon` | [`icon`] | Zero-dependency native SVG icons (1.5px stroke) |
//! | `typography` | [`typography`] | Typography primitives (`Heading`, `Text`, `Kbd`, `Code`) |
//! | `notification` | [`notification`] | Global toast notification system (`NotificationProvider`, `use_notification`) |
//! | `tabs` | [`tabs`] | Horizontal tab navigation (`TabList`, `Tab`, `TabPanel`) |
//! | `switch` | [`switch`] | Precision tactile toggle switches |
//! | `tooltip` | [`tooltip`] | Floating hover tooltips with 4-directional placement |
//! | `drawer` | [`drawer`] | Slide-over side panels from left or right with backdrop |
//! | `dropdown` | [`dropdown`] | Trigger-anchored context menus with auto-dismiss |
//! | `animation` | [`animation`] | Optical transitions and micro-animation wrappers |
//!
//! ## 🎨 Reactive Theming vs. Zero-Cost Static Fallback
//!
//! Shadow Design components support dual execution paths:
//!
//! 1. **With `theme` feature enabled**: Components reactively read the current theme from Dioxus Context via [`theme::use_theme()`]. When you switch palettes, radius scale, or glow mode at runtime, every mounted component updates immediately.
//! 2. **Without `theme` feature enabled**: Components fall back transparently to optimized, hardcoded static classes, incurring zero context lookup or runtime overhead.
//!
//! Or enable the `all` feature to include the entire suite.

use dioxus::prelude::*;

/// Master SCSS stylesheet asset provided by the Shadow Design library.
///
/// This asset points to `/scss/main.scss` packaged inside the crate and is processed
/// natively by Dioxus 0.7's [`asset!`] macro. It contains the complete styling engine:
/// preflight reset, typography, theme color variables, surface elevation layers,
/// component classes (`.sd-btn`, `.sd-card`, `.sd-input`, etc.), and keyframe animations.
///
/// ### Usage in Dioxus
///
/// Include it directly in your application's root component head:
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use shadow_design::prelude::*;
///
/// #[component]
/// fn App() -> Element {
///     rsx! {
///         // Embed the compiled stylesheet
///         document::Link { rel: "stylesheet", href: STYLES }
///
///         // Your UI components
///         Button { "Click me" }
///     }
/// }
/// ```
///
/// Downstream projects do not need Node.js, npm, or Tailwind CLI; Dioxus bundles
/// the SCSS stylesheet automatically.
pub const STYLES: Asset = asset!("/scss/main.scss");

#[cfg(feature = "alert")]
pub mod alert;
#[cfg(feature = "animation")]
pub mod animation;
#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "checkbox")]
pub mod checkbox;
#[cfg(feature = "drawer")]
pub mod drawer;
#[cfg(feature = "dropdown")]
pub mod dropdown;
#[cfg(feature = "icon")]
pub mod icon;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "textarea")]
pub mod textarea;
#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "navbar")]
pub mod navbar;
#[cfg(feature = "notification")]
pub mod notification;
#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "theme")]
pub mod theme;
#[cfg(feature = "tooltip")]
pub mod tooltip;
#[cfg(feature = "typography")]
pub mod typography;

pub mod prelude {
    #[cfg(feature = "alert")]
    pub use crate::alert::*;
    #[cfg(feature = "animation")]
    pub use crate::animation::*;
    #[cfg(feature = "badge")]
    pub use crate::badge::*;
    #[cfg(feature = "button")]
    pub use crate::button::*;
    #[cfg(feature = "card")]
    pub use crate::card::*;
    #[cfg(feature = "checkbox")]
    pub use crate::checkbox::*;
    #[cfg(feature = "drawer")]
    pub use crate::drawer::*;
    #[cfg(feature = "dropdown")]
    pub use crate::dropdown::*;
    #[cfg(feature = "icon")]
    pub use crate::icon::*;
    #[cfg(feature = "input")]
    pub use crate::input::*;
    #[cfg(feature = "select")]
    pub use crate::select::*;
    #[cfg(feature = "textarea")]
    pub use crate::textarea::*;
    #[cfg(feature = "modal")]
    pub use crate::modal::*;
    #[cfg(feature = "navbar")]
    pub use crate::navbar::*;
    #[cfg(feature = "notification")]
    pub use crate::notification::*;
    #[cfg(feature = "switch")]
    pub use crate::switch::*;
    #[cfg(feature = "tabs")]
    pub use crate::tabs::*;
    #[cfg(feature = "theme")]
    pub use crate::theme::*;
    #[cfg(feature = "tooltip")]
    pub use crate::tooltip::*;
    #[cfg(feature = "typography")]
    pub use crate::typography::*;
    pub use crate::STYLES;
}
