//! Story: Divider (feature `divider`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/dividers.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "dividers",
        "Divider",
        Category::Layout,
        &["divider"],
        IconKind::Code,
        "Separatore orizzontale o verticale, con etichetta opzionale.",
    )
}
