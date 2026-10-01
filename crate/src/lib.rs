//! # Shadow Design
//!
//! A modular, high-performance UI component library for Dioxus 0.7 applications.
//! Built with Mantine-inspired themes, tactile craft surfaces, and zero external bloat.
//!
//! Each component is gated behind its own Cargo feature:
//! - `button`
//! - `card`
//! - `input`
//! - `badge`
//! - `navbar`
//! - `modal`
//! - `alert`
//! - `theme`
//! - `icon`
//! - `typography`
//! - `notification`
//! - `tabs`
//! - `switch`
//! - `tooltip`
//! - `drawer`
//! - `dropdown`
//! - `checkbox`
//! - `animation`
//!
//! Or enable the `all` feature to include the entire suite.

use dioxus::prelude::*;

/// Master SCSS stylesheet asset provided by the Shadow Design library.
/// Can be included directly in your Dioxus application:
/// `document::Link { rel: "stylesheet", href: shadow_design::STYLES }`
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
