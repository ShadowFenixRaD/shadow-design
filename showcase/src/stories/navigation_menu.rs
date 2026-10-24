//! Story: NavigationMenu (feature `navigation_menu`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/navigation_menu.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "navigation-menu",
        "NavigationMenu",
        Category::Navigazione,
        &["navigation_menu"],
        IconKind::Menu,
        "Mega-menu di navigazione con pannelli a più colonne.",
    )
}
