#![cfg_attr(docsrs, feature(doc_auto_cfg))]

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
//! ```rust,ignore
//! # #[cfg(all(feature = "button", feature = "card", feature = "notification"))]
//! # fn doc_example() {
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
//! # }
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
//! | `button` | `button` | Buttons with BEM variants, sizes, loading spinners, and tactile active states |
//! | `card` | `card` | Container cards with elevated, interactive, and metric variants |
//! | `input` | `input` | Reactive text inputs with clickable icons, clear buttons, and signal binding |
//! | `select` | `select` | Advanced dropdown Select, MultiSelect with tags, and Combobox autocomplete |
//! | `textarea` | `textarea` | Multiline text areas with dynamic auto-resize, character count, and clear button |
//! | `checkbox` | `checkbox` | Tactile checkboxes with indeterminate and error states |
//! | `badge` | `badge` | Status labels, pill badges, and pulsating live dots |
//! | `navbar` | `navbar` | Responsive navigation bars with brand and action slots |
//! | `modal` | `modal` | Overlay dialog modals with animated backdrop and signal control |
//! | `alert` | `alert` | Callout banners with semantic variants and dismiss handlers |
//! | `theme` | `theme` | Mantine-style reactive theme engine (`ThemeProvider`, `use_theme`) |
//! | `icon` | `icon` | Zero-dependency native SVG icons (1.5px stroke) |
//! | `typography` | `typography` | Typography primitives (`Heading`, `Text`, `Kbd`, `Code`) |
//! | `notification` | `notification` | Global toast notification system (`NotificationProvider`, `use_notification`) |
//! | `tabs` | `tabs` | Horizontal tab navigation (`TabList`, `Tab`, `TabPanel`) |
//! | `switch` | `switch` | Precision tactile toggle switches |
//! | `tooltip` | `tooltip` | Floating hover tooltips with 4-directional placement |
//! | `drawer` | `drawer` | Slide-over side panels from left or right with backdrop |
//! | `dropdown` | `dropdown` | Trigger-anchored context menus with auto-dismiss |
//! | `animation` | `animation` | Optical transitions and micro-animation wrappers |
//! | `avatar` | `avatar` | User avatars, presence status dot, initials fallback, and avatar groups |
//! | `chat` | `chat` | P2P & messaging chat bubbles with delivery status, security badges, and rich media |
//! | `progress` | `progress` | Progress bars with semantic variants and shimmer indeterminate animations |
//! | `attachment_card` | `attachment_card` | Media attachments and file transfer cards with action slots |
//! | `contact_card` | `contact_card` | Contact profile cards with presence indicators and quick action buttons |
//! | `segmented_control` | `segmented_control` | Tactile sliding segmented selectors with theme glow |
//! | `divider` | `divider` | Horizontal and vertical translucent separators with label support |
//! | `tree` | `tree` | Hierarchical tree views with collapsible nodes and connection lines |
//! | `skeleton` | `skeleton` | Geometric loading placeholder shimmers (circle, rect, text) |
//! | `number_input` | `number_input` | Numeric inputs with stepper controls, decimal precision, prefix/suffix |
//! | `radio` | `radio` | Radio, RadioGroup, and RadioCard with horizontal/vertical orientations |
//! | `slider` | `slider` | Graduated sliders and dual-thumb RangeSlider with marks and tooltips |
//! | `rating` | `rating` | Interactive star/icon ratings with half-fractions and hover preview |
//! | `pin_input` | `pin_input` | Multi-digit PIN/OTP code inputs with auto-advance and masking |
//! | `color_picker` | `color_picker` | ColorPicker and ColorInput with HSV/Hex/RGB formats, alpha, and swatches |
//! | `chip` | `chip` | Compact interactive filter and selection chips with single and multi-select groups |
//! | `accordion` | `accordion` | Collapsible accordion panels with single or multiple expansion |
//! | `spoiler` | `spoiler` | Expandable content container with max-height truncation and fade overlay |
//! | `scroll_area` | `scroll_area` | Scrollable container with micro-scrollbars, fade edges and hover types |
//! | `breadcrumbs` | `breadcrumbs` | Hierarchical wayfinding trail with customizable separators and ellipsis |
//! | `pagination` | `pagination` | Tactile numbered pagination controls with sibling & boundary ranges |
//!
//! ## 🎨 Reactive Theming vs. Zero-Cost Static Fallback
//!
//! Shadow Design components support dual execution paths:
//!
//! 1. **With `theme` feature enabled**: Components reactively read the current theme from Dioxus Context via `theme::use_theme()`. When you switch palettes, radius scale, or glow mode at runtime, every mounted component updates immediately.
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
/// # #[cfg(feature = "button")]
/// # fn doc_example() {
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
/// # }
/// ```
///
/// Downstream projects do not need Node.js, npm, or Tailwind CLI; Dioxus bundles
/// the SCSS stylesheet automatically.
pub const STYLES: Asset = asset!("/scss/main.scss");

pub mod a11y;
pub(crate) mod glow;
pub mod locale;
pub(crate) mod util;

