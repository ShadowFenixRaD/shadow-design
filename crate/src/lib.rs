//! # Shadow Design
//!
//! A modular UI component library for Dioxus 0.7 applications.
//!
//! Each component is gated behind its own Cargo feature:
//! - `button`
//! - `card`
//! - `input`
//! - `badge`
//! - `navbar`
//! - `modal`
//! - `alert`
//!
//! Or enable the `all` feature to include every component.

#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "card")]
pub mod card;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "badge")]
pub mod badge;
#[cfg(feature = "navbar")]
pub mod navbar;
#[cfg(feature = "modal")]
pub mod modal;
#[cfg(feature = "alert")]
pub mod alert;

pub mod prelude {
    #[cfg(feature = "button")]
    pub use crate::button::*;
    #[cfg(feature = "card")]
    pub use crate::card::*;
    #[cfg(feature = "input")]
    pub use crate::input::*;
    #[cfg(feature = "badge")]
    pub use crate::badge::*;
    #[cfg(feature = "navbar")]
    pub use crate::navbar::*;
    #[cfg(feature = "modal")]
    pub use crate::modal::*;
    #[cfg(feature = "alert")]
    pub use crate::alert::*;
}
