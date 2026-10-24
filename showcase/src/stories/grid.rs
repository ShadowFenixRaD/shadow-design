//! Story: Grid (feature `grid`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/grid.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "grid",
        "Grid",
        Category::Layout,
        &["grid"],
        IconKind::Layers,
        "Griglia a 12 colonne e SimpleGrid a colonne uguali con spaziature a token.",
    )
}
