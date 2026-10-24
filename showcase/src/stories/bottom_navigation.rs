//! Story: BottomNavigation (feature `bottom_navigation`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/bottom_navigation.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "bottom-navigation",
        "BottomNavigation",
        Category::Navigazione,
        &["bottom_navigation"],
        IconKind::Menu,
        "Barra di navigazione inferiore per mobile.",
    )
}
