//! Story: Highlight (feature `highlight`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/highlight.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "highlight",
        "Highlight",
        Category::Utility,
        &["highlight"],
        IconKind::Sparkles,
        "Evidenzia le sottostringhe cercate dentro un testo.",
    )
}
