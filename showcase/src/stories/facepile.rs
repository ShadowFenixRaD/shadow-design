//! Story: Facepile (feature `facepile`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/facepile.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "facepile",
        "Facepile",
        Category::Contenuti,
        &["facepile"],
        IconKind::Sparkles,
        "Gruppo compatto di avatar con contatore +N.",
    )
}
