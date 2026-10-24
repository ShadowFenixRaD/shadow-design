//! Story: Center (feature `center`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/center.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "center",
        "Center",
        Category::Layout,
        &["center"],
        IconKind::Box,
        "Centra il contenuto in orizzontale e in verticale.",
    )
}