#[cfg(feature = "accordion")]
pub mod accordion;
#[cfg(feature = "alert")]
pub mod alert;
#[cfg(feature = "animation")]
pub mod animation;
#[cfg(feature = "attachment_card")]
pub mod attachment_card;
#[cfg(feature = "avatar")]
pub mod avatar;
#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "chat")]
pub mod chat;
#[cfg(feature = "checkbox")]
pub mod checkbox;
#[cfg(feature = "contact_card")]
pub mod contact_card;
#[cfg(feature = "divider")]
pub mod divider;
#[cfg(feature = "drawer")]
pub mod drawer;
#[cfg(feature = "dropdown")]
pub mod dropdown;
#[cfg(feature = "icon")]
pub mod icon;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "list")]
pub mod list;
#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "navbar")]
pub mod navbar;
#[cfg(feature = "notification")]
pub mod notification;
#[cfg(feature = "progress")]
pub mod progress;
#[cfg(feature = "segmented_control")]
pub mod segmented_control;
#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "skeleton")]
pub mod skeleton;
#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "textarea")]
pub mod textarea;
#[cfg(feature = "theme")]
pub mod theme;
#[cfg(feature = "tooltip")]
pub mod tooltip;
#[cfg(feature = "tree")]
pub mod tree;
#[cfg(feature = "typography")]
pub mod typography;
#[cfg(feature = "number_input")]
pub mod number_input;
#[cfg(feature = "radio")]
pub mod radio;
#[cfg(feature = "slider")]
pub mod slider;
#[cfg(feature = "rating")]
pub mod rating;
#[cfg(feature = "pin_input")]
pub mod pin_input;
#[cfg(feature = "color_picker")]
pub mod color_picker;
#[cfg(feature = "chip")]
pub mod chip;
#[cfg(feature = "spoiler")]
pub mod spoiler;
#[cfg(feature = "timeline")]
pub mod timeline;
#[cfg(feature = "table")]
pub mod table;
#[cfg(feature = "scroll_area")]
pub mod scroll_area;
#[cfg(feature = "breadcrumbs")]
pub mod breadcrumbs;
#[cfg(feature = "pagination")]
pub mod pagination;

pub mod prelude {
    pub use crate::a11y::ControlSize;
    #[cfg(feature = "accordion")]
    pub use crate::accordion::*;
    #[cfg(feature = "alert")]
    pub use crate::alert::*;
    #[cfg(feature = "animation")]
    pub use crate::animation::*;
    #[cfg(feature = "attachment_card")]
    pub use crate::attachment_card::*;
    #[cfg(feature = "avatar")]
    pub use crate::avatar::*;
    #[cfg(feature = "badge")]
    pub use crate::badge::*;
    #[cfg(feature = "button")]
    pub use crate::button::*;
    #[cfg(feature = "card")]
    pub use crate::card::*;
    #[cfg(feature = "chat")]
    pub use crate::chat::*;
    #[cfg(feature = "checkbox")]
    pub use crate::checkbox::*;
    #[cfg(feature = "contact_card")]
    pub use crate::contact_card::*;
    #[cfg(feature = "divider")]
    pub use crate::divider::*;
    #[cfg(feature = "drawer")]
    pub use crate::drawer::*;
    #[cfg(feature = "dropdown")]
    pub use crate::dropdown::*;
    #[cfg(feature = "icon")]
    pub use crate::icon::*;
    #[cfg(feature = "input")]
    pub use crate::input::*;
    #[cfg(feature = "list")]
    pub use crate::list::*;
    pub use crate::locale::*;
    #[cfg(feature = "modal")]
    pub use crate::modal::*;
    #[cfg(feature = "navbar")]
    pub use crate::navbar::*;
    #[cfg(feature = "notification")]
    pub use crate::notification::*;
    #[cfg(feature = "progress")]
    pub use crate::progress::*;
    #[cfg(feature = "segmented_control")]
    pub use crate::segmented_control::*;
    #[cfg(feature = "select")]
    pub use crate::select::*;
    #[cfg(feature = "skeleton")]
    pub use crate::skeleton::*;
    #[cfg(feature = "switch")]
    pub use crate::switch::*;
    #[cfg(feature = "tabs")]
    pub use crate::tabs::*;
    #[cfg(feature = "textarea")]
    pub use crate::textarea::*;
    #[cfg(feature = "theme")]
    pub use crate::theme::*;
    #[cfg(feature = "tooltip")]
    pub use crate::tooltip::*;
    #[cfg(feature = "tree")]
    pub use crate::tree::*;
    #[cfg(feature = "typography")]
    pub use crate::typography::*;
    #[cfg(feature = "number_input")]
    pub use crate::number_input::*;
    #[cfg(feature = "radio")]
    pub use crate::radio::*;
    #[cfg(feature = "slider")]
    pub use crate::slider::*;
    #[cfg(feature = "rating")]
    pub use crate::rating::*;
    #[cfg(feature = "pin_input")]
    pub use crate::pin_input::*;
    #[cfg(feature = "color_picker")]
    pub use crate::color_picker::*;
    #[cfg(feature = "chip")]
    pub use crate::chip::*;
    #[cfg(feature = "spoiler")]
    pub use crate::spoiler::*;
    #[cfg(feature = "timeline")]
    pub use crate::timeline::*;
    #[cfg(feature = "table")]
    pub use crate::table::*;
    #[cfg(feature = "scroll_area")]
    pub use crate::scroll_area::*;
    #[cfg(feature = "breadcrumbs")]
    pub use crate::breadcrumbs::*;
    #[cfg(feature = "pagination")]
    pub use crate::pagination::*;
    pub use crate::STYLES;
}
